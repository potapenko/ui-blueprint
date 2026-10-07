use super::{DarwinPlatform, HostError, errno, os_error};
use crate::{CONTROL_BYTES, process_api::WorkerPlatform};
use std::mem::MaybeUninit;

fn limits(resource: i32) -> Result<libc::rlimit, HostError> {
    let mut value = MaybeUninit::uninit();
    // SAFETY: getrlimit initializes value on success; resource is a public constant.
    if unsafe { libc::getrlimit(resource, value.as_mut_ptr()) } != 0 {
        return Err(os_error(errno()));
    }
    Ok(unsafe { value.assume_init() })
}
fn set(resource: i32, value: &libc::rlimit) -> Result<(), HostError> {
    // SAFETY: valid immutable limit record; only the calling owned child is changed.
    if unsafe { libc::setrlimit(resource, value) } != 0 {
        Err(os_error(errno()))
    } else {
        Ok(())
    }
}
impl WorkerPlatform for DarwinPlatform {
    fn setup_main(main_stack_bytes: usize) -> Result<usize, HostError> {
        // SAFETY: public allocation-free query for the current thread.
        if unsafe { libc::pthread_main_np() } != 1 {
            return Err(HostError::InvalidState);
        }
        if main_stack_bytes == 0 || main_stack_bytes > 8 * 1024 * 1024 {
            return Err(HostError::InvalidLimits);
        }
        set(
            libc::RLIMIT_CORE,
            &libc::rlimit {
                rlim_cur: 0,
                rlim_max: 0,
            },
        )?;
        let core = limits(libc::RLIMIT_CORE)?;
        if core.rlim_cur != 0 || core.rlim_max != 0 {
            return Err(HostError::InvalidState);
        }
        let requested = main_stack_bytes as libc::rlim_t;
        let existing = limits(libc::RLIMIT_STACK)?;
        if requested > existing.rlim_max {
            return Err(HostError::InvalidLimits);
        }
        set(
            libc::RLIMIT_STACK,
            &libc::rlimit {
                rlim_cur: requested,
                rlim_max: requested,
            },
        )?;
        let applied = limits(libc::RLIMIT_STACK)?;
        let actual = Self::current_stack_bytes()?;
        if applied.rlim_cur != requested
            || applied.rlim_max != requested
            || actual > main_stack_bytes
        {
            return Err(HostError::ResourceLimit);
        }
        Ok(actual)
    }
    fn watchdog_stack_request(ceiling: usize) -> Result<usize, HostError> {
        // This is a creation request for the qualified Darwin profile, not proof
        // of the resulting extent. Caller MUST query it inside the actual thread.
        if ceiling == 0 || ceiling > 1024 * 1024 {
            return Err(HostError::InvalidLimits);
        }
        // SAFETY: public constant query, no pointers or caller state mutated.
        let page = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
        let page = usize::try_from(page)
            .ok()
            .filter(|n| *n > 0)
            .ok_or(HostError::Io)?;
        let usable = ceiling.checked_sub(page).ok_or(HostError::InvalidLimits)?;
        // Round down, never above the original ceiling, leaving one host page
        // for observed OS padding. Multiplication cannot exceed usable.
        let requested = (usable / page) * page;
        if requested < libc::PTHREAD_STACK_MIN || requested == 0 {
            return Err(HostError::InvalidLimits);
        }
        Ok(requested)
    }
    fn current_stack_bytes() -> Result<usize, HostError> {
        // SAFETY: pthread_self identifies the live calling thread; the public
        // Darwin query reads its stack extent. No cached/requested value is used.
        let size = unsafe { libc::pthread_get_stacksize_np(libc::pthread_self()) };
        if size == 0 {
            Err(HostError::InvalidState)
        } else {
            Ok(size)
        }
    }
    fn fatal_exit(fd: i32, record: &[u8; CONTROL_BYTES], exit_code: i32) -> ! {
        // SAFETY: record is live for exactly 64 readable bytes. send is one
        // nonblocking best-effort write, even if caller status setup failed; it
        // cannot deliver SIGPIPE or unwind/allocate in Rust. No retry or formatting.
        unsafe {
            if fd >= 0 {
                libc::send(
                    fd,
                    record.as_ptr().cast(),
                    CONTROL_BYTES,
                    libc::MSG_DONTWAIT | libc::MSG_NOSIGNAL,
                );
            }
            libc::_exit(exit_code)
        }
    }
}
