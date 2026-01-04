use split_decls_genesis::daemon::{MatrixDaemon};
use split_decls_genesis::symbol_cache::SymbolCache;
use split_decls_genesis::function_graduator::FunctionGraduator;
use split_decls_genesis::ast_tracer::{init_tracer, trace_ast};
use split_decls_genesis::hir_tracer::run_with_hir_tracing;
use std::io::{self, Write};
use std::collections::HashMap;

async fn eval_symbol(symbol_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Evaluating symbol: {}", symbol_name);
    
    let mut cache = SymbolCache::new();
    cache.load_or_create()?;
    
    if let Some(symbol) = cache.get(symbol_name) {
        println!("✅ Found symbol: {} (type: {})", symbol_name, symbol.symbol_type);
        
        // Create simple function file
        let code = format!(r#"
// Function: {}
// Type: {}
// Dependencies: {:?}

fn main() {{
    println!("Symbol {} evaluated successfully");
}}
"#, symbol.name, symbol.symbol_type, symbol.dependencies, symbol.name);
        
        // Save to functions directory
        std::fs::create_dir_all("./functions")?;
        let safe_name = symbol_name.replace("::", "_").replace("<", "_").replace(">", "_");
        let file_path = format!("./functions/{}.rs", safe_name);
        std::fs::write(&file_path, &code)?;
        
        println!("💾 Saved function to {}", file_path);
        println!("🎯 Function ready for hot reload");
    } else {
        println!("❌ Symbol '{}' not found", symbol_name);
    }
    
    Ok(())
}

async fn query_symbols(pattern: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut cache = SymbolCache::new();
    cache.load_or_create()?;
    
    let matches: Vec<_> = cache.keys()
        .filter(|name| name.contains(pattern))
        .take(20)
        .collect();
    
    println!("Found {} matches (showing first 20):", matches.len());
    for symbol in matches {
        println!("  {}", symbol);
    }
    Ok(())
}

async fn list_categories() -> Result<(), Box<dyn std::error::Error>> {
    let mut cache = SymbolCache::new();
    cache.load_or_create()?;
    
    let mut categories = std::collections::HashMap::new();
    
    for name in cache.keys().take(1000) { // Sample first 1000
        let category = if name.contains("::main") { "main_functions" }
        else if name.contains("::new") { "constructors" }
        else if name.contains("rustc_") { "rustc_internals" }
        else if name.contains("std::") { "std_library" }
        else if name.contains("::test") { "tests" }
        else { "other" };
        
        *categories.entry(category).or_insert(0) += 1;
    }
    
    println!("📊 Symbol Categories:");
    for (cat, count) in categories {
        println!("  {}: {} symbols", cat, count);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut daemon = MatrixDaemon::new("rustc-matrix");
    
    println!("🎯 RustC Matrix Service Manager");
    println!("Commands: start, stop, restart, status, eval <symbol>, query <pattern>, categories, graduate, verb <name> [args...], compile <symbol> [rustc-args...], hir <file> [rustc-args...], search <pattern>, verbs, quit");
    
    loop {
        print!("matrix> ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        match input {
            "start" => {
                daemon.start()?;
            }
            "stop" => {
                daemon.stop()?;
            }
            "restart" => {
                daemon.restart()?;
            }
            "status" => {
                let actual_status = daemon.status();
                println!("Service: {} - Status: {:?}", daemon.name, actual_status);
            }
            cmd if cmd.starts_with("eval ") => {
                let symbol = cmd.strip_prefix("eval ").unwrap();
                println!("🔍 Evaluating symbol: {}", symbol);
                eval_symbol(symbol).await?;
            }
            cmd if cmd.starts_with("query ") => {
                let pattern = cmd.strip_prefix("query ").unwrap();
                println!("🔍 Querying symbols matching: {}", pattern);
                query_symbols(pattern).await?;
            }
            "graduate" => {
                println!("🎓 Initializing function graduator...");
                let mut graduator = FunctionGraduator::new();
                match graduator.init() {
                    Ok(_) => {
                        println!("✅ Ready! {} symbols available as verbs", graduator.get_symbol_count());
                        println!("💡 Try: verb <name>, search <pattern>, or verbs");
                    }
                    Err(e) => println!("❌ Initialization failed: {}", e),
                }
            }
            cmd if cmd.starts_with("verb ") => {
                let parts: Vec<&str> = cmd.strip_prefix("verb ").unwrap().split_whitespace().collect();
                if parts.is_empty() {
                    println!("❌ Usage: verb <name> [args...]");
                    continue;
                }
                
                let verb = parts[0];
                let args = &parts[1..];
                
                let mut graduator = FunctionGraduator::new();
                graduator.init()?;
                match graduator.call_verb_with_args(verb, args) {
                    Ok(result) => println!("{}", result),
                    Err(e) => println!("❌ {}", e),
                }
            }
            cmd if cmd.starts_with("search ") => {
                let pattern = cmd.strip_prefix("search ").unwrap();
                let mut graduator = FunctionGraduator::new();
                graduator.init()?;
                let matches = graduator.search_verbs(pattern);
                println!("🔍 Found {} verbs matching '{}':", matches.len(), pattern);
                for verb in matches.iter().take(10) {
                    println!("  {}", verb);
                }
                if matches.len() > 10 {
                    println!("  ... and {} more", matches.len() - 10);
                }
            }
            "verbs" => {
                let mut graduator = FunctionGraduator::new();
                graduator.init()?;
                let verbs = graduator.list_available_verbs();
                println!("📋 Available verbs ({} total):", verbs.len());
                for verb in verbs.iter().take(20) {
                    println!("  {}", verb);
                }
                if verbs.len() > 20 {
                    println!("  ... and {} more", verbs.len() - 20);
                }
            }
            cmd if cmd.starts_with("compile ") => {
                let parts: Vec<&str> = cmd.strip_prefix("compile ").unwrap().split_whitespace().collect();
                if parts.is_empty() {
                    println!("❌ Usage: compile <symbol> [rustc-args...]");
                    continue;
                }
                
                let symbol = parts[0];
                let args = &parts[1..];
                
                // Initialize AST tracer
                let tracer = init_tracer();
                tracer.clear();
                
                // Trace the compilation start
                let mut metadata = HashMap::new();
                metadata.insert("symbol".to_string(), symbol.to_string());
                metadata.insert("args".to_string(), args.join(" "));
                trace_ast("compile_start", "rustc_main", "matrix_ctl", metadata);
                
                let mut graduator = FunctionGraduator::new();
                graduator.init()?;
                match graduator.compile_symbol_with_tracing(symbol, args) {
                    Ok(result) => {
                        println!("{}", result);
                        
                        // Show AST traces
                        let traces = tracer.get_traces();
                        println!("\n📊 AST Trace Report ({} operations):", traces.len());
                        for (i, trace) in traces.iter().enumerate().take(10) {
                            println!("  {}. {} -> {} at {}", 
                                i + 1, trace.operation, trace.node_type, trace.location);
                        }
                        if traces.len() > 10 {
                            println!("  ... and {} more operations", traces.len() - 10);
                        }
                    }
                    Err(e) => println!("❌ {}", e),
                }
            }
            "categories" => {
                println!("📊 Analyzing symbol categories...");
                list_categories().await?;
            }
            cmd if cmd.starts_with("hir ") => {
                let parts: Vec<&str> = cmd.strip_prefix("hir ").unwrap().split_whitespace().collect();
                if parts.is_empty() {
                    println!("❌ Usage: hir <file> [rustc-args...]");
                    continue;
                }
                
                let file = parts[0];
                let mut args = vec!["rustc".to_string(), file.to_string()];
                args.extend(parts[1..].iter().map(|s| s.to_string()));
                
                println!("🔍 Running HIR analysis on: {}", file);
                run_with_hir_tracing(args);
            }
            "quit" | "exit" => {
                daemon.stop()?;
                break;
            }
            "" => continue,
            _ => {
                println!("Unknown command. Available: start, stop, restart, status, eval <symbol>, quit");
            }
        }
    }
    
    Ok(())
}
