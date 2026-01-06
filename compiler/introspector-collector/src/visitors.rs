#![feature(rustc_private)]

extern crate rustc_middle;
extern crate rustc_hir;
extern crate rustc_ast;

use crate::data_structures::*;
use rustc_middle::ty::TyCtxt;
use rustc_hir::intravisit::{self, Visitor};
use std::collections::HashMap;
use crate::usage_collector::UsageCollector;

pub struct LiteralVisitor<'a> {
    pub collector: &'a mut UsageCollector,
    pub context: String,
}

impl<'tcx> intravisit::Visitor<'tcx> for LiteralVisitor<'_> {
    fn visit_expr(&mut self, expr: &'tcx rustc_hir::Expr<'tcx>) {
        match &expr.kind {
            rustc_hir::ExprKind::Lit(lit) => {
                let (literal_value, type_prefix) = match &lit.node {
                    rustc_ast::LitKind::Bool(b) => (format!("{}", b), "b"),
                    rustc_ast::LitKind::Char(c) => (format!("'{}'", c), "c"),
                    rustc_ast::LitKind::Int(i, ty) => {
                        let prefix = match ty {
                            rustc_ast::LitIntType::Signed(rustc_ast::IntTy::I8) => "i8_",
                            rustc_ast::LitIntType::Signed(rustc_ast::IntTy::I16) => "i16_",
                            rustc_ast::LitIntType::Signed(rustc_ast::IntTy::I32) => "i32_",
                            rustc_ast::LitIntType::Signed(rustc_ast::IntTy::I64) => "i64_",
                            rustc_ast::LitIntType::Signed(rustc_ast::IntTy::I128) => "i128_",
                            rustc_ast::LitIntType::Signed(rustc_ast::IntTy::Isize) => "isize_",
                            rustc_ast::LitIntType::Unsigned(rustc_ast::UintTy::U8) => "u8_",
                            rustc_ast::LitIntType::Unsigned(rustc_ast::UintTy::U16) => "u16_",
                            rustc_ast::LitIntType::Unsigned(rustc_ast::UintTy::U32) => "u32_",
                            rustc_ast::LitIntType::Unsigned(rustc_ast::UintTy::U64) => "u64_",
                            rustc_ast::LitIntType::Unsigned(rustc_ast::UintTy::U128) => "u128_",
                            rustc_ast::LitIntType::Unsigned(rustc_ast::UintTy::Usize) => "usize_",
                            rustc_ast::LitIntType::Unsuffixed => "int_",
                        };
                        (format!("{}", i), prefix)
                    },
                    rustc_ast::LitKind::Float(f, ty) => {
                        let prefix = match ty {
                            rustc_ast::LitFloatType::Suffixed(rustc_ast::FloatTy::F32) => "f32_",
                            rustc_ast::LitFloatType::Suffixed(rustc_ast::FloatTy::F64) => "f64_",
                            rustc_ast::LitFloatType::Suffixed(rustc_ast::FloatTy::F16) => "f16_",
                            rustc_ast::LitFloatType::Suffixed(rustc_ast::FloatTy::F128) => "f128_",
                            rustc_ast::LitFloatType::Unsuffixed => "float_",
                        };
                        (format!("{}", f), prefix)
                    },
                    rustc_ast::LitKind::Str(s, _) => (format!("\"{}\"", s), "str_"),
                    rustc_ast::LitKind::Byte(b) => (format!("{}", b), "byte_"),
                    rustc_ast::LitKind::ByteStr(bytes, _) => (format!("b\"{}\"", String::from_utf8_lossy(bytes.as_byte_str())), "bstr_"),
                    _ => return,
                };
                
                self.collector.add_usage(
                    "literals", 
                    format!("{}{}", type_prefix, literal_value),
                    "literal".to_string(),
                    "NumericLiteral".to_string(), 
                    "Expression".to_string(), 
                    self.context.clone(),
                    literal_value,
                    None,
                    None,
                    None,
                    None,
                );
            }
            
            rustc_hir::ExprKind::Field(expr, field) => {
                let field_name = field.name.to_string();
                self.collector.add_usage(
                    "field_access",
                    format!("field_{}", field_name),
                    "field_access".to_string(),
                    "FieldAccess".to_string(),
                    "Expression".to_string(),
                    self.context.clone(),
                    field_name,
                    None,
                    None,
                    None,
                    None,
                );
                self.visit_expr(expr);
            }
            
            rustc_hir::ExprKind::Call(func, args) => {
                let arg_count = args.len();
                self.collector.add_usage(
                    "function_calls",
                    format!("call_{}_args", arg_count),
                    "function_call".to_string(),
                    "FunctionCall".to_string(),
                    "Expression".to_string(),
                    self.context.clone(),
                    format!("{}_args", arg_count),
                    None,
                    None,
                    None,
                    None,
                );
                self.visit_expr(func);
                for arg in args.iter() { self.visit_expr(arg); }
            }
            
            rustc_hir::ExprKind::AddrOf(_, mutability, expr) => {
                let ref_type = match mutability {
                    rustc_hir::Mutability::Mut => "mut_ref",
                    rustc_hir::Mutability::Not => "ref",
                };
                self.collector.add_usage(
                    "memory_safety",
                    format!("addr_{}", ref_type),
                    "memory_ref".to_string(),
                    "MemoryRef".to_string(),
                    "Expression".to_string(),
                    self.context.clone(),
                    ref_type.to_string(),
                    None,
                    None,
                    None,
                    None,
                );
                self.visit_expr(expr);
            }
            
            rustc_hir::ExprKind::Index(base, index, _) => {
                self.collector.add_usage(
                    "indexing",
                    "index_access".to_string(),
                    "index_access".to_string(),
                    "IndexAccess".to_string(),
                    "Expression".to_string(),
                    self.context.clone(),
                    "array_index".to_string(),
                    None,
                    None,
                    None,
                    None,
                );
                self.visit_expr(base);
                self.visit_expr(index);
            }
            
            rustc_hir::ExprKind::Match(expr, arms, _) => {
                let arm_count = arms.len();
                self.collector.add_usage(
                    "patterns",
                    format!("match_{}_arms", arm_count),
                    "pattern_match".to_string(),
                    "PatternMatch".to_string(),
                    "Expression".to_string(),
                    self.context.clone(),
                    format!("{}_arms", arm_count),
                    None,
                    None,
                    None,
                    None,
                );
                self.visit_expr(expr);
                for arm in arms.iter() {
                    if let Some(_guard) = &arm.guard { 
                        // TODO: Fix guard access
                    }
                    self.visit_expr(&arm.body);
                }
            }
            
            _ => {}
        }
        
        intravisit::walk_expr(self, expr);
    }
}

pub struct TypeUsageVisitor<'a, 'tcx> {
    pub collector: &'a mut UsageCollector,
    pub crate_name: String,
    pub tcx: TyCtxt<'tcx>,
}

impl<'tcx> intravisit::Visitor<'tcx> for TypeUsageVisitor<'_, 'tcx> {
    fn visit_ty(&mut self, ty: &'tcx rustc_hir::Ty<'tcx, rustc_hir::AmbigArg>) {
        match &ty.kind {
            rustc_hir::TyKind::Path(rustc_hir::QPath::Resolved(_, path)) => {
                if let Some(def_id) = path.res.opt_def_id() {
                    let def_kind = self.tcx.def_kind(def_id);
                    match def_kind {
                        rustc_hir::def::DefKind::Struct | 
                        rustc_hir::def::DefKind::Enum |
                        rustc_hir::def::DefKind::Union => {
                            let _symbol = self.tcx.item_name(def_id).to_string();
                            // Record type usage
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        intravisit::walk_ty(self, ty);
    }
}
