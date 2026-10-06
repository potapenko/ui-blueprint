//! Explicit local file adapter for E01 testing/handoff, not the product CLI.
use std::{io::Read, path::Path};
use uiblueprint_export::{DrawingBrief, ExportLimits, compile};
fn main() {
    if run().is_err() {
        eprintln!("export failed; invalid input, budget, or output destination");
        std::process::exit(2);
    }
}
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() != 7 {
        return Err("usage: compile INPUT OUTPUT MAX_INPUT_BYTES MAX_OUTPUT_BYTES MAX_COMPONENTS MAX_VIEWS COMPONENTS_PER_DETAIL".into());
    }
    let limits = ExportLimits {
        max_input_bytes: args[2].parse()?,
        max_output_bytes: args[3].parse()?,
        max_components: args[4].parse()?,
        max_views: args[5].parse()?,
        components_per_detail: args[6].parse()?,
    };
    let cap = u64::try_from(limits.max_input_bytes)?
        .checked_add(1)
        .ok_or("limit overflow")?;
    let mut bytes = vec![];
    std::fs::File::open(&args[0])?
        .take(cap)
        .read_to_end(&mut bytes)?;
    let brief = DrawingBrief::from_json(&bytes, limits)?;
    compile(&brief, limits)?.write_new(Path::new(&args[1]))?;
    Ok(())
}
