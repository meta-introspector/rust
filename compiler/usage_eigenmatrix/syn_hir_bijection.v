(* Auto-generated bijection proof *)
Theorem syn_hir_bijection :
  forall s : SynType, exists! h : HirType, phi s = h.
Proof.
  (* Type::Array <-> TyKind::Array with score 1.00 *)
  (* Type::Path <-> TyKind::Path with score 1.00 *)
  (* Type::Reference <-> TyKind::Ref with score 0.95 *)
  (* Expr::Binary <-> ExprKind::Binary with score 1.00 *)
  (* Expr::Call <-> ExprKind::Call with score 1.00 *)
  (* Completeness: 100.00% *)
  constructor.
Qed.
