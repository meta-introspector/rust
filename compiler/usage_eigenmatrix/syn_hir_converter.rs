// Auto-generated converter from bijection proof
impl SynToHir {
    fn convert_type(syn: syn::Type) -> hir::TyKind {
        match syn {
            syn::Type::Array => hir::TyKind::Array,
            syn::Type::Path => hir::TyKind::Path,
            syn::Type::Reference => hir::TyKind::Ref,
        }
    }
    fn convert_expr(syn: syn::Expr) -> hir::ExprKind {
        match syn {
            syn::Expr::Binary => hir::ExprKind::Binary,
            syn::Expr::Call => hir::ExprKind::Call,
        }
    }
}
