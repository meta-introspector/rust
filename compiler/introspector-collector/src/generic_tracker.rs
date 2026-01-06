use crate::usage_types::*;
use rustc_hir;

pub struct GenericTracker;

impl GenericTracker {
    /// Track generic parameters - HIGH IMPACT (16 occurrences found in analysis)
    pub fn track_generics<'tcx>(
        generics: &rustc_hir::Generics<'tcx>,
        item_name: &str,
        crate_name: &str,
        item_type: &str,
        add_usage_fn: &mut dyn FnMut(&str, String, String, String, String, String, String, Option<String>, Option<String>)
    ) {
        for param in generics.params {
            match param.kind {
                rustc_hir::GenericParamKind::Type { .. } => {
                    let param_name = param.name.ident().to_string();
                    add_usage_fn(
                        "generics",
                        format!("{}::<{}>", item_name, param_name),
                        "generic_param".to_string(),
                        "GenericParam".to_string(),
                        "Type".to_string(),
                        crate_name.to_string(),
                        format!("{}::{}", item_type, param_name),
                        Some(crate_name.to_string()),
                        None
                    );
                }
                rustc_hir::GenericParamKind::Lifetime { .. } => {
                    let lifetime_name = param.name.ident().to_string();
                    add_usage_fn(
                        "lifetimes",
                        format!("{}::{}", item_name, lifetime_name),
                        "lifetime_param".to_string(),
                        "LifetimeParam".to_string(),
                        "Lifetime".to_string(),
                        crate_name.to_string(),
                        lifetime_name,
                        Some(crate_name.to_string()),
                        None
                    );
                }
                rustc_hir::GenericParamKind::Const { .. } => {
                    let const_name = param.name.ident().to_string();
                    add_usage_fn(
                        "const_generics",
                        format!("{}::{}", item_name, const_name),
                        "const_param".to_string(),
                        "ConstParam".to_string(),
                        "Const".to_string(),
                        crate_name.to_string(),
                        const_name,
                        Some(crate_name.to_string()),
                        None
                    );
                }
            }
        }
        
        // Track where clauses for trait bounds
        if !generics.predicates.is_empty() {
            add_usage_fn(
                "trait_bounds",
                format!("{}_where_clause", item_name),
                "where_clause".to_string(),
                "WhereClause".to_string(),
                "TraitBound".to_string(),
                crate_name.to_string(),
                format!("where_clause_{}", generics.predicates.len()),
                Some(crate_name.to_string()),
                None
            );
        }
    }
}
