use syn::{parse_file, Item, File};
use quote::quote;
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::sync::{Mutex, LazyLock};

// Global error type counters for sampling
static ERROR_COUNTERS: LazyLock<Mutex<HashMap<String, usize>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

// Global audit log
static AUDIT_LOG: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));

fn log_audit(message: String) {
    if let Ok(mut log) = AUDIT_LOG.lock() {
        log.push(message);
    }
}

#[derive(Debug, Clone)]
pub struct TransformationStep {
    pub name: String,
    pub content: String,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug)]
pub struct ProcessingAudit {
    pub original_content: String,
    pub steps: Vec<TransformationStep>,
    pub final_success: bool,
    pub bisection_log: Vec<String>,
}

impl ProcessingAudit {
    pub fn new(content: &str) -> Self {
        Self {
            original_content: content.to_string(),
            steps: Vec::new(),
            final_success: false,
            bisection_log: Vec::new(),
        }
    }

    pub fn add_step(&mut self, name: &str, content: String, success: bool, error: Option<String>) {
        self.steps.push(TransformationStep {
            name: name.to_string(),
            content,
            success,
            error,
        });
    }

    pub fn test_parse(&self, content: &str) -> (bool, Option<String>) {
        match parse_file(content) {
            Ok(_) => (true, None),
            Err(e) => (false, Some(e.to_string())),
        }
    }

    pub fn bisect_transformations(&mut self) -> Result<String, String> {
        self.bisection_log.push("Starting bisection process".to_string());
        
        // Test original
        let (original_ok, _) = self.test_parse(&self.original_content);
        if !original_ok {
            return Err("Original content doesn't parse".to_string());
        }

        // Try all transformations together first
        let mut current = self.original_content.clone();
        let mut successful_steps = Vec::new();

        // Apply each transformation one by one and test
        for (i, step) in self.steps.iter().enumerate() {
            let test_content = apply_transformation_by_name(&current, &step.name);
            let (parse_ok, parse_error) = self.test_parse(&test_content);
            
            if parse_ok {
                self.bisection_log.push(format!("✅ Step {}: {} - SUCCESS", i+1, step.name));
                current = test_content;
                successful_steps.push(step.name.clone());
            } else {
                self.bisection_log.push(format!("❌ Step {}: {} - FAILED: {}", 
                    i+1, step.name, parse_error.unwrap_or("Unknown error".to_string())));
                // Skip this transformation
            }
        }

        self.bisection_log.push(format!("Bisection complete. Applied {} out of {} transformations", 
            successful_steps.len(), self.steps.len()));

        Ok(current)
    }
}

pub fn add_prelude(content: &str) -> String {
    match add_prelude_syn(content) {
        Ok(result) => result,
        Err(_) => {
            // Fallback to original method if syn parsing fails
            format!("use split_decls_genesis::ourprelude::*;\n{}", content)
        }
    }
}

fn add_prelude_syn(content: &str) -> Result<String, syn::Error> {
    let mut file: File = parse_file(content)?;
    
    // Create the prelude use statement
    let prelude_use: Item = syn::parse_quote! {
        use split_decls_genesis::ourprelude::*;
    };
    
    // Insert at the beginning of items (after attributes and comments)
    file.items.insert(0, prelude_use);
    
    Ok(quote!(#file).to_string())
}

pub fn fix_file_paths(content: &str) -> String {
    content.replace("\"../messages.ftl\"", "\"messages.ftl\"")
}

pub fn fix_env_vars(content: &str) -> String {
    content.replace("env ! (\"CFG_RELEASE_CHANNEL\")", "\"dev\"")
}

pub fn fix_attribute_spacing(content: &str) -> String {
    content.replace("# [", "#[")
}

pub fn remove_crate_attrs(content: &str) -> String {
    content.replace("# [allow (internal_features)] # [allow (rustc :: untranslatable_diagnostic)] # [doc (html_root_url = \"https://doc.rust-lang.org/nightly/nightly-rustc/\")] # [doc (rust_logo)] # [feature (decl_macro)] # [feature (panic_backtrace_config)] # [feature (panic_update_hook)] # [feature (rustdoc_internals)] # [feature (try_blocks)] ", "")
}

fn apply_transformation_by_name(content: &str, name: &str) -> String {
    match name {
        "add_prelude" => add_prelude(content),
        "fix_file_paths" => fix_file_paths(content),
        "fix_env_vars" => fix_env_vars(content),
        "fix_attribute_spacing" => fix_attribute_spacing(content),
        "remove_crate_attrs" => remove_crate_attrs(content),
        _ => content.to_string(),
    }
}

pub fn process_content_with_audit(content: &str) -> Result<(String, ProcessingAudit), Box<dyn std::error::Error>> {
    let mut audit = ProcessingAudit::new(content);
    
    // Define transformation steps
    let transformations = vec![
        "add_prelude",
        "fix_file_paths", 
        "fix_env_vars",
        "fix_attribute_spacing",
        "remove_crate_attrs",
    ];

    // Test original content
    let (original_ok, original_error) = audit.test_parse(content);
    if !original_ok {
        return Err(format!("Original content doesn't parse: {}", 
            original_error.unwrap_or("Unknown error".to_string())).into());
    }

    // Apply transformations and record each step
    let mut current = content.to_string();
    for transform_name in &transformations {
        let transformed = apply_transformation_by_name(&current, transform_name);
        let (parse_ok, parse_error) = audit.test_parse(&transformed);
        
        audit.add_step(transform_name, transformed.clone(), parse_ok, parse_error);
        
        if parse_ok {
            current = transformed;
        }
    }

    // Try to parse the final result
    let (final_ok, _) = audit.test_parse(&current);
    
    if !final_ok {
        // If final result doesn't parse, run bisection
        match audit.bisect_transformations() {
            Ok(bisected_result) => {
                audit.final_success = true;
                Ok((bisected_result, audit))
            }
            Err(e) => Err(e.into())
        }
    } else {
        audit.final_success = true;
        Ok((current, audit))
    }
}

fn wrap_item(item: &Item) -> String {
    match item {
        Item::Mod(item_mod) => {
            let mod_name = &item_mod.ident;
            let content = if let Some((_, items)) = &item_mod.content {
                let wrapped_items: Vec<_> = items.iter().map(wrap_item).collect();
                wrapped_items.join("\n")
            } else {
                String::new()
            };
            format!("mkmod!{{{}, {{ 
                getname!({});
                getsrc!({});
                getpath!({});
                get_deps!({});
                get_crates!({});
                mkinclude!({});
                {} 
            }}}}", mod_name, mod_name, mod_name, mod_name, mod_name, mod_name, mod_name, content)
        }
        Item::Use(_) => {
            format!("mkuse!{{{}}}", quote::ToTokens::to_token_stream(item))
        }
        Item::Fn(item_fn) => {
            let fn_name = &item_fn.sig.ident;
            format!("
macro_rules! {}_introspect {{
    () => {{
        emit_message!(\"📊 INTROSPECT: Function {} in module {{}}\", module_path!());
    }};
}}

mkfn!{{
    {}_introspect!();
    {}
}}", fn_name, fn_name, fn_name, quote::ToTokens::to_token_stream(item))
        }
        Item::Struct(_) => {
            format!("mkitem!{{mkstruct!{{{}}}}}", quote::ToTokens::to_token_stream(item))
        }
        Item::Enum(_) => {
            format!("mkitem!{{mkenum!{{{}}}}}", quote::ToTokens::to_token_stream(item))
        }
        Item::Trait(_) => {
            format!("mkitem!{{mktrait!{{{}}}}}", quote::ToTokens::to_token_stream(item))
        }
        Item::Impl(_) => {
            format!("mkitem!{{mkimpl!{{{}}}}}", quote::ToTokens::to_token_stream(item))
        }
        _ => format!("mkitem!{{{}}}", quote::ToTokens::to_token_stream(item))
    }
}

fn create_minimal_test_case(file_path: &str, content: &str, error: &dyn std::error::Error) -> Result<(), Box<dyn std::error::Error>> {
    // Extract error type from error message
    let error_msg = error.to_string();
    let error_type = if error_msg.contains("expected square brackets") {
        "expected_square_brackets"
    } else if error_msg.contains("expected identifier") {
        "expected_identifier"  
    } else if error_msg.contains("expected `,`") {
        "expected_comma"
    } else if error_msg.contains("expected an expression") {
        "expected_expression"
    } else if error_msg.contains("unexpected token") {
        "unexpected_token"
    } else {
        "other_parse_error"
    };
    
    // Check if we should create a test case for this error type (max 3 per type)
    let mut counters = ERROR_COUNTERS.lock().unwrap();
    let count = counters.entry(error_type.to_string()).or_insert(0);
    *count += 1;
    
    // Only keep 3 examples of each error type
    if *count > 3 {
        return Ok(()); // Skip this test case
    }
    
    let file_name = file_path.split('/').last().unwrap_or("unknown");
    let test_case_name = format!("test_case_{}_{}.rs", error_type, file_name.replace('.', "_"));
    let test_case_path = format!("test_cases/{}", test_case_name);
    
    // Create test_cases directory if it doesn't exist
    fs::create_dir_all("test_cases")?;
    
    let test_content = format!(
        r#"// Test case for parsing error: {}
// Original file: {}
// Error type: {}
// Sample #{} of 3

use syn::parse_file;
use split_decls_genesis::build_lib::*;
use std::fs;

macro_rules! runbuild {{
    ($source_path:expr) => {{
        {{
            println!("🔧 Running build process on: {{}}", $source_path);
            
            // Read original source
            let original = fs::read_to_string($source_path).expect("Failed to read source file");
            println!("1️⃣ Original source loaded ({{}} bytes)", original.len());
            
            // Test original parsing
            match parse_file(&original) {{
                Ok(_) => println!("✅ Original parses fine"),
                Err(e) => {{
                    println!("❌ Original source broken: {{}}", e);
                    return;
                }}
            }}
            
            // Apply transformations step by step
            println!("\\n2️⃣ Adding prelude...");
            let step2 = add_prelude(&original);
            match parse_file(&step2) {{
                Ok(_) => println!("✅ After prelude: Parse OK"),
                Err(e) => {{
                    println!("❌ Prelude broke parsing: {{}}", e);
                    return;
                }}
            }}
            
            println!("\\n3️⃣ Running full process_content...");
            match process_content(&original) {{
                Ok(result) => {{
                    println!("✅ Full process completed ({{}} bytes)", result.len());
                    
                    // Test final result parsing
                    match parse_file(&result) {{
                        Ok(_) => println!("✅ Final result parses fine"),
                        Err(e) => println!("❌ Final result broken: {{}}", e),
                    }}
                }}
                Err(e) => println!("❌ Process failed: {{}}", e),
            }}
        }}
    }};
}}

fn main() {{
    runbuild!("{}");
}}
"#,
        error_msg.lines().next().unwrap_or("unknown error"),
        file_path,
        error_type,
        *count,
        file_path
    );
    
    fs::write(&test_case_path, test_content)?;
    println!("📝 Created test case: {} (sample {}/3)", test_case_name, *count);
    
    Ok(())
}

fn process_file(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    
    // Use the working audit system from build_lib
    match process_content_with_audit(&content) {
        Ok((result, audit)) => {
            println!("    ✅ Audit completed: {}/{} transformations successful, final: {}", 
                audit.steps.iter().filter(|s| s.success).count(),
                audit.steps.len(),
                audit.final_success);
            
            // Show any bisection that occurred
            if !audit.bisection_log.is_empty() {
                println!("    🔍 Bisection performed:");
                for log_entry in &audit.bisection_log {
                    println!("      {}", log_entry);
                }
            }
            
            Ok(result)
        }
        Err(e) => {
            println!("    ❌ Audit failed: {}", e);
            Err(e)
        }
    }
}

fn oldmain() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=symbol_map.json.gz");
    println!("cargo:rerun-if-changed=../rust");
    
    // Check environment variables
    println!("🔍 Checking environment variables...");
    if let Ok(skip) = std::env::var("SKIP_BUILD") {
        println!("⏭️ SKIP_BUILD={}, exiting early", skip);
        return Ok(());
    }
    
    if let Ok(scripts) = std::env::var("CARGO_BUILD_SCRIPTS") {
        println!("📋 CARGO_BUILD_SCRIPTS={}", scripts);
        if scripts == "false" {
            println!("⏭️ Build scripts disabled, exiting early");
            return Ok(());
        }
    }
    
    println!("🚀 Building with macro wrappers...");
    
    // Check if we have a recent successful build
    let cache_path = Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap_or(".".to_string())).join("build_cache.json");
    if cache_path.exists() {
        if let Ok(cache_content) = std::fs::read_to_string(&cache_path) {
            if let Ok(cache_data) = serde_json::from_str::<serde_json::Value>(&cache_content) {
                if let Some(timestamp) = cache_data.get("timestamp").and_then(|t| t.as_u64()) {
                    let cache_age = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)?
                        .as_secs() - timestamp;
                    
                    println!("💾 Build cache age: {} seconds", cache_age);
                    
                    // Only rebuild if cache is older than 1 hour (3600 seconds)
                    if cache_age < 3600 {
                        println!("✅ Build cache is recent (< 1 hour old), skipping symbol processing");
                        println!("💡 To force rebuild: rm build_cache.json");
                        return Ok(());
                    }
                }
            }
        }
    }
    
    println!("🚀 Starting symbol map processing...");
    println!("📊 Loading symbol map from symbol_map.json.gz...");
        let start_time = std::time::Instant::now();
        let file = std::fs::File::open("symbol_map.json.gz")?;
        let decoder = flate2::read::GzDecoder::new(file);
        let symbol_map: serde_json::Value = serde_json::from_reader(decoder)?;
        println!("⏱️ Symbol map loaded in {:?}", start_time.elapsed());
        
        if let Some(obj) = symbol_map.as_object() {
            println!("🔍 Processing {} symbol entries...", obj.len());
            let mut source_files = std::collections::HashSet::new();
            
            // Extract unique source files from symbol map
            for (_symbol_name, entry) in obj.iter() {
                if let Some(source_file) = entry.get("source_file").and_then(|s| s.as_str()) {
                    if source_file.ends_with(".rs") && !source_file.contains("test") {
                        source_files.insert(source_file.to_string());
                    }
                }
            }
            
            println!("📁 Found {} unique source files from {} symbols", source_files.len(), obj.len());
            
            // Create submodules directory structure
            println!("📂 Creating submodules directory...");
            fs::create_dir_all("submodules")?;
            
            let total_files = source_files.len();
            let mut processed_count = 0;
            let mut success_count = 0;
            let mut failure_count = 0;
            let start_time = std::time::Instant::now();
            
            // Process each source file
            for (i, source_file) in source_files.iter().enumerate() {
                let file_path = source_file.replace("../rust/", "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/rust/");
                
                if !Path::new(&file_path).exists() {
                    println!("⚠️  File not found: {}", file_path);
                    continue;
                }
                
                processed_count += 1;
                let file_start = std::time::Instant::now();
                
                println!("🔄 Processing {}/{}: {} ({})", i+1, total_files, source_file, file_path);
                
                match process_file(&file_path) {
                    Ok(wrapped_content) => {
                        // Create proper submodules directory structure
                        let output_path = format!("submodules/{}", source_file.replace("../rust/", "rust/"));
                        
                        // Create parent directories
                        if let Some(parent) = Path::new(&output_path).parent() {
                            fs::create_dir_all(parent)?;
                        }
                        
                        fs::write(&output_path, wrapped_content)?;
                        success_count += 1;
                        let msg = format!("✅ {}/{}: {} -> {} ({:?})", i+1, total_files, source_file, output_path, file_start.elapsed());
                        println!("{}", msg);
                        
                        // Also log to file
                        if let Ok(mut audit_log) = AUDIT_LOG.lock() {
                            audit_log.push(msg);
                        }
                    }
                    Err(e) => {
                        failure_count += 1;
                        let msg = format!("❌ Failed to process {} ({:?}): {}", source_file, file_start.elapsed(), e);
                        println!("{}", msg);
                        
                        // Also log to file
                        if let Ok(mut audit_log) = AUDIT_LOG.lock() {
                            audit_log.push(msg.clone());
                        }
                        
                        // Create minimal test case for this failure
                        if let Ok(content) = fs::read_to_string(&file_path) {
                            if let Err(test_err) = create_minimal_test_case(&source_file, &content, e.as_ref()) {
                                let test_msg = format!("⚠️  Failed to create test case: {}", test_err);
                                println!("{}", test_msg);
                                if let Ok(mut audit_log) = AUDIT_LOG.lock() {
                                    audit_log.push(test_msg);
                                }
                            }
                        }
                    }
                }
                
                // Progress report every 100 files
                if processed_count % 100 == 0 {
                    let elapsed = start_time.elapsed();
                    let rate = processed_count as f64 / elapsed.as_secs_f64();
                    let progress_msg = format!("📊 Progress: {}/{} files ({:.1}%), {:.1} files/sec, ✅{} ❌{}", 
                             processed_count, total_files, 
                             (processed_count as f64 / total_files as f64) * 100.0,
                             rate, success_count, failure_count);
                    println!("{}", progress_msg);
                    
                    // Also log to file
                    if let Ok(mut audit_log) = AUDIT_LOG.lock() {
                        audit_log.push(progress_msg);
                    }
                }
            }
            
            // Final summary
            let total_elapsed = start_time.elapsed();
            let final_rate = processed_count as f64 / total_elapsed.as_secs_f64();
            println!("\n🏁 BUILD.RS COMPLETE");
            println!("📊 Final Summary:");
            println!("   Total time: {:?}", total_elapsed);
            println!("   Files processed: {}/{}", processed_count, total_files);
            println!("   Success rate: {:.1}% ({}/{})", 
                     (success_count as f64 / processed_count as f64) * 100.0, 
                     success_count, processed_count);
            println!("   Failures: {}", failure_count);
            println!("   Average rate: {:.1} files/sec", final_rate);
            
            // Create build cache to avoid reprocessing
            let cache_data = serde_json::json!({
                "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs(),
                "source_files_count": source_files.len(),
                "symbols_count": obj.len()
            });
            std::fs::write("build_cache.json", serde_json::to_string_pretty(&cache_data)?)?;
            println!("💾 Build cache updated");
            
            // Write audit log
            if let Ok(audit_log) = AUDIT_LOG.lock() {
                if !audit_log.is_empty() {
                    println!("📝 Writing audit log ({} entries)...", audit_log.len());
                    fs::write("build_audit.log", audit_log.join("\n"))?;
                }
            }
        }
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Test the transformation functions on our test cases
    println!("🧪 Testing transformation functions on test cases...");
    
    let test_cases_dir = "test_cases";
    if !Path::new(test_cases_dir).exists() {
        println!("❌ Test cases directory not found");
        return Ok(());
    }
    
    let test_files: Vec<_> = std::fs::read_dir(test_cases_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "rs"))
        .collect();
    
    println!("📁 Found {} test case files", test_files.len());
    
    for (i, entry) in test_files.iter().enumerate() {
        let file_path = entry.path();
        let file_name = file_path.file_name().unwrap().to_string_lossy();
        
        println!("\n🔄 Testing {}/{}: {}", i+1, test_files.len(), file_name);
        
        // Read the test case to extract original file path
        let content = std::fs::read_to_string(&file_path)?;
        
        // Extract the original file path from the test case comment
        if let Some(line) = content.lines().find(|line| line.starts_with("// Original file:")) {
            if let Some(original_path) = line.strip_prefix("// Original file: ") {
                let full_path = original_path.replace("../rust/", "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/");
                
                if Path::new(&full_path).exists() {
                    println!("  📄 Processing original: {}", original_path);
                    
                    match process_file(&full_path) {
                        Ok(result) => {
                            println!("  ✅ Transformation successful!");
                            
                            // Test if the result parses
                            match syn::parse_file(&result) {
                                Ok(_) => println!("  ✅ Result parses correctly!"),
                                Err(e) => println!("  ❌ Result still has parse error: {}", e),
                            }
                        }
                        Err(e) => {
                            println!("  ❌ Transformation failed: {}", e);
                        }
                    }
                } else {
                    println!("  ⚠️  Original file not found: {}", full_path);
                }
            }
        }
    }
    
    Ok(())
}
