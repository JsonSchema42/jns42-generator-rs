pub fn join_json_pointer(base: &str, parts: Vec<&str>) -> String {
    let suffix = parts
        .iter()
        .map(|part| urlencoding::encode(part))
        .map(|part| format!("/{}", part))
        .collect::<Vec<_>>()
        .join("");

    format!("{}{}", base, suffix)
}
