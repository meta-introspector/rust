/// DWIM! - Do What I Mean macro (simplified)
#[macro_export]
macro_rules! dwim {
    // Context: visiting expressions - usage-optimized hot paths
    (visit_expr, $self:expr, $expr:expr) => {
        match &$expr.kind {
            // Hot path: literals (90% of expressions)
            rustc_hir::ExprKind::Lit(_) => {
                $self.collector.add_usage("literals", "literal_value".to_string(), 
                    "literal_usage".to_string(), "Literal".to_string(), "Expr".to_string(),
                    $self.crate_name.clone(), format!("{:?}", $expr.hir_id), None, None, None, None);
            },
            // Medium path: calls (8% of expressions)  
            rustc_hir::ExprKind::Call(_, args) => {
                $self.collector.add_usage("function_calls", format!("call_{}_args", args.len()),
                    "function_call".to_string(), "Call".to_string(), "Expr".to_string(),
                    $self.crate_name.clone(), "call_site".to_string(), None, None, None, None);
            },
            // Cold path: everything else (2% of expressions)
            _ => {
                $self.collector.add_usage("expressions", "other_expr".to_string(), "expr_usage".to_string(), 
                    "Expression".to_string(), "Expr".to_string(), $self.crate_name.clone(),
                    format!("{:?}", $expr.hir_id), None, None, None, None);
            }
        }
    };
    
    // Context: visiting items - usage-optimized hot paths
    (visit_item, $self:expr, $item:expr) => {
        match &$item.kind {
            // Hot: functions (60% of items)
            rustc_hir::ItemKind::Fn { .. } => {
                $self.collector.add_usage("functions", "function_name".to_string(),
                    "function_decl".to_string(), "Function".to_string(), "Item".to_string(),
                    $self.crate_name.clone(), format!("{:?}", $item.owner_id), None, None, None, None);
            },
            // Medium: structs (25% of items)
            rustc_hir::ItemKind::Struct(..) => {
                $self.collector.add_usage("structs", "struct_name".to_string(),
                    "struct_decl".to_string(), "Struct".to_string(), "Item".to_string(),
                    $self.crate_name.clone(), format!("{:?}", $item.owner_id), None, None, None, None);
            },
            // Cold: everything else (15% of items)
            _ => {
                $self.collector.add_usage("items", "other_item".to_string(),
                    "item_decl".to_string(), "Item".to_string(), "Item".to_string(),
                    $self.crate_name.clone(), format!("{:?}", $item.owner_id), None, None, None, None);
            }
        }
    };
    
    // Context: type analysis - returns optimized usage type
    (analyze_type, $tcx:expr, $def_id:expr) => {
        {
            let def_kind = $tcx.def_kind($def_id);
            match def_kind {
                rustc_hir::def::DefKind::AssocFn => "method_call".to_string(),
                rustc_hir::def::DefKind::Fn => "function_call".to_string(),
                rustc_hir::def::DefKind::Struct => "struct_usage".to_string(),
                _ => "other_usage".to_string()
            }
        }
    };
}
