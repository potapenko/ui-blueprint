// Appended inside the real worker_main module in the task-temp copy only.
#[cfg(test)]
mod q02_stage_format {
    use super::*;
    #[test]
    #[ignore = "Q02 pinned saved-input worker encoding diagnostic only"]
    fn saved_fixed_output() {
        guard::configure(1, 63 * 1048576, 64 * 1048576, 1048576, 2);
        let base = std::path::PathBuf::from(std::env::var("UIB_Q02_STAGE_INPUTS").unwrap());
        let kind = std::env::var("UIB_Q02_STAGE_KIND").unwrap();
        let repeats: usize = std::env::var("UIB_Q02_STAGE_REPEATS")
            .unwrap()
            .parse()
            .unwrap();
        assert!(matches!(repeats, 1 | 100));
        let bytes = std::fs::read(base.join(format!("{kind}-canonical.json"))).unwrap();
        let document = uiblueprint_schema::model::Document::from_json(&bytes, 524288).unwrap();
        let cap = if kind == "documents" { 524288 } else { 65536 };
        let mut buffer = vec![0; cap];
        let mut rows = Vec::with_capacity(repeats);
        for index in 0..repeats {
            let mut output = FixedOutput {
                bytes: &mut buffer,
                used: 0,
            };
            let started = Instant::now();
            let result = {
                let _reserve = guard::PublicationGuard::enter(guard::Phase::Validate);
                serde_json::to_writer(&mut output, std::hint::black_box(&document))
            };
            let format_fixed_ns = started.elapsed().as_nanos();
            result.expect("actual worker encoding");
            assert!(
                output.bytes[..output.used] == bytes,
                "encoded canonical bytes changed"
            );
            rows.push(serde_json::json!({"index":index,"format_fixed_ns":format_fixed_ns,"bytes":output.used}));
        }
        println!(
            "@Q02_STAGES {}",
            serde_json::json!({"kind":kind,"samples":rows,"allocator":"actual GuardedAllocator and PublicationGuard; standalone test process","quality":"byte-identical canonical Document"})
        );
    }
}
