// Returns a formatted index string with left padding if needed
pub fn format_index(index: usize, need_padding: bool) -> String {
    if need_padding {
        format!(" {}", index) // Index with left padding
    } else {
        index.to_string() // No padding needed
    }
}
