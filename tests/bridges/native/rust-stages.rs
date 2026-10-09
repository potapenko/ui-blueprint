// Appended only inside actual worker_observation in an immutable temp source copy.
#[cfg(test)]
mod q02_native_stages {
    use super::*;
    #[test]
    #[ignore = "Q02 saved Native response replay; no platform calls"]
    fn saved_receive() {
        use std::{fs, path::PathBuf, time::Instant};
        use uiblueprint_engine::cache::LedgerLimits;
        guard::configure(1, 63 * 1048576, 64 * 1048576, 1048576, 2);
        let root = PathBuf::from(std::env::var("UIB_Q02_NATIVE_INPUTS").unwrap());
        let repeats: usize = std::env::var("UIB_Q02_NATIVE_REPEATS")
            .unwrap()
            .parse()
            .unwrap();
        assert!(matches!(repeats, 1 | 100));
        let descriptor = fs::read(root.join("descriptor.json")).unwrap();
        let request = fs::read(root.join("request.json")).unwrap();
        let ax = fs::read(root.join("ax.json")).unwrap();
        let capture = fs::read(root.join("capture.json")).unwrap();
        let doc = Document::from_json(&descriptor, 524288).unwrap();
        let Artifact::Session(ref desc) = doc.artifact else {
            panic!("descriptor")
        };
        let target = TargetLease::authorized(&desc.target, false).unwrap();
        let mib = 1048576;
        let limits = HostLimits {
            workers: 2,
            worker_bytes: 64 * mib,
            publication_reserve: mib,
            bootstrap_bytes: mib,
            parent_bytes: 32 * mib,
            input_bytes: 2 * mib,
            ingress_bytes: 524288,
            output_bytes: 524288,
            request_output_bytes: 2 * mib,
            completion_groups: 2,
            control_bytes: 4096,
            cleanup_ms: 1000,
            retained_domain_bytes: 64 * mib,
            retained_per_worker: 15 * mib,
            main_stack_bytes: 8 * mib,
            watchdog_stack_bytes: mib,
        };
        let mut samples = Vec::with_capacity(repeats);
        for index in 0..repeats {
            // A new session makes the saved dispatch_sequence=1 exact. Setup is untimed.
            let ledger = QuotaLedger::new(LedgerLimits {
                retained_bytes: limits.retained_per_worker,
                session_slots: 1,
                grants: 1,
            })
            .unwrap();
            let mut session = CanonicalSession::attach(
                &descriptor,
                target,
                Id("q02-native-saved".into()),
                0,
                &ledger,
                limits,
            )
            .unwrap();
            let start = Instant::now();
            let (_, mut run) = session
                .begin_observation(std::hint::black_box(&request), || 0, 3)
                .unwrap();
            let request_admission_ns = start.elapsed().as_nanos();
            let start = Instant::now();
            assert!(
                run.receive_channel(std::hint::black_box(&ax), Channel::ExternalSemantics)
                    .unwrap()
            );
            let ax_receive_ns = start.elapsed().as_nanos();
            let start = Instant::now();
            assert!(
                run.receive_channel(std::hint::black_box(&capture), Channel::RenderedCapture)
                    .unwrap()
            );
            let capture_receive_ns = start.elapsed().as_nanos();
            let start = Instant::now();
            run.finish().unwrap();
            let complete_ns = start.elapsed().as_nanos();
            // Bytes passed to the production publisher are the original saved frames.
            assert_eq!(ax, fs::read(root.join("ax.json")).unwrap());
            assert_eq!(capture, fs::read(root.join("capture.json")).unwrap());
            samples.push(serde_json::json!({"index":index,"request_admission_ns":request_admission_ns,
                "ax_receive_ns":ax_receive_ns,"capture_receive_ns":capture_receive_ns,"complete_ns":complete_ns}));
        }
        println!(
            "@Q02_NATIVE_STAGES {}",
            serde_json::json!({"samples":samples,
            "allocator":"actual worker GuardedAllocator; fresh canonical sessions in reused test process",
            "format":"Native passes original Swift bytes; no Rust response serializer dispatched",
            "quality":"actual begin/receive/complete, partial coverage preserved, exact saved frames"})
        );
    }
}
