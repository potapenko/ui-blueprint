use uiblueprint_host::quota::{ChargeError, QuotaCounter};
#[test]
fn exact_precharge_and_realloc_overlap_do_not_wrap() {
    let counter = QuotaCounter::new();
    counter.charge(8, 12).expect("old allocation");
    assert_eq!(counter.charge(5, 12), Err(ChargeError::Limit));
    assert_eq!(counter.live(), 8);
    counter
        .charge(4, 12)
        .expect("full new layout reserved while old lives");
    assert_eq!(counter.live(), 12);
    counter
        .release(8)
        .expect("old released on successful realloc");
    assert_eq!(counter.live(), 4);
    assert_eq!(counter.peak(), 12);
    assert_eq!(counter.release(5), Err(ChargeError::Underflow));
    assert_eq!(counter.live(), 4);
    assert_eq!(
        counter.charge(usize::MAX, usize::MAX),
        Err(ChargeError::Overflow)
    );
    counter.release(4).expect("deallocation");
    assert_eq!(counter.live(), 0);
}
#[test]
fn concurrent_charges_preserve_real_live_ownership() {
    let counter = QuotaCounter::new();
    std::thread::scope(|scope| {
        for _ in 0..4 {
            let counter = &counter;
            scope.spawn(move || {
                for _ in 0..1000 {
                    counter.charge(16, 64).expect("reserved aggregate");
                    std::hint::black_box(counter.live());
                    counter.release(16).expect("owned release");
                }
            });
        }
    });
    assert_eq!(counter.live(), 0);
    assert!(counter.peak() <= 64);
}
