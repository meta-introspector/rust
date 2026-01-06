use jsonschema::JSONSchema;
use serde_json::Value;
use std::fs;

fn validate_file(schema: &Value, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let data: Value = serde_json::from_str(&content)?;
    
    match JSONSchema::compile(schema) {
        Ok(compiled) => {
            let result = compiled.validate(&data);
            match result {
                Ok(_) => println!("✅ {} - Valid", path),
                Err(errors) => {
                    println!("❌ {} - Invalid:", path);
                    for error in errors {
                        println!("  - {}", error);
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ Schema compilation error: {}", e);
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema_content = fs::read_to_string("./usage_data_schema.json")?;
    let schema: Value = serde_json::from_str(&schema_content)?;
    
    println!("🔍 Validating usage data files against schema...");
    
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file1.json> [file2.json] ...", args[0]);
        return Ok(());
    }
    
    for file_path in &args[1..] {
        validate_file(&schema, file_path)?;
    }
    
    println!("📊 Usage data schema validation complete");
    Ok(())
}
