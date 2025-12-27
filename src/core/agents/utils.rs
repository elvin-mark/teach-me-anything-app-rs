pub fn clean_json_response(content: &str) -> String {
    let mut cleaned = content.trim().to_string();

    // Remove ```json prefix
    if cleaned.starts_with("```json") {
        cleaned = cleaned.strip_prefix("```json").unwrap().to_string();
    }

    // Remove ``` suffix
    if cleaned.ends_with("```") {
        cleaned = cleaned.strip_suffix("```").unwrap().to_string();
    }

    cleaned.trim().to_string()
}
