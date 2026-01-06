use std::collections::HashMap;
use crate::libusagedata::{UsageData, CrateUsageData};

pub type CodeGenerator = Box<dyn Fn(&UsageData) -> String>;

pub struct UsageCodeMap {
    generators: HashMap<String, CodeGenerator>,
}

impl UsageCodeMap {
    pub fn new() -> Self {
        let mut generators: HashMap<String, CodeGenerator> = HashMap::new();
        
        // Syn parsing patterns
        generators.insert("parse_file".to_string(), Box::new(|usage| {
            format!(
                "let ast = syn::parse_file(&content)?; // Usage: {} times", 
                usage.usage_count
            )
        }));
        
        generators.insert("visit_fn".to_string(), Box::new(|usage| {
            format!(
                "visitor.visit_item_fn(&function); // Usage: {} times", 
                usage.usage_count
            )
        }));
        
        generators.insert("to_token_stream".to_string(), Box::new(|usage| {
            format!(
                "let tokens = item.to_token_stream(); // Usage: {} times", 
                usage.usage_count
            )
        }));
        
        // AST navigation patterns
        generators.insert("Item::Fn".to_string(), Box::new(|usage| {
            format!(
                "if let syn::Item::Fn(func) = item {{ /* Process function */ }} // Usage: {} times", 
                usage.usage_count
            )
        }));
        
        generators.insert("Item::Struct".to_string(), Box::new(|usage| {
            format!(
                "if let syn::Item::Struct(s) = item {{ /* Process struct */ }} // Usage: {} times", 
                usage.usage_count
            )
        }));
        
        // Token manipulation patterns
        generators.insert("quote!".to_string(), Box::new(|usage| {
            format!(
                "let generated = quote! {{ /* template */ }}; // Usage: {} times", 
                usage.usage_count
            )
        }));
        
        Self { generators }
    }
    
    pub fn generate_code(&self, usage_data: &[UsageData]) -> String {
        let mut code = String::new();
        
        code.push_str("// Generated from usage patterns\n");
        code.push_str("use syn::{{Item, ItemFn, ItemStruct}};\n");
        code.push_str("use quote::{{quote, ToTokens}};\n\n");
        
        code.push_str("pub fn execute_usage_pattern(items: &[Item]) {\n");
        
        for usage in usage_data {
            if let Some(generator) = self.generators.get(&usage.symbol) {
                let generated_line = generator(usage);
                code.push_str(&format!("    {}\n", generated_line));
            }
        }
        
        code.push_str("}\n\n");
        
        // Generate weighted execution function
        code.push_str("pub fn weighted_execution(items: &[Item], weights: &std::collections::HashMap<String, f64>) {\n");
        code.push_str("    use rand::{{thread_rng, Rng}};\n");
        code.push_str("    let mut rng = thread_rng();\n\n");
        
        code.push_str("    for item in items {\n");
        code.push_str("        let name = match item {\n");
        code.push_str("            Item::Fn(f) => f.sig.ident.to_string(),\n");
        code.push_str("            Item::Struct(s) => s.ident.to_string(),\n");
        code.push_str("            _ => continue,\n");
        code.push_str("        };\n\n");
        
        code.push_str("        if let Some(&weight) = weights.get(&name) {\n");
        code.push_str("            if rng.gen::<f64>() < weight {\n");
        
        for usage in usage_data {
            if let Some(generator) = self.generators.get(&usage.symbol) {
                let generated_line = generator(usage);
                code.push_str(&format!("                {}\n", generated_line));
            }
        }
        
        code.push_str("            }\n");
        code.push_str("        }\n");
        code.push_str("    }\n");
        code.push_str("}\n");
        
        code
    }
    
    pub fn add_pattern<F>(&mut self, pattern: String, generator: F) 
    where F: Fn(&UsageData) -> String + 'static {
        self.generators.insert(pattern, Box::new(generator));
    }
}

pub fn compile_usage_to_executable(usage_data: &[UsageData]) -> String {
    let mapper = UsageCodeMap::new();
    mapper.generate_code(usage_data)
}
