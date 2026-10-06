use uiblueprint_schema::{VersionDocument, VersionError};

fn main() -> Result<(), VersionError> {
    let current = include_bytes!("../tests/fixtures/current.json");
    let incompatible = include_bytes!("../tests/fixtures/incompatible.json");
    let parsed = VersionDocument::from_json(current, current.len())?;
    println!("{}", parsed.to_json()?);
    match VersionDocument::from_json(incompatible, incompatible.len()) {
        Err(VersionError::IncompatibleVersion) => {
            println!("{}", VersionError::IncompatibleVersion);
            Ok(())
        }
        Err(error) => Err(error),
        Ok(_) => Err(VersionError::IncompatibleVersion),
    }
}
