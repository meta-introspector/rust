// Layer 1: Language of only valid Rust constants
// Grammar: name = value;

#[derive(Debug, Clone)]
enum Value {
    Integer(i64),
    String(String),
    Boolean(bool),
}

#[derive(Debug)]
struct Constant {
    name: String,
    value: Value,
}

fn parse_value(input: &str) -> Option<Value> {
    if let Ok(i) = input.parse::<i64>() {
        Some(Value::Integer(i))
    } else if input == "true" || input == "false" {
        Some(Value::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(Value::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}

fn parse_constant(line: &str) -> Option<Constant> {
    let line = line.trim();
    if !line.ends_with(';') { return None; }
    
    let line = &line[..line.len()-1]; // remove ;
    let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
    if parts.len() != 2 { return None; }
    
    let name = parts[0].to_string();
    let value = parse_value(parts[1])?;
    
    Some(Constant { name, value })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_integer() {
        let c = parse_constant("BITS = 42;").unwrap();
        assert_eq!(c.name, "BITS");
        assert!(matches!(c.value, Value::Integer(42)));
    }

    #[test]
    fn test_parse_boolean() {
        let c = parse_constant("DEBUG = true;").unwrap();
        assert_eq!(c.name, "DEBUG");
        assert!(matches!(c.value, Value::Boolean(true)));
    }

    #[test]
    fn test_parse_string() {
        let c = parse_constant("NAME = \"hello\";").unwrap();
        assert_eq!(c.name, "NAME");
        assert!(matches!(c.value, Value::String(ref s) if s == "hello"));
    }

    #[test]
    fn test_invalid_syntax() {
        assert!(parse_constant("INVALID").is_none());
        assert!(parse_constant("NO_SEMICOLON = 1").is_none());
        assert!(parse_constant("= 1;").is_none());
    }
}

const EXAMPLES: &str = r#"
BITS = 64;
ENABLED = true;
MESSAGE = "hello";
COUNT = 42;
DEBUG = false;
VERSION = "1.0";
"#;
