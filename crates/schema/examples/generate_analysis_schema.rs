fn main() -> Result<(), uiblueprint_schema::validation::ValidationError> {
    let schema = uiblueprint_schema::analysis::json_schema()?;
    let json = serde_json::to_string_pretty(&schema)
        .map_err(|_| uiblueprint_schema::validation::ValidationError::InternalSchema)?;
    println!("{json}");
    Ok(())
}
