// Minimal implementation to satisfy compilation
pub fn escape_str_literal(s: &str) -> String {
    s.to_string()
}

pub fn escape_byte_literal(b: u8) -> String {
    format!("{}", b)
}
