//! Read-only parent-policy checks. This module never installs a signal action.
use super::{HostError, errno, os_error};
use std::mem::MaybeUninit;

/// Supported for the entire managed-child lifetime: default SIGCHLD, no kernel
/// auto-reap and no custom handler. SIG_DFL's discarded notification is NOT the
/// explicit SIG_IGN policy (Darwin sets P_NOCLDWAIT for the latter).
///
/// Core/embedding must keep this invariant stable and forbid competing waiters.
/// Point checks cannot detect an incompatible policy changed and restored by
/// arbitrary same-process native code between calls; this is not a pidfd/sandbox.
pub(super) fn supported() -> Result<(), HostError> {
    let mut action = MaybeUninit::<libc::sigaction>::uninit();
    // SAFETY: null act queries only; out storage is correctly sized/aligned and
    // is read only after success. No parent/operator signal state is changed.
    if unsafe { libc::sigaction(libc::SIGCHLD, std::ptr::null(), action.as_mut_ptr()) } != 0 {
        return Err(os_error(errno()));
    }
    // SAFETY: successful sigaction initialized the public record fields.
    let action = unsafe { action.assume_init() };
    if action.sa_sigaction != libc::SIG_DFL
        || action.sa_flags & (libc::SA_NOCLDWAIT | libc::SA_SIGINFO) != 0
    {
        return Err(HostError::InvalidState);
    }
    Ok(())
}
