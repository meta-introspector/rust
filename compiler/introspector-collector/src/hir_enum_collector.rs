use rustc_hir as hir;
use rustc_hir::intravisit::{self, Visitor};
use rustc_middle::ty::TyCtxt;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub discriminant: Option<i128>,
}

#[derive(Debug, Clone)]
pub struct RustcEnum {
    pub name: String,
    pub crate_name: String,
    pub module_path: String,
    pub variants: Vec<EnumVariant>,
    pub usage_count: u32,
}

pub struct HirEnumCollector<'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub enums: Vec<RustcEnum>,
    pub current_module: Vec<String>,
}

impl<'tcx> HirEnumCollector<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>) -> Self {
        Self {
            tcx,
            enums: Vec::new(),
            current_module: Vec::new(),
        }
    }

    pub fn collect_all_enums(&mut self) {
        let crate_items = self.tcx.hir_crate_items(());
        for item_id in crate_items.free_items() {
            let item = self.tcx.hir_item(item_id);
            self.extract_enum_info(item);
        }
    }

    fn extract_enum_info(&mut self, item: &hir::Item<'_>) {
        if let hir::ItemKind::Enum(enum_def, generics, _) = &item.kind {
            let crate_name = self.tcx.crate_name(rustc_hir::def_id::LOCAL_CRATE).to_string();
            let module_path = self.current_module.join("::");
            
            let mut variants = Vec::new();
            for variant in enum_def.variants.iter() {
                let variant_name = variant.ident.name.to_string();
                
                // Try to get discriminant value
                let discriminant = if let Some(ref disc) = variant.disr_expr {
                    // For now, we'll just mark that it has a discriminant
                    // Getting the actual value requires more complex evaluation
                    None
                } else {
                    None
                };
                
                variants.push(EnumVariant {
                    name: variant_name,
                    discriminant,
                });
            }
            
            self.enums.push(RustcEnum {
                name: self.tcx.item_name(item.owner_id.to_def_id()).to_string(),
                crate_name,
                module_path,
                variants,
                usage_count: 1,
            });
        }
    }

    pub fn generate_macros(&self) -> String {
        let mut output = String::new();
        
        output.push_str("//! Generated enum-to-string macros from HIR analysis\n");
        output.push_str("//! Auto-generated from rustc HIR traversal\n\n");
        
        // Group by crate
        let mut by_crate: HashMap<String, Vec<&RustcEnum>> = HashMap::new();
        for enum_info in &self.enums {
            by_crate.entry(enum_info.crate_name.clone())
                .or_insert_with(Vec::new)
                .push(enum_info);
        }
        
        for (crate_name, enums) in by_crate {
            output.push_str(&format!("// === {} ===\n", crate_name));
            
            for enum_info in enums {
                output.push_str(&self.generate_enum_macro(enum_info));
                output.push('\n');
            }
            output.push('\n');
        }
        
        // Generate summary
        output.push_str(&self.generate_summary());
        
        output
    }

    fn generate_enum_macro(&self, enum_info: &RustcEnum) -> String {
        let macro_name = format!("mk_{}_to_string", 
            enum_info.name.to_lowercase().replace("::", "_"));
        
        let mut macro_body = format!(
            "/// Convert {} enum to string\n",
            enum_info.name
        );
        macro_body.push_str(&format!("/// Crate: {}\n", enum_info.crate_name));
        macro_body.push_str(&format!("/// Module: {}\n", enum_info.module_path));
        macro_body.push_str(&format!("/// Variants: {}\n", enum_info.variants.len()));
        macro_body.push_str(&format!("macro_rules! {} {{\n", macro_name));
        
        for variant in &enum_info.variants {
            let full_path = if enum_info.module_path.is_empty() {
                format!("{}::{}", enum_info.name, variant.name)
            } else {
                format!("{}::{}::{}", enum_info.module_path, enum_info.name, variant.name)
            };
            
            macro_body.push_str(&format!(
                "    ({}) => {{ \"{}\" }};\n",
                full_path, variant.name
            ));
        }
        
        macro_body.push_str("}\n");
        macro_body
    }

    fn generate_summary(&self) -> String {
        let mut summary = String::new();
        
        summary.push_str("// === HIR ENUM SUMMARY ===\n");
        summary.push_str(&format!("// Total enums: {}\n", self.enums.len()));
        
        let mut by_size: HashMap<usize, u32> = HashMap::new();
        let mut total_variants = 0;
        
        for enum_info in &self.enums {
            let size = enum_info.variants.len();
            *by_size.entry(size).or_insert(0) += 1;
            total_variants += size;
        }
        
        summary.push_str(&format!("// Total variants: {}\n", total_variants));
        summary.push_str("// Size distribution:\n");
        
        let mut sizes: Vec<_> = by_size.keys().collect();
        sizes.sort();
        
        for size in sizes {
            let count = by_size[size];
            summary.push_str(&format!("//   {} variants: {} enums\n", size, count));
        }
        
        summary
    }

    pub fn generate_orbit_analysis(&self) -> String {
        let mut analysis = String::new();
        
        analysis.push_str("// === HIR ORBIT ANALYSIS ===\n");
        analysis.push_str("// Enums grouped by variant count (orbit size)\n\n");
        
        let mut orbits: HashMap<usize, Vec<&RustcEnum>> = HashMap::new();
        for enum_info in &self.enums {
            orbits.entry(enum_info.variants.len())
                .or_insert_with(Vec::new)
                .push(enum_info);
        }
        
        let mut orbit_sizes: Vec<_> = orbits.keys().collect();
        orbit_sizes.sort();
        
        for &size in &orbit_sizes {
            let enums_in_orbit = &orbits[&size];
            analysis.push_str(&format!("// Orbit size {}: {} enums\n", size, enums_in_orbit.len()));
            
            for enum_info in enums_in_orbit.iter().take(5) {
                analysis.push_str(&format!("//   {}::{} ({} variants)\n", 
                    enum_info.crate_name, enum_info.name, enum_info.variants.len()));
                
                // Show first few variants
                for variant in enum_info.variants.iter().take(3) {
                    analysis.push_str(&format!("//     - {}\n", variant.name));
                }
                if enum_info.variants.len() > 3 {
                    analysis.push_str(&format!("//     ... and {} more variants\n", 
                        enum_info.variants.len() - 3));
                }
            }
            
            if enums_in_orbit.len() > 5 {
                analysis.push_str(&format!("//   ... and {} more enums\n", enums_in_orbit.len() - 5));
            }
            analysis.push('\n');
        }
        
        analysis
    }
}

impl<'tcx> Visitor<'tcx> for HirEnumCollector<'tcx> {
    type NestedFilter = rustc_middle::hir::nested_filter::All;

    fn visit_item(&mut self, item: &'tcx hir::Item<'tcx>) {
        // Track module path
        if let hir::ItemKind::Mod(..) = item.kind {
            self.current_module.push(self.tcx.item_name(item.owner_id.to_def_id()).to_string());
            intravisit::walk_item(self, item);
            self.current_module.pop();
        } else {
            self.extract_enum_info(item);
            intravisit::walk_item(self, item);
        }
    }
}
