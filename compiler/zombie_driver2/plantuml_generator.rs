use serde_json::Value;
use std::collections::HashSet;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 PLANTUML GENERATOR - Rust Compiler Flow Diagram");
    println!("==================================================");

    let mut plantuml = String::new();

    // PlantUML header
    plantuml.push_str("@startuml RustCompilerFlow\n");
    plantuml.push_str("!theme plain\n");
    plantuml.push_str("skinparam backgroundColor #f8f9fa\n");
    plantuml.push_str("skinparam activity {\n");
    plantuml.push_str("  BackgroundColor #e3f2fd\n");
    plantuml.push_str("  BorderColor #1976d2\n");
    plantuml.push_str("}\n\n");

    plantuml
        .push_str("title Rust Compiler Execution Flow\\nFrom run_compiler to Code Generation\n\n");
    plantuml.push_str("start\n\n");

    // Entry point
    plantuml.push_str(":🎯 **ENTRY POINT**\\nrun_compiler closure\\n(0x42f4980);\n");
    plantuml.push_str("note right: Primary compilation entry\\nLMFDB: 12.2.12.o\n\n");

    // Load actual call data
    let main_file = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/complete_analysis/chunk_00000042/addr_42f4980__ZN17rustc_driver_impl12run_compiler28__u7b__u7b_c.json";
    let content = fs::read_to_string(main_file)?;
    let data: Value = serde_json::from_str(&content)?;

    // Direct function calls
    if let Some(calls_to) = data["function_calls"]["calls_to"].as_array() {
        plantuml.push_str(":📞 **DIRECT CALLS**;\n");
        for (i, target) in calls_to.iter().enumerate() {
            if let Some(addr) = target.as_str() {
                plantuml.push_str(&format!(":Call {}\\n{};\n", i + 1, addr));
            }
        }
        plantuml.push_str("\n");
    }

    // Syscalls section
    plantuml.push_str(":🔧 **SYSCALLS**;\n");
    plantuml.push_str("partition \"System Interface\" {\n");
    plantuml.push_str("  :read() - Source files;\n");
    plantuml.push_str("  :write() - Output files;\n");
    plantuml.push_str("  :mmap() - Memory mapping;\n");
    plantuml.push_str("}\n\n");

    // Compilation phases
    plantuml.push_str(":🚀 **COMPILATION PHASES**;\n");
    plantuml.push_str("partition \"Frontend\" {\n");
    plantuml.push_str("  :📖 **Lexical Analysis**\\nTokens: TokenKind;\n");
    plantuml.push_str("  :🌳 **Parsing**\\nAST: Expr, ItemKind;\n");
    plantuml.push_str("  :⬇️ **HIR Lowering**\\nHIR: HirId, HirKind;\n");
    plantuml.push_str("}\n\n");

    plantuml.push_str("partition \"Analysis\" {\n");
    plantuml.push_str("  :🔍 **Type Checking**\\nTypes: Ty, TyKind, TyCtxt;\n");
    plantuml.push_str("}\n\n");

    plantuml.push_str("partition \"Backend\" {\n");
    plantuml.push_str("  :🏗️ **MIR Building**\\nMIR: BasicBlock, StatementKind;\n");
    plantuml.push_str("  :⚙️ **Code Generation**\\nCodegenUnit;\n");
    plantuml.push_str("}\n\n");

    // Types and enums
    plantuml.push_str("note left\n");
    plantuml.push_str("**Key Types:**\n");
    plantuml.push_str("• Ty, TyCtxt, HirId\n");
    plantuml.push_str("• Expr, Token, Mir\n");
    plantuml.push_str("• Ast, BasicBlock, Hir\n");
    plantuml.push_str("• CodegenUnit\n\n");
    plantuml.push_str("**Key Enums:**\n");
    plantuml.push_str("• ItemKind, TokenKind\n");
    plantuml.push_str("• ExprKind, TyKind\n");
    plantuml.push_str("• HirKind, StatementKind\n");
    plantuml.push_str("end note\n\n");

    plantuml.push_str("stop\n");
    plantuml.push_str("@enduml\n");

    // Write to file
    let output_file = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/rust_compiler_flow.puml";
    fs::write(&output_file, &plantuml)?;

    println!("✅ PlantUML diagram generated: {}", output_file);
    println!("\n📋 PLANTUML CONTENT:");
    println!("{}", plantuml);

    Ok(())
}
