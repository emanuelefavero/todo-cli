// * Returns a formatted index string with left padding if needed
pub fn pad_index(index: usize, length: usize) -> String {
    let need_padding = length >= 10 && index < 10;

    if need_padding {
        format!(" {}", index) // Index with left padding
    } else {
        index.to_string() // No padding needed
    }
}
