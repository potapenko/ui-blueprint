use super::*;
use uiblueprint_schema::model::{Id, Identity};

struct FakeIo {
    sent: usize,
    resumed: usize,
    reads: usize,
    fail: bool,
}
impl ExchangeIo for FakeIo {
    fn send_text(&mut self, _: &str) -> Result<SendStatus, transport::Failure> {
        self.sent += 1;
        assert_eq!(self.sent, 1, "must not enqueue twice");
        Ok(SendStatus::Pending)
    }
    fn resume_send(&mut self) -> Result<SendStatus, transport::Failure> {
        self.resumed += 1;
        if self.fail {
            return Err(transport::Failure {
                kind: transport::ErrorKind::Timeout,
                send_progress: SendProgress::PossiblyWritten,
            });
        }
        Ok(if self.resumed < 3 {
            SendStatus::Pending
        } else {
            SendStatus::Flushed
        })
    }
    fn receive(&mut self) -> Result<TransportEvent, transport::Failure> {
        self.reads += 1;
        Ok(if self.reads == 1 {
            TransportEvent::WouldBlock
        } else {
            TransportEvent::Text(r#"{"id":1,"result":{}}"#.into())
        })
    }
}
fn state() -> (State, Ticket) {
    let binding = Arc::new(Binding {
        target: Identity {
            id: Id("target".into()),
            generation: Id("g".into()),
        },
        cdp_session_id: None,
    });
    let ticket = Ticket {
        epoch: 1,
        id: 1,
        binding: binding.clone(),
    };
    (
        State {
            binding,
            epoch: 1,
            active: Some(1),
            events: vec![None],
            head: 0,
            queued: 0,
            loss_generation: 0,
            unreported_loss: 0,
            event_pool: quota::Pool::new(1, 1088),
        },
        ticket,
    )
}
#[test]
fn pending_flush_resumes_exactly_once_queued_command() {
    let (mut state, ticket) = state();
    let mut io = FakeIo {
        sent: 0,
        resumed: 0,
        reads: 0,
        fail: false,
    };
    let (_, kind) = exchange(
        &mut io,
        &mut state,
        super::super::tests::limits(),
        &ticket,
        "command",
    )
    .expect("resumed command result");
    assert_eq!(kind, ReplyKind::Result);
    assert_eq!((io.sent, io.resumed, io.reads), (1, 3, 2));
}
#[test]
fn uncertain_pending_failure_does_not_retry_or_read() {
    let (mut state, ticket) = state();
    let mut io = FakeIo {
        sent: 0,
        resumed: 0,
        reads: 0,
        fail: true,
    };
    let error = exchange(
        &mut io,
        &mut state,
        super::super::tests::limits(),
        &ticket,
        "command",
    )
    .expect_err("uncertain flush");
    assert_eq!(error.send_progress, SendProgress::PossiblyWritten);
    assert_eq!((io.sent, io.resumed, io.reads), (1, 1, 0));
}
#[test]
fn inactive_old_epoch_oversize_and_wrong_session_do_not_complete() {
    let (mut state, mut ticket) = state();
    let caps = super::super::tests::limits();
    state.active = None;
    assert_eq!(
        handle_wire(&mut state, caps, &ticket, r#"{"id":1,"result":{}}"#.into())
            .expect_err("cancelled ticket")
            .kind,
        ErrorKind::UnexpectedReply
    );
    state.active = Some(1);
    ticket.epoch = 2;
    assert_eq!(
        handle_wire(&mut state, caps, &ticket, r#"{"id":1,"result":{}}"#.into())
            .expect_err("old epoch")
            .kind,
        ErrorKind::UnexpectedReply
    );
    ticket.epoch = 1;
    assert_eq!(
        handle_wire(
            &mut state,
            caps,
            &ticket,
            r#"{"id":1,"result":{},"sessionId":"foreign"}"#.into()
        )
        .expect_err("unsolicited session")
        .kind,
        ErrorKind::WrongSession
    );
    let mut oversized = String::with_capacity(2048);
    oversized.push_str(r#"{"id":1,"result":{}}"#);
    assert_eq!(
        handle_wire(&mut state, caps, &ticket, oversized)
            .expect_err("capacity is counted")
            .kind,
        ErrorKind::Budget
    );
}
#[test]
fn queued_event_byte_pressure_and_loss_overflow_are_explicit() {
    let (mut state, ticket) = state();
    let caps = super::super::tests::limits();
    let wire = r#"{"method":"Runtime.changed","params":{}}"#;
    handle_wire(&mut state, caps, &ticket, wire.into()).expect("first event");
    handle_wire(&mut state, caps, &ticket, wire.into()).expect("queue loss");
    assert_eq!(
        (state.queued, state.loss_generation, state.unreported_loss),
        (1, 1, 1)
    );
    state.loss_generation = u64::MAX;
    assert_eq!(
        handle_wire(&mut state, caps, &ticket, wire.into())
            .expect_err("loss counter exhaustion")
            .kind,
        ErrorKind::CounterExhausted
    );
    let (mut state, ticket) = self::state();
    state.event_pool = quota::Pool::new(1, 1);
    handle_wire(&mut state, caps, &ticket, wire.into()).expect("byte loss");
    assert_eq!((state.queued, state.loss_generation), (0, 1));
    assert_eq!(state.event_pool.usage(), Usage::default());
}
