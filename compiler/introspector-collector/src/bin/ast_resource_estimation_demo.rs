use introspector_collector::{est};
use introspector_collector::ast_resource_estimation::*;
use introspector_collector::system_introspection::SystemIntrospector;

fn main() {
    println!("📊 AST RESOURCE ESTIMATION");
    println!("🎯 Each operation carries resource cost and complexity");
    println!("📈 est(mem, [ast, block, fun, mod, crate, repo, ecosystem])");
    
    // Initialize system and estimator
    let system = SystemIntrospector::new();
    let estimator = ResourceEstimator::new(system.detected_resources);
    
    // Show operation costs
    println!("\n💾 BASE OPERATION COSTS:");
    let operations = [
        ("literal", "Literal value"),
        ("variable", "Variable access"),
        ("binary_op", "Binary operation"),
        ("function_call", "Function call"),
        ("assignment", "Assignment"),
        ("if_statement", "If statement"),
        ("loop", "Loop construct"),
        ("block", "Code block"),
        ("function", "Function definition"),
        ("module", "Module"),
        ("crate", "Crate"),
        ("repository", "Repository"),
        ("ecosystem", "Ecosystem"),
    ];
    
    for (op, desc) in &operations {
        if let Some(cost) = estimator.operation_costs.get(*op) {
            println!("  {}: {} cycles, {:.1}KB, complexity {:.1}", 
                desc, cost.cpu_cycles, cost.memory_bytes as f64 / 1024.0, cost.complexity_score);
        }
    }
    
    // Demonstrate est() macro
    println!("\n📊 RESOURCE ESTIMATION BY LEVEL:");
    let memory_estimates = est!(mem, ["ast", "block", "function", "module", "crate", "repository", "ecosystem"]);
    
    for (level, cost) in &memory_estimates {
        let ram_mb = cost.memory_bytes as f64 / (1024.0 * 1024.0);
        let can_handle = estimator.can_system_handle(cost);
        let status = if can_handle { "✅" } else { "❌" };
        
        println!("  {} {}: {:.2}MB RAM, {} cycles, complexity {:.1}", 
            status, level.to_uppercase(), ram_mb, cost.cpu_cycles, cost.complexity_score);
    }
    
    // Generate complete report
    println!("\n📋 COMPLETE RESOURCE REPORT:");
    let report = est!(report, ["ast", "block", "function", "module", "crate", "repository", "ecosystem"]);
    println!("{}", report);
    
    // Build sample annotated AST
    println!("\n🌳 SAMPLE ANNOTATED AST:");
    
    // Create a simple function AST: fn add(a: i32, b: i32) -> i32 { a + b }
    let literal_a = estimator.annotate_ast(ASTNodeType::Variable("a".to_string()), vec![]);
    let literal_b = estimator.annotate_ast(ASTNodeType::Variable("b".to_string()), vec![]);
    let binary_op = estimator.annotate_ast(ASTNodeType::BinaryOp("+".to_string()), vec![literal_a, literal_b]);
    let return_stmt = estimator.annotate_ast(ASTNodeType::Return, vec![binary_op]);
    let block = estimator.annotate_ast(ASTNodeType::Block, vec![return_stmt]);
    let function = estimator.annotate_ast(ASTNodeType::Function("add".to_string()), vec![block]);
    
    println!("Function 'add' resource annotation:");
    println!("{}", estimator.generate_annotation(&function.accumulated_cost));
    
    // Show resource accumulation
    println!("\n📈 RESOURCE ACCUMULATION:");
    println!("  Variable 'a': {} cycles, {}B", 
        function.children[0].children[0].children[0].operation_cost.cpu_cycles,
        function.children[0].children[0].children[0].operation_cost.memory_bytes);
    println!("  Variable 'b': {} cycles, {}B", 
        function.children[0].children[0].children[1].operation_cost.cpu_cycles,
        function.children[0].children[0].children[1].operation_cost.memory_bytes);
    println!("  Binary '+': {} cycles, {}B", 
        function.children[0].children[0].operation_cost.cpu_cycles,
        function.children[0].children[0].operation_cost.memory_bytes);
    println!("  Return: {} cycles, {}B", 
        function.children[0].operation_cost.cpu_cycles,
        function.children[0].operation_cost.memory_bytes);
    println!("  Block: {} cycles, {}B", 
        function.children[0].accumulated_cost.cpu_cycles,
        function.children[0].accumulated_cost.memory_bytes);
    println!("  Function total: {} cycles, {}B", 
        function.accumulated_cost.cpu_cycles,
        function.accumulated_cost.memory_bytes);
    
    // Show scaling across levels
    println!("\n🔢 SCALING ACROSS ABSTRACTION LEVELS:");
    let levels = ["ast", "block", "function", "module", "crate", "repository", "ecosystem"];
    let base_cost = estimator.operation_costs.get("literal").unwrap();
    
    for level in &levels {
        let multiplier = estimator.level_multipliers.get(*level).unwrap_or(&1.0);
        let scaled = base_cost.scale(*multiplier);
        println!("  {}: {}x multiplier = {} cycles, {:.2}KB", 
            level, multiplier, scaled.cpu_cycles, scaled.memory_bytes as f64 / 1024.0);
    }
    
    // System capacity analysis
    println!("\n🖥️  SYSTEM CAPACITY ANALYSIS:");
    println!("Available resources:");
    println!("  RAM: {}GB", system.detected_resources.available_ram_gb);
    println!("  CPU: {} cores", system.detected_resources.cpu_cores);
    
    println!("\nWhat your system can handle:");
    for (level, cost) in &memory_estimates {
        let can_handle = estimator.can_system_handle(cost);
        let ram_gb = cost.memory_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
        
        if can_handle {
            println!("  ✅ {}: {:.3}GB RAM needed", level, ram_gb);
        } else {
            println!("  ❌ {}: {:.3}GB RAM needed (exceeds capacity)", level, ram_gb);
        }
    }
    
    // Practical implications
    println!("\n🛠️  PRACTICAL IMPLICATIONS:");
    println!("  📊 Every AST node now has resource annotations");
    println!("  💾 Memory usage accumulates from leaves to root");
    println!("  🔥 CPU cycles scale with operation complexity");
    println!("  📈 Higher abstraction levels have exponential costs");
    println!("  🎯 System can predict resource needs before execution");
    println!("  ⚠️  Large codebases may exceed system capacity");
    
    // Code generation with annotations
    println!("\n💻 GENERATED CODE WITH ANNOTATIONS:");
    println!("```rust");
    println!("// Generated resource annotations");
    println!("{}", estimator.generate_annotation(&function.accumulated_cost));
    println!("fn add(a: i32, b: i32) -> i32 {{");
    println!("    a + b  // Binary op: {} cycles, {}B", 
        function.children[0].children[0].operation_cost.cpu_cycles,
        function.children[0].children[0].operation_cost.memory_bytes);
    println!("}}");
    println!("```");
    
    println!("\n✨ AST RESOURCE ESTIMATION COMPLETE!");
    println!("📊 Every operation now carries resource cost");
    println!("🎯 est(mem, [ast,block,fun,mod,crate,repo,ecosystem]) implemented");
    println!("💾 Resource accumulation works across all abstraction levels");
    println!("🖥️  System capacity analysis integrated");
    println!("📈 Ready for complexity-aware code generation!");
    
    // Save estimation results
    std::fs::create_dir_all("src/generated/ast_estimation").ok();
    
    std::fs::write("src/generated/ast_estimation/resource_report.txt", report)
        .expect("Failed to write resource report");
    
    let function_annotation = estimator.generate_annotation(&function.accumulated_cost);
    std::fs::write("src/generated/ast_estimation/sample_function_annotation.rs", 
        format!("// Sample function with resource annotation\n{}\nfn add(a: i32, b: i32) -> i32 {{\n    a + b\n}}", function_annotation))
        .expect("Failed to write function annotation");
    
    // Save operation costs as JSON
    let costs_json = serde_json::to_string_pretty(&estimator.operation_costs)
        .expect("Failed to serialize operation costs");
    std::fs::write("src/generated/ast_estimation/operation_costs.json", costs_json)
        .expect("Failed to write operation costs");
    
    println!("💾 AST resource estimation results saved!");
}
