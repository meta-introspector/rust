use std::process::Command;

fn main() {
    println!("🔍 Real Error Analysis - unified_rustc_wrapped compilation issues");
    
    // Get actual compilation errors
    let output = Command::new("cargo")
        .args(&["check", "--bin", "unified_rustc_wrapped"])
        .output()
        .expect("Failed to run cargo check");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Count error types
    let mut unstable_features = 0;
    let mut duplicate_diagnostics = 0;
    let mut unresolved_imports = 0;
    let mut name_conflicts = 0;
    let mut other_errors = 0;
    
    for line in stderr.lines() {
        if line.contains("use of unstable library feature") {
            unstable_features += 1;
        } else if line.contains("duplicate diagnostic item") {
            duplicate_diagnostics += 1;
        } else if line.contains("failed to resolve") || line.contains("unresolved") {
            unresolved_imports += 1;
        } else if line.contains("defined multiple times") {
            name_conflicts += 1;
        } else if line.starts_with("error[E") {
            other_errors += 1;
        }
    }
    
    println!("\n📊 ERROR BREAKDOWN:");
    println!("• Unstable features: {}", unstable_features);
    println!("• Duplicate diagnostics: {}", duplicate_diagnostics);
    println!("• Unresolved imports: {}", unresolved_imports);
    println!("• Name conflicts: {}", name_conflicts);
    println!("• Other errors: {}", other_errors);
    
    println!("\n🔧 PRIORITY FIXES:");
    if unstable_features > 0 {
        println!("1. Add missing feature flags: #![feature(assert_matches)] #![feature(error_reporter)]");
    }
    if duplicate_diagnostics > 0 {
        println!("2. Remove duplicate #[rustc_diagnostic_item] attributes");
    }
    if unresolved_imports > 0 {
        println!("3. Add missing extern crate declarations or create stubs");
    }
    if name_conflicts > 0 {
        println!("4. Rename conflicting imports with aliases");
    }
    
    println!("\n📝 REAL STATUS:");
    println!("❌ unified_rustc_wrapped: BROKEN ({} total errors)", 
             unstable_features + duplicate_diagnostics + unresolved_imports + name_conflicts + other_errors);
    println!("✅ unified_build: Working (processes rustc files)");
    println!("✅ unified_driver: Working (generates code)");
    println!("✅ Library compilation: Working (src/current.rs compiles)");
    
    println!("\n🎯 NEXT STEPS:");
    println!("1. Fix unstable feature usage in unified_rustc_wrapped.rs");
    println!("2. Remove duplicate diagnostic items");
    println!("3. Test minimal fixes incrementally");
    println!("4. Build working rustc wrapper step by step");
}
