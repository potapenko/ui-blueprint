use std::{
    env,
    fs::File,
    io::{self, Read},
    process::ExitCode,
};
use uiblueprint_schema::{analysis::validate_input_document, validation::ValidationError};

fn report(code: &'static str, exit: u8) -> ExitCode {
    println!("{{\"valid\":{},\"code\":\"{}\"}}", exit == 0, code);
    ExitCode::from(exit)
}
fn main() -> ExitCode {
    let args: Vec<_> = env::args_os().skip(1).collect();
    if args.len() != 3 || args[0] != "--max-bytes" {
        return report("invalid_arguments", 2);
    }
    let Some(limit) = args[1]
        .to_str()
        .and_then(|s| s.parse::<u64>().ok())
        .filter(|n| *n > 0 && *n < usize::MAX as u64)
    else {
        return report("invalid_limit", 2);
    };
    let reader: Box<dyn Read> = if args[2] == "-" {
        Box::new(io::stdin())
    } else {
        match File::open(&args[2]) {
            Ok(file) => Box::new(file),
            Err(_) => return report("io_error", 1),
        }
    };
    let mut data = Vec::new();
    if reader.take(limit + 1).read_to_end(&mut data).is_err() {
        return report("io_error", 1);
    }
    match validate_input_document(&data, limit as usize) {
        Ok(_) => report("valid", 0),
        Err(ValidationError::InternalSchema) => report("internal_schema", 1),
        Err(error) => {
            // Only a closed error enum is serialized, never the document/path.
            match serde_json::to_string(&error) {
                Ok(code) => {
                    println!("{{\"valid\":false,\"code\":{code}}}");
                    ExitCode::from(2)
                }
                Err(_) => report("internal_error", 1),
            }
        }
    }
}
