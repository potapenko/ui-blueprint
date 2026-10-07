//! Deterministic two-descriptor scheduling: the real fatal lane becomes readable
//! to this driver only after it observes the worker's normal IO close. No status
//! is fabricated; this exercises the real installed guard and Darwin child.
use super::*;
use std::os::fd::BorrowedFd;
use uiblueprint_host::{
    process::DarwinChild,
    process_api::{OwnedProcess, PollInterest, ProcessPlatform, ProcessState, Transfer},
};
struct Platform {
    deliver_status: bool,
}
struct Child {
    real: DarwinChild,
    closed: bool,
    deliver_status: bool,
}
impl ProcessPlatform for Platform {
    type Child = Child;
    fn validate_parent_reaping() -> Result<(), HostError> {
        DarwinPlatform::validate_parent_reaping()
    }
    fn spawn(&mut self, spec: &SpawnSpec) -> Result<Child, HostError> {
        DarwinPlatform.spawn(spec).map(|real| Child {
            real,
            closed: false,
            deliver_status: self.deliver_status,
        })
    }
    fn poll(&mut self, i: &mut [PollInterest<'_>], ms: u32) -> Result<(), HostError> {
        DarwinPlatform.poll(i, ms)
    }
}
impl OwnedProcess for Child {
    fn write_input(&mut self, b: &[u8]) -> Result<Transfer, HostError> {
        let result = self.real.write_input(b);
        if matches!(result, Ok(Transfer::Closed) | Err(_)) {
            self.closed = true;
        }
        result
    }
    fn read_output(&mut self, b: &mut [u8]) -> Result<Transfer, HostError> {
        let result = self.real.read_output(b);
        if matches!(result, Ok(Transfer::Closed) | Err(_)) {
            self.closed = true;
        }
        result
    }
    fn read_fatal(&mut self, b: &mut [u8]) -> Result<Transfer, HostError> {
        if !self.deliver_status {
            return Ok(Transfer::Closed);
        }
        if self.closed {
            self.real.read_fatal(b)
        } else {
            Ok(Transfer::WouldBlock)
        }
    }
    fn close_input(&mut self) {
        self.real.close_input();
    }
    fn terminate(&mut self) -> Result<(), HostError> {
        self.real.terminate()
    }
    fn try_reap(&mut self) -> Result<ProcessState, HostError> {
        self.real.try_reap()
    }
    fn input_fd(&self) -> Option<BorrowedFd<'_>> {
        self.real.input_fd()
    }
    fn output_fd(&self) -> BorrowedFd<'_> {
        self.real.output_fd()
    }
    fn fatal_fd(&self) -> BorrowedFd<'_> {
        self.real.fatal_fd()
    }
}
#[test]
fn reliable_fatal_is_drained_after_io_close_before_generic_failure_attribution() {
    let _serial = RUNTIME_TEST.lock().unwrap();
    for deliver_status in [true, false] {
        let mut config = limits();
        config.worker_bytes = 2 * MIB;
        let domain = HostDomain::new::<Platform>(config).unwrap();
        let mut host = RuntimeHost::new(
            &domain,
            SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).unwrap(),
            Platform { deliver_status },
        )
        .unwrap();
        let mut input = host
            .reserve_attach_input(target(), DESCRIPTOR.len())
            .unwrap();
        input.bytes_mut().copy_from_slice(DESCRIPTOR);
        host.attach(input, deadline()).unwrap();
        let result = complete(&mut host);
        assert_eq!(
            result.terminal,
            Terminal::Failed(if deliver_status {
                HostError::ResourceLimit
            } else {
                HostError::WorkerFailed
            })
        );
        assert_eq!(result.committed(), 0);
        assert!(matches!(next(&mut host), HostEvent::Closed { .. }));
        assert_eq!(domain.usage().reserved_sessions, 0);
    }
}
