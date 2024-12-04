pub fn generate_page_filename(name: &str) -> String {
    let name = name.trim().to_ascii_lowercase().replace(' ', "-");
    let name = format!("{name}.html");
    name
}
