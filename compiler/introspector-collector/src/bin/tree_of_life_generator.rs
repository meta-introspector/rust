use introspector_collector::rust_tree_of_life::*;

fn main() {
    println!("Creating Rust Tree of Life model...");
    
    let mut tree = RustTreeOfLife::new();
    
    // Bind actual rustc enums from our collected data
    tree.bind_rustc_enum("rustc_ast::ItemKind", vec![
        "Const".to_string(),
        "Fn".to_string(), 
        "Enum".to_string(),
        "Struct".to_string(),
        "Trait".to_string(),
    ]);
    
    tree.bind_rustc_enum("rustc_hir::ExprKind", vec![
        "Call".to_string(),
        "Match".to_string(),
        "If".to_string(),
        "Loop".to_string(),
    ]);
    
    // Bind our generated macros
    tree.bind_macro("mk_itemkind_to_string", "macro_rules! mk_itemkind_to_string { ... }");
    
    // Generate the complete tree structure
    tree.generate_tree();
    
    // Generate executable Rust code
    let generated_code = tree.codegen();
    
    // Write to file
    std::fs::write("src/generated/rust_tree_model.rs", &generated_code)
        .expect("Failed to write tree model");
    
    println!("Tree of Life model generated!");
    println!("Enum bindings: {}", tree.enum_bindings.len());
    println!("Macro bindings: {}", tree.macro_bindings.len());
    
    // Show the structure
    println!("\nTree structure:");
    println!("{:#?}", tree.root);
}
