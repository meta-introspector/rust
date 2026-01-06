use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=usage_patterns/");
    
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_usage_code.rs");
    
    // Generate code from usage patterns
    let generated_code = generate_usage_code();
    
    fs::write(&dest_path, generated_code).unwrap();
    
    println!("cargo:rustc-env=GENERATED_CODE_PATH={}", dest_path.display());
}

fn generate_usage_code() -> String {
    let mut code = String::new();
    
    code.push_str("// Auto-generated usage-driven code\n");
    code.push_str("use std::collections::HashMap;\n\n");
    
    // Generate usage pattern executors
    code.push_str("pub struct UsageExecutor {\n");
    code.push_str("    patterns: HashMap<String, Box<dyn Fn() -> String>>,\n");
    code.push_str("}\n\n");
    
    code.push_str("impl UsageExecutor {\n");
    code.push_str("    pub fn new() -> Self {\n");
    code.push_str("        let mut patterns = HashMap::new();\n");
    
    // Add common syn usage patterns
    add_syn_patterns(&mut code);
    
    code.push_str("        Self { patterns }\n");
    code.push_str("    }\n\n");
    
    code.push_str("    pub fn execute(&self, pattern: &str) -> Option<String> {\n");
    code.push_str("        self.patterns.get(pattern).map(|f| f())\n");
    code.push_str("    }\n");
    code.push_str("}\n");
    
    code
}

fn add_syn_patterns(code: &mut String) {
    // Parse function pattern
    code.push_str("        patterns.insert(\"parse_file\".to_string(), Box::new(|| {\n");
    code.push_str("            \"syn::parse_file(&content).expect(\\\"Failed to parse\\\")\".to_string()\n");
    code.push_str("        }));\n");
    
    // Visit pattern
    code.push_str("        patterns.insert(\"visit_fn\".to_string(), Box::new(|| {\n");
    code.push_str("            \"visitor.visit_item_fn(&func)\".to_string()\n");
    code.push_str("        }));\n");
    
    // Token stream pattern
    code.push_str("        patterns.insert(\"to_tokens\".to_string(), Box::new(|| {\n");
    code.push_str("            \"item.to_token_stream().to_string()\".to_string()\n");
    code.push_str("        }));\n");
}
