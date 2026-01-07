use crate::rustmap_macros::*;
use crate::{lift_structure, visitor_weave};
use std::collections::HashMap;

/// Eigenform wrapper for mathematical analysis
#[derive(Debug, Clone)]
pub struct EigenForm<T> {
    pub value: T,
    pub eigenvalues: Vec<f64>,
    pub complexity: f64,
}

impl<T> EigenForm<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            eigenvalues: vec![1.0], // Default eigenvalue
            complexity: 1.0,
        }
    }
}

/// Our mathematical forms lifted from Rust
lift_structure!(rustc_hir::Item => OurItem {
    name: String,
    kind: String,
    complexity: f64
});

lift_structure!(rustc_hir::Expr => OurExpr {
    kind: String,
    usage_pattern: String,
    eigenvalue: f64
});

lift_structure!(rustc_middle::ty::Ty => OurType {
    name: String,
    category: String,
    dimension: usize
});

/// Visitor as projection/weave
visitor_weave!(HIRProjection: rustc_hir::Item => OurItem);
visitor_weave!(ExprWeave: rustc_hir::Expr => OurExpr);
visitor_weave!(TypeWeave: rustc_middle::ty::Ty => OurType);

/// Metaprogram generator for automatic lifting
pub struct MetaprogramGenerator {
    mappings: HashMap<String, String>,
    projections: Vec<String>,
}

impl MetaprogramGenerator {
    pub fn new() -> Self {
        Self {
            mappings: HashMap::new(),
            projections: Vec::new(),
        }
    }
    
    pub fn generate_rustmaps(&mut self, typespace_mappings: &[crate::typespace_mapper::TypespaceMapping]) {
        println!("=== GENERATING RUSTMAP! MACROS ===");
        
        for mapping in typespace_mappings {
            if mapping.similarity_score > 0.8 {
                // High similarity - generate duplicate mapping
                self.generate_duplicate_map(mapping);
            } else if mapping.similarity_score > 0.5 {
                // Medium similarity - generate equivalent mapping with field lifting
                self.generate_equivalent_map(mapping);
            } else {
                // Low similarity - novel type, generate projection
                self.generate_projection_map(mapping);
            }
        }
    }
    
    fn generate_duplicate_map(&mut self, mapping: &crate::typespace_mapper::TypespaceMapping) {
        let rust_match = &mapping.rust_matches[0]; // Best match
        
        let macro_code = format!(
            "rustmap!(duplicate {} => {});",
            mapping.collector_element,
            rust_match.rust_element
        );
        
        println!("DUPLICATE: {}", macro_code);
        self.mappings.insert(mapping.collector_element.clone(), macro_code);
    }
    
    fn generate_equivalent_map(&mut self, mapping: &crate::typespace_mapper::TypespaceMapping) {
        let rust_match = &mapping.rust_matches[0];
        
        let macro_code = format!(
            "rustmap!(equivalent {} => {} {{ /* fields to be analyzed */ }});",
            mapping.collector_element,
            rust_match.rust_element
        );
        
        println!("EQUIVALENT: {}", macro_code);
        self.mappings.insert(mapping.collector_element.clone(), macro_code);
    }
    
    fn generate_projection_map(&mut self, mapping: &crate::typespace_mapper::TypespaceMapping) {
        let projection_code = format!(
            "visitor_weave!({}Projection: RustType => {});",
            mapping.collector_element,
            mapping.collector_element
        );
        
        println!("PROJECTION: {}", projection_code);
        self.projections.push(projection_code);
    }
    
    pub fn save_generated_code(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut code = String::new();
        
        code.push_str("// Auto-generated rustmap! macros\n");
        code.push_str("use crate::rustmap_macros::*;\n\n");
        
        for (_, mapping_code) in &self.mappings {
            code.push_str(&mapping_code);
            code.push_str("\n");
        }
        
        code.push_str("\n// Auto-generated projections/weaves\n");
        for projection in &self.projections {
            code.push_str(&projection);
            code.push_str("\n");
        }
        
        std::fs::write(path, code)?;
        println!("Generated metaprogram saved to: {}", path);
        Ok(())
    }
}
