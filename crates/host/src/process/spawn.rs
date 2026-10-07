use super::{DarwinChild, HostError, Ownership, errno, os_error};
use crate::process_api::SpawnSpec;
use std::{
    mem::MaybeUninit,
    os::fd::{AsRawFd, FromRawFd, OwnedFd},
    ptr,
};

fn check(code: i32) -> Result<(), HostError> {
    if code == 0 {
        Ok(())
    } else {
        Err(os_error(code))
    }
}
fn flag(fd: &OwnedFd, command: i32, value: i32) -> Result<(), HostError> {
    // SAFETY: owned descriptor; the selected fcntl commands take an integer value.
    if unsafe { libc::fcntl(fd.as_raw_fd(), command, value) } < 0 {
        Err(os_error(errno()))
    } else {
        Ok(())
    }
}
fn high(fd: OwnedFd) -> Result<OwnedFd, HostError> {
    if fd.as_raw_fd() >= 6 {
        flag(&fd, libc::F_SETFD, libc::FD_CLOEXEC)?;
        return Ok(fd);
    }
    // SAFETY: duplicate our live descriptor, with an independent owned lifetime.
    // Moving every source above 5 prevents all dup2/close target collisions.
    let copy = unsafe { libc::fcntl(fd.as_raw_fd(), libc::F_DUPFD_CLOEXEC, 6) };
    if copy < 0 {
        return Err(os_error(errno()));
    }
    // SAFETY: successful fcntl returned a new descriptor, not owned elsewhere.
    Ok(unsafe { OwnedFd::from_raw_fd(copy) })
}
fn pair(child_nonblocking: bool) -> Result<(OwnedFd, OwnedFd), HostError> {
    let mut fds = [-1; 2];
    // SAFETY: fds has two writable ints. On success both are new owned sockets.
    if unsafe { libc::socketpair(libc::AF_UNIX, libc::SOCK_STREAM, 0, fds.as_mut_ptr()) } != 0 {
        return Err(os_error(errno()));
    }
    // Wrap both immediately so a later setup failure closes only these sockets.
    let [parent, child] = unsafe { [OwnedFd::from_raw_fd(fds[0]), OwnedFd::from_raw_fd(fds[1])] };
    let parent = high(parent)?;
    let child = high(child)?;
    flag(&parent, libc::F_SETFL, libc::O_NONBLOCK)?;
    if child_nonblocking {
        flag(&child, libc::F_SETFL, libc::O_NONBLOCK)?;
    }
    Ok((parent, child))
}
struct Actions(libc::posix_spawn_file_actions_t);
impl Actions {
    fn new() -> Result<Self, HostError> {
        let mut raw = MaybeUninit::uninit();
        // SAFETY: initializer writes the opaque handle; assume_init only on success.
        check(unsafe { libc::posix_spawn_file_actions_init(raw.as_mut_ptr()) })?;
        Ok(Self(unsafe { raw.assume_init() }))
    }
    fn dup(&mut self, fd: &OwnedFd, target: i32) -> Result<(), HostError> {
        // SAFETY: initialized actions, owned source >=6 and fixed target 0..5.
        check(unsafe {
            libc::posix_spawn_file_actions_adddup2(&mut self.0, fd.as_raw_fd(), target)
        })
    }
    fn close(&mut self, fd: &OwnedFd) -> Result<(), HostError> {
        // SAFETY: owned source >=6; closes a child copy after all dup2 actions.
        check(unsafe { libc::posix_spawn_file_actions_addclose(&mut self.0, fd.as_raw_fd()) })
    }
}
impl Drop for Actions {
    fn drop(&mut self) {
        // SAFETY: this owner is constructed only after successful init; destroy once.
        unsafe {
            libc::posix_spawn_file_actions_destroy(&mut self.0);
        }
    }
}
struct Attributes(libc::posix_spawnattr_t);
impl Attributes {
    fn new() -> Result<Self, HostError> {
        let mut raw = MaybeUninit::uninit();
        // SAFETY: writable storage for initializer; read only after success.
        check(unsafe { libc::posix_spawnattr_init(raw.as_mut_ptr()) })?;
        let mut value = Self(unsafe { raw.assume_init() });
        // SAFETY: initialized handle; public Darwin flag closes unrelated FDs,
        // even descriptors another parent thread has not yet marked CLOEXEC.
        check(unsafe {
            libc::posix_spawnattr_setflags(&mut value.0, libc::POSIX_SPAWN_CLOEXEC_DEFAULT as i16)
        })?;
        Ok(value)
    }
}
impl Drop for Attributes {
    fn drop(&mut self) {
        // SAFETY: initialized opaque handle remains exclusively owned until drop.
        unsafe {
            libc::posix_spawnattr_destroy(&mut self.0);
        }
    }
}

pub(super) fn spawn(spec: &SpawnSpec) -> Result<DarwinChild, HostError> {
    let (input, child_input) = pair(false)?;
    let (output, child_output) = pair(false)?;
    let (fatal, child_fatal) = pair(true)?;
    // SAFETY: fixed NUL-terminated path; no user path/env used for null streams.
    let raw_null = unsafe { libc::open(c"/dev/null".as_ptr(), libc::O_RDWR | libc::O_CLOEXEC) };
    if raw_null < 0 {
        return Err(os_error(errno()));
    }
    // SAFETY: successful open created this owned descriptor.
    let null = high(unsafe { OwnedFd::from_raw_fd(raw_null) })?;
    let mut actions = Actions::new()?;
    for target in 0..3 {
        actions.dup(&null, target)?;
    }
    actions.dup(&child_input, 3)?;
    actions.dup(&child_output, 4)?;
    actions.dup(&child_fatal, 5)?;
    for fd in [
        &input,
        &output,
        &fatal,
        &child_input,
        &child_output,
        &child_fatal,
        &null,
    ] {
        actions.close(fd)?;
    }
    let attributes = Attributes::new()?;
    let argv = [spec.executable().as_ptr().cast_mut(), ptr::null_mut()];
    let env = [ptr::null_mut()];
    let mut pid = 0;
    // SAFETY: all C handles initialized; path/argv/env are terminated and live
    // through the call; spawn does not modify their strings. Empty env, no shell.
    check(unsafe {
        libc::posix_spawn(
            &mut pid,
            spec.executable().as_ptr(),
            &actions.0,
            &attributes.0,
            argv.as_ptr(),
            env.as_ptr(),
        )
    })?;
    if pid <= 0 {
        return Err(HostError::WorkerFailed);
    }
    Ok(DarwinChild {
        input: Some(input),
        output,
        fatal,
        ownership: Ownership::Child(pid),
    })
}
