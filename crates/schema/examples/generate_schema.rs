fn main() -> Result<(), uiblueprint_schema::validation::ValidationError> {
    let schema = uiblueprint_schema::validation::json_schema()?;
    match serde_json::to_string_pretty(&schema) {
        Ok(json) => {
            println!("{json}");
            Ok(())
        }
        Err(_) => Err(uiblueprint_schema::validation::ValidationError::InternalSchema),
    }
}
