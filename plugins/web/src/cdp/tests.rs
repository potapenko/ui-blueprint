use super::*;

pub(super) fn limits() -> Limits {
    Limits {
        max_request_bytes: 512,
        max_message_bytes: 1024,
        max_metadata_bytes: 64,
        max_results: 2,
        result_bytes: 2048,
        max_events: 1,
        event_bytes: 1088,
    }
}
#[test]
fn exact_id_and_epoch_counters_refuse_wrap() {
    let ids = AtomicU32::new(MAX_WIRE_ID - 1);
    assert_eq!(next_id(&ids).expect("last ID"), MAX_WIRE_ID);
    assert_eq!(
        next_id(&ids).expect_err("exhausted").kind,
        ErrorKind::CounterExhausted
    );
    assert_eq!(ids.load(Ordering::Relaxed), MAX_WIRE_ID);
    let epochs = AtomicU64::new(u64::MAX - 1);
    assert_eq!(increment_epoch(&epochs).expect("last epoch"), u64::MAX);
    assert_eq!(
        increment_epoch(&epochs).expect_err("exhausted").kind,
        ErrorKind::CounterExhausted
    );
}
#[test]
fn limits_and_quota_arithmetic_reject_zero_overflow_and_live_lease_overcommit() {
    for caps in [
        Limits {
            max_results: 0,
            ..limits()
        },
        Limits {
            max_events: usize::MAX,
            ..limits()
        },
        Limits {
            max_events: isize::MAX as usize,
            ..limits()
        },
        Limits {
            result_bytes: 1,
            ..limits()
        },
    ] {
        assert_eq!(
            caps.validate().expect_err("bad limits").kind,
            ErrorKind::InvalidLimits
        );
    }
    let pool = quota::Pool::new(2, usize::MAX);
    let lease = pool.reserve(usize::MAX).expect("logical reservation");
    assert_eq!(
        pool.reserve(1).err().expect("checked overflow").kind,
        ErrorKind::Budget
    );
    assert_eq!(pool.usage().slots, 1);
    drop(lease);
    assert_eq!(pool.usage(), Usage::default());
    let pool = quota::Pool::new(1, 20);
    let lease = pool.reserve(10).expect("slot");
    assert!(pool.reserve(1).is_err());
    drop(lease);
    assert!(pool.reserve(20).is_ok());
}
#[test]
fn shallow_codec_rejects_ambiguous_missing_mistyped_and_lossy_ids() {
    for wire in [
        r#"{"id":1,"id":1,"result":{}}"#,
        r#"{"id":1,"result":null}"#,
        r#"{"id":1,"result":[],"error":{"code":1,"message":"private"}}"#,
        r#"{"id":1,"result":{},"error":{"code":1,"message":"private"}}"#,
        r#"{"id":1,"result":{},"sessionId":null}"#,
        r#"{"id":1,"result":{},"sessionId":""}"#,
        r#"{"id":1,"result":{},"unknown":"private"}"#,
        r#"{"id":1,"result":{}} trailing"#,
        r#"{"id":1.0,"result":{}}"#,
        r#"{"id":1e0,"result":{}}"#,
        r#"{"id":0,"result":{}}"#,
        r#"{"id":-1,"result":{}}"#,
        r#"{"id":2147483648,"result":{}}"#,
        r#"{"id":"1","result":{}}"#,
        r#"{"id":1,"error":{"code":1}}"#,
        r#"{"id":1,"error":{"code":1.0,"message":"private"}}"#,
        r#"{"id":1,"error":{"code":1,"message":null}}"#,
        r#"{"id":1,"error":{"code":1,"message":"a","message":"b"}}"#,
        r#"{"method":"Runtime.changed"}"#,
        r#"{"method":false,"params":{}}"#,
        r#"{"id":1,"method":"Runtime.changed","params":{}}"#,
        r#"{"result":{}}"#,
        r#"{"id":1,"result":{},"sessionId":"a","sessionId":"a"}"#,
    ] {
        assert!(codec::inspect(wire, 64).is_err(), "must reject: {wire}");
    }
    assert!(
        codec::inspect(
            r#"{"id":2147483647,"result":{"nested":[null,true,{"x":1}]}}"#,
            64
        )
        .is_ok()
    );
    assert!(codec::inspect(r#"{"method":"Runtime.\u0063hanged","params":{}}"#, 64).is_ok());
    assert!(codec::inspect(r#"{"method":"longer","params":{}}"#, 2).is_err());
}
