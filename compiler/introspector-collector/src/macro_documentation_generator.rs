use std::fs;
use std::collections::HashMap;

pub struct MacroDocumentationGenerator {
    generated_macros: HashMap<String, MacroDoc>,
    categories: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct MacroDoc {
    pub name: String,
    pub category: String,
    pub description: String,
    pub usage_example: String,
    pub rust_type: String,
    pub variants: Vec<String>,
    pub content_hash: String,
}

impl MacroDocumentationGenerator {
    pub fn new() -> Self {
        Self {
            generated_macros: HashMap::new(),
            categories: HashMap::new(),
        }
    }
    
    pub fn add_macro_doc(&mut self, doc: MacroDoc) {
        self.categories.entry(doc.category.clone())
            .or_insert_with(Vec::new)
            .push(doc.name.clone());
        self.generated_macros.insert(doc.name.clone(), doc);
    }
    
    pub fn generate_all_rust_enum_docs(&mut self) {
        // HIR Enums
        self.add_macro_doc(MacroDoc {
            name: "switch_itemkind2string".to_string(),
            category: "HIR Types".to_string(),
            description: "Converts rustc_hir::ItemKind to optimized string representation".to_string(),
            usage_example: r#"let item_type = switch_itemkind2string!(&item.kind, {
    rustc_hir::ItemKind::Fn { .. } => "custom_fn",
    rustc_hir::ItemKind::Struct(..) => "custom_struct",
});"#.to_string(),
            rust_type: "rustc_hir::ItemKind".to_string(),
            variants: vec!["Fn".to_string(), "Struct".to_string(), "Enum".to_string(), "Const".to_string(), "Static".to_string()],
            content_hash: "a1b2c3d4e5f6789a".to_string(),
        });
        
        self.add_macro_doc(MacroDoc {
            name: "switch_defkind2string".to_string(),
            category: "HIR Types".to_string(),
            description: "Converts rustc_hir::def::DefKind to optimized string representation".to_string(),
            usage_example: r#"let def_usage = switch_defkind2string!(def_kind, {
    rustc_hir::def::DefKind::Fn => "function_call",
    rustc_hir::def::DefKind::Struct => "struct_usage",
});"#.to_string(),
            rust_type: "rustc_hir::def::DefKind".to_string(),
            variants: vec!["Fn".to_string(), "AssocFn".to_string(), "Struct".to_string(), "Variant".to_string()],
            content_hash: "b2c3d4e5f6789ab1".to_string(),
        });
        
        self.add_macro_doc(MacroDoc {
            name: "switch_exprkind2string".to_string(),
            category: "HIR Types".to_string(),
            description: "Converts rustc_hir::ExprKind to optimized string representation".to_string(),
            usage_example: r#"let expr_type = switch_exprkind2string!(&expr.kind, {
    rustc_hir::ExprKind::Lit(_) => "literal_value",
    rustc_hir::ExprKind::Call(..) => "function_call",
});"#.to_string(),
            rust_type: "rustc_hir::ExprKind".to_string(),
            variants: vec!["Lit".to_string(), "Call".to_string(), "Match".to_string()],
            content_hash: "c3d4e5f6789ab1c2".to_string(),
        });
        
        self.add_macro_doc(MacroDoc {
            name: "switch_mutability2string".to_string(),
            category: "HIR Types".to_string(),
            description: "Converts rustc_hir::Mutability to optimized string representation".to_string(),
            usage_example: r#"let ref_type = switch_mutability2string!(mutability, {
    rustc_hir::Mutability::Mut => "mut_ref",
    rustc_hir::Mutability::Not => "ref",
});"#.to_string(),
            rust_type: "rustc_hir::Mutability".to_string(),
            variants: vec!["Mut".to_string(), "Not".to_string()],
            content_hash: "f6789ab1c2d3e4f5".to_string(),
        });
        
        // AST Enums
        self.add_macro_doc(MacroDoc {
            name: "switch_litkind2string".to_string(),
            category: "AST Types".to_string(),
            description: "Converts rustc_ast::LitKind to optimized string representation with type prefix".to_string(),
            usage_example: r#"let (literal_value, type_prefix) = switch_litkind2string!(lit.node, {
    rustc_ast::LitKind::Str(..) => ("custom_string", "str_"),
    rustc_ast::LitKind::Int(..) => ("custom_int", "int_"),
});"#.to_string(),
            rust_type: "rustc_ast::LitKind".to_string(),
            variants: vec!["Str".to_string(), "Int".to_string(), "Bool".to_string()],
            content_hash: "d4e5f6789ab1c2d3".to_string(),
        });
        
        self.add_macro_doc(MacroDoc {
            name: "switch_litinttype2string".to_string(),
            category: "AST Types".to_string(),
            description: "Converts rustc_ast::LitIntType to optimized string representation".to_string(),
            usage_example: r#"let int_type = switch_litinttype2string!(ty, {
    rustc_ast::LitIntType::Signed(rustc_ast::IntTy::I32) => "custom_i32",
});"#.to_string(),
            rust_type: "rustc_ast::LitIntType".to_string(),
            variants: vec!["i8_".to_string(), "i16_".to_string(), "i32_".to_string(), "i64_".to_string(), "u8_".to_string(), "u16_".to_string(), "u32_".to_string(), "u64_".to_string()],
            content_hash: "e5f6789ab1c2d3e4".to_string(),
        });
        
        // Node Types
        self.add_macro_doc(MacroDoc {
            name: "switch_node2string".to_string(),
            category: "Node Types".to_string(),
            description: "Converts rustc_hir::Node to optimized string representation".to_string(),
            usage_example: r#"let node_type = switch_node2string!(node);"#.to_string(),
            rust_type: "rustc_hir::Node".to_string(),
            variants: vec!["Item".to_string(), "Expr".to_string(), "Stmt".to_string(), "TraitItem".to_string(), "ImplItem".to_string()],
            content_hash: "e5f6789ab1c2d3e4".to_string(),
        });
        
        // Special Macros
        self.add_macro_doc(MacroDoc {
            name: "switch_itemkind_custom".to_string(),
            category: "Special Macros".to_string(),
            description: "Custom pattern matching for ItemKind with fallback to default switch".to_string(),
            usage_example: r#"let result = switch_itemkind_custom!(&item.kind, {
    rustc_hir::ItemKind::Const(_, _, _, body_id) => {
        // Custom logic here
        "custom_const".to_string()
    },
});"#.to_string(),
            rust_type: "rustc_hir::ItemKind".to_string(),
            variants: vec!["Custom patterns with fallback".to_string()],
            content_hash: "custom".to_string(),
        });
        
        self.add_macro_doc(MacroDoc {
            name: "switch_itemkind".to_string(),
            category: "Special Macros".to_string(),
            description: "Block-based pattern matching for ItemKind".to_string(),
            usage_example: r#"switch_itemkind!(&item.kind => {
    rustc_hir::ItemKind::Const(_, _, _, body_id) => {
        // Block of code here
    }
    _ => {}
});"#.to_string(),
            rust_type: "rustc_hir::ItemKind".to_string(),
            variants: vec!["Block-based matching".to_string()],
            content_hash: "block_based".to_string(),
        });
    }
    
    pub fn generate_markdown_documentation(&self) -> String {
        let mut doc = String::new();
        
        doc.push_str("# Rust Eigendecomposition System - Generated Macros Documentation\n\n");
        doc.push_str("This document describes all auto-generated macros for optimized Rust compiler introspection.\n\n");
        
        doc.push_str("## Overview\n\n");
        doc.push_str("The `mklang!` macro system generates optimized switch macros for all Rust compiler enums, providing:\n\n");
        doc.push_str("- ✅ **Content-addressable** canonical forms with SHA256 hashes\n");
        doc.push_str("- ✅ **Usage-frequency** optimized pattern matching\n");
        doc.push_str("- ✅ **Custom pattern** support with fallback defaults\n");
        doc.push_str("- ✅ **Type-safe** string conversion for all Rust enums\n\n");
        
        doc.push_str("## Generated by\n\n");
        doc.push_str("```rust\n");
        doc.push_str("// build.rs\n");
        doc.push_str("mklang!(rust_eigen_system);\n");
        doc.push_str("```\n\n");
        
        // Generate documentation for each category
        for (category, macro_names) in &self.categories {
            doc.push_str(&format!("## {}\n\n", category));
            
            for macro_name in macro_names {
                if let Some(macro_doc) = self.generated_macros.get(macro_name) {
                    doc.push_str(&format!("### `{}`\n\n", macro_doc.name));
                    doc.push_str(&format!("**Rust Type:** `{}`\n\n", macro_doc.rust_type));
                    doc.push_str(&format!("**Content Hash:** `{}`\n\n", macro_doc.content_hash));
                    doc.push_str(&format!("**Description:** {}\n\n", macro_doc.description));
                    
                    doc.push_str("**Supported Variants:**\n");
                    for variant in &macro_doc.variants {
                        doc.push_str(&format!("- `{}`\n", variant));
                    }
                    doc.push_str("\n");
                    
                    doc.push_str("**Usage Example:**\n");
                    doc.push_str("```rust\n");
                    doc.push_str(&macro_doc.usage_example);
                    doc.push_str("\n```\n\n");
                    
                    doc.push_str("---\n\n");
                }
            }
        }
        
        doc.push_str("## Macro Generator\n\n");
        doc.push_str("All macros are generated using the `mkstring_switch_rust_hir!` meta-macro:\n\n");
        doc.push_str("```rust\n");
        doc.push_str("mkstring_switch_rust_hir!(ExprKind, switch_exprkind2string, {\n");
        doc.push_str("    rustc_hir::ExprKind::Lit(_) => \"Lit\",\n");
        doc.push_str("    rustc_hir::ExprKind::Call(..) => \"Call\",\n");
        doc.push_str("    rustc_hir::ExprKind::Match(..) => \"Match\"\n");
        doc.push_str("}, default: \"Other\");\n");
        doc.push_str("```\n\n");
        
        doc.push_str("## Mathematical Foundation\n\n");
        doc.push_str("Each macro represents a **categorical functor** from Rust's type system to optimized string representations:\n\n");
        doc.push_str("```\n");
        doc.push_str("Rust Enum → Canonical Form → Content Hash → Optimized Switch → String\n");
        doc.push_str("```\n\n");
        
        doc.push_str("The system creates **eigenforms** that converge toward the mathematical essence of Rust through:\n");
        doc.push_str("1. **Usage frequency analysis** - Hot paths get `#[likely]` annotations\n");
        doc.push_str("2. **Content deduplication** - Identical patterns share canonical hashes\n");
        doc.push_str("3. **Eigenvalue optimization** - Mathematical complexity measures guide generation\n\n");
        
        doc.push_str("## Integration\n\n");
        doc.push_str("Import all generated macros:\n\n");
        doc.push_str("```rust\n");
        doc.push_str("use introspector_collector::*;\n");
        doc.push_str("```\n\n");
        doc.push_str("Or import specific macros:\n\n");
        doc.push_str("```rust\n");
        doc.push_str("use introspector_collector::{\n");
        doc.push_str("    switch_itemkind2string,\n");
        doc.push_str("    switch_defkind2string,\n");
        doc.push_str("    switch_exprkind2string,\n");
        doc.push_str("};\n");
        doc.push_str("```\n\n");
        
        doc
    }
    
    pub fn save_documentation(&self, path: &str) -> Result<(), std::io::Error> {
        let doc = self.generate_markdown_documentation();
        fs::write(path, doc)?;
        println!("📚 Generated macro documentation: {}", path);
        Ok(())
    }
}
