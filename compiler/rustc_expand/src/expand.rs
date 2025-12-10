use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::{iter, mem, slice};

use rustc_ast::mut_visit::*;
use rustc_ast::tokenstream::TokenStream;
use rustc_ast::visit::{self, AssocCtxt, Visitor, VisitorResult, try_visit, walk_list};
use rustc_ast::{
    self as ast, AssocItemKind, AstNodeWrapper, AttrArgs, AttrStyle, AttrVec, DUMMY_NODE_ID,
    ExprKind, ForeignItemKind, HasAttrs, HasNodeId, Inline, ItemKind, MacStmtStyle, MetaItemInner,
    MetaItemKind, ModKind, NodeId, PatKind, StmtKind, TyKind, token,
};
use rustc_ast_pretty::pprust;
use rustc_attr_parsing::{AttributeParser, Early, EvalConfigResult, ShouldEmit, validate_attr};
use rustc_data_structures::flat_map_in_place::FlatMapInPlace;
use rustc_data_structures::stack::ensure_sufficient_stack;
use rustc_errors::PResult;
use rustc_feature::Features;
use rustc_hir::Target;
use rustc_hir::def::MacroKinds;
use rustc_hir::limit::Limit;
use rustc_parse::parser::{
    AttemptLocalParseRecovery, CommaRecoveryMode, ForceCollect, Parser, RecoverColon, RecoverComma,
    token_descr,
};
use rustc_session::Session;
use rustc_session::lint::builtin::{UNUSED_ATTRIBUTES, UNUSED_DOC_COMMENTS};
use rustc_session::parse::feature_err;
use rustc_span::hygiene::SyntaxContext;
use rustc_span::{ErrorGuaranteed, FileName, Ident, LocalExpnId, Span, Symbol, sym};
use smallvec::SmallVec;

mod ast_fragments_defs;
use ast_fragments_defs::{AstFragment, AstFragmentKind, AddSemicolon, InvocationCollectorNode, DummyAstNode};

use crate::base::*;
use crate::config::{StripUnconfigured, attr_into_trace};
use crate::errors::{
    EmptyDelegationMac, GlobDelegationOutsideImpls, GlobDelegationTraitlessQpath, IncompleteParse,
    RecursionLimitReached, RemoveExprNotSupported, RemoveNodeNotSupported, UnsupportedKeyValue,
    WrongFragmentKind,
};
use crate::fluent_generated;
use crate::mbe::diagnostics::annotate_err_with_kind;
use crate::module::{
    DirOwnership, ParsedExternalMod, mod_dir_path, mod_file_path_from_attr, parse_external_mod,
};
use crate::placeholders::{PlaceholderExpander, placeholder};
use crate::stats::*;




pub struct Invocation {
    pub kind: InvocationKind,
    pub fragment_kind: AstFragmentKind,
    pub expansion_data: ExpansionData,
}

pub enum InvocationKind {
    Bang {
        mac: Box<ast::MacCall>,
        span: Span,
    },
    Attr {
        attr: ast::Attribute,
        /// Re-insertion position for inert attributes.
        pos: usize,
        item: Annotatable,
        /// Required for resolving derive helper attributes.
        derives: Vec<ast::Path>,
    },
    Derive {
        path: ast::Path,
        is_const: bool,
        item: Annotatable,
    },
    GlobDelegation {
        item: Box<ast::AssocItem>,
        /// Whether this is a trait impl or an inherent impl
        of_trait: bool,
    },
}

impl InvocationKind {
    fn placeholder_visibility(&self) -> Option<ast::Visibility> {
        // HACK: For unnamed fields placeholders should have the same visibility as the actual
        // fields because for tuple structs/variants resolve determines visibilities of their
        // constructor using these field visibilities before attributes on them are expanded.
        // The assumption is that the attribute expansion cannot change field visibilities,
        // and it holds because only inert attributes are supported in this position.
        match self {
            InvocationKind::Attr { item: Annotatable::FieldDef(field), .. }
            | InvocationKind::Derive { item: Annotatable::FieldDef(field), .. }
                if field.ident.is_none() =>
            {
                Some(field.vis.clone())
            }
            _ => None,
        }
    }
}

impl Invocation {
    pub fn span(&self) -> Span {
        match &self.kind {
            InvocationKind::Bang { span, .. } => *span,
            InvocationKind::Attr { attr, .. } => attr.span,
            InvocationKind::Derive { path, .. } => path.span,
            InvocationKind::GlobDelegation { item, .. } => item.span,
        }
    }

    fn span_mut(&mut self) -> &mut Span {
        match &mut self.kind {
            InvocationKind::Bang { span, .. } => span,
            InvocationKind::Attr { attr, .. } => &mut attr.span,
            InvocationKind::Derive { path, .. } => &mut path.span,
            InvocationKind::GlobDelegation { item, .. } => &mut item.span,
        }
    }
}

pub struct MacroExpander<'a, 'b> {
    pub cx: &'a mut ExtCtxt<'b>,
    monotonic: bool, // cf. `cx.monotonic_expander()`
}

impl<'a, 'b> MacroExpander<'a, 'b> {
    pub fn new(cx: &'a mut ExtCtxt<'b>, monotonic: bool) -> Self {
        MacroExpander { cx, monotonic }
    }

    pub fn expand_crate(&mut self, krate: ast::Crate) -> ast::Crate {
        let file_path = match self.cx.source_map().span_to_filename(krate.spans.inner_span) {
            FileName::Real(name) => name
                .into_local_path()
                .expect("attempting to resolve a file path in an external file"),
            other => PathBuf::from(other.prefer_local().to_string()),
        };
        let dir_path = file_path.parent().unwrap_or(&file_path).to_owned();
        self.cx.root_path = dir_path.clone();
        self.cx.current_expansion.module = Rc::new(ModuleData {
            mod_path: vec![Ident::with_dummy_span(self.cx.ecfg.crate_name)],
            file_path_stack: vec![file_path],
            dir_path,
        });
        let krate = self.fully_expand_fragment(AstFragment::Crate(krate)).make_crate();
        assert_eq!(krate.id, ast::CRATE_NODE_ID);
        self.cx.trace_macros_diag();
        krate
    }

    /// Recursively expand all macro invocations in this AST fragment.
    pub fn fully_expand_fragment(&mut self, input_fragment: AstFragment) -> AstFragment {
        let orig_expansion_data = self.cx.current_expansion.clone();
        let orig_force_mode = self.cx.force_mode;

        // Collect all macro invocations and replace them with placeholders.
        let (mut fragment_with_placeholders, mut invocations) =
            self.collect_invocations(input_fragment, &[]);

        // Optimization: if we resolve all imports now,
        // we'll be able to immediately resolve most of imported macros.
        self.resolve_imports();

        // Resolve paths in all invocations and produce output expanded fragments for them, but
        // do not insert them into our input AST fragment yet, only store in `expanded_fragments`.
        // The output fragments also go through expansion recursively until no invocations are left.
        // Unresolved macros produce dummy outputs as a recovery measure.
        invocations.reverse();
        let mut expanded_fragments = Vec::new();
        let mut undetermined_invocations = Vec::new();
        let (mut progress, mut force) = (false, !self.monotonic);
        loop {
            let Some((invoc, ext)) = invocations.pop() else {
                self.resolve_imports();
                if undetermined_invocations.is_empty() {
                    break;
                }
                invocations = mem::take(&mut undetermined_invocations);
                force = !progress;
                progress = false;
                if force && self.monotonic {
                    self.cx.dcx().span_delayed_bug(
                        invocations.last().unwrap().0.span(),
                        "expansion entered force mode without producing any errors",
                    );
                }
                continue;
            };

            let ext = match ext {
                Some(ext) => ext,
                None => {
                    let eager_expansion_root = if self.monotonic {
                        invoc.expansion_data.id
                    } else {
                        orig_expansion_data.id
                    };
                    match self.cx.resolver.resolve_macro_invocation(
                        &invoc,
                        eager_expansion_root,
                        force,
                    ) {
                        Ok(ext) => ext,
                        Err(Indeterminate) => {
                            // Cannot resolve, will retry this invocation later.
                            undetermined_invocations.push((invoc, None));
                            continue;
                        }
                    }
                }
            };

            let ExpansionData { depth, id: expn_id, .. } = invoc.expansion_data;
            let depth = depth - orig_expansion_data.depth;
            self.cx.current_expansion = invoc.expansion_data.clone();
            self.cx.force_mode = force;

            let fragment_kind = invoc.fragment_kind;
            match self.expand_invoc(invoc, &ext.kind) {
                ExpandResult::Ready(fragment) => {
                    let mut derive_invocations = Vec::new();
                    let derive_placeholders = self
                        .cx
                        .resolver
                        .take_derive_resolutions(expn_id)
                        .map(|derives| {
                            derive_invocations.reserve(derives.len());
                            derives
                                .into_iter()
                                .map(|DeriveResolution { path, item, exts: _, is_const }| {
                                    // FIXME: Consider using the derive resolutions (`_exts`)
                                    // instead of enqueuing the derives to be resolved again later.
                                    // Note that this can result in duplicate diagnostics.
                                    let expn_id = LocalExpnId::fresh_empty();
                                    derive_invocations.push((
                                        Invocation {
                                            kind: InvocationKind::Derive { path, item, is_const },
                                            fragment_kind,
                                            expansion_data: ExpansionData {
                                                id: expn_id,
                                                ..self.cx.current_expansion.clone()
                                            },
                                        },
                                        None,
                                    ));
                                    NodeId::placeholder_from_expn_id(expn_id)
                                })
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();

                    let (expanded_fragment, collected_invocations) =
                        self.collect_invocations(fragment, &derive_placeholders);
                    // We choose to expand any derive invocations associated with this macro
                    // invocation *before* any macro invocations collected from the output
                    // fragment.
                    derive_invocations.extend(collected_invocations);

                    progress = true;
                    if expanded_fragments.len() < depth {
                        expanded_fragments.push(Vec::new());
                    }
                    expanded_fragments[depth - 1].push((expn_id, expanded_fragment));
                    invocations.extend(derive_invocations.into_iter().rev());
                }
                ExpandResult::Retry(invoc) => {
                    if force {
                        self.cx.dcx().span_bug(
                            invoc.span(),
                            "expansion entered force mode but is still stuck",
                        );
                    } else {
                        // Cannot expand, will retry this invocation later.
                        undetermined_invocations.push((invoc, Some(ext)));
                    }
                }
            }
        }

        self.cx.current_expansion = orig_expansion_data;
        self.cx.force_mode = orig_force_mode;

        // Finally incorporate all the expanded macros into the input AST fragment.
        let mut placeholder_expander = PlaceholderExpander::default();
        while let Some(expanded_fragments) = expanded_fragments.pop() {
            for (expn_id, expanded_fragment) in expanded_fragments.into_iter().rev() {
                placeholder_expander
                    .add(NodeId::placeholder_from_expn_id(expn_id), expanded_fragment);
            }
        }
        fragment_with_placeholders.mut_visit_with(&mut placeholder_expander);
        fragment_with_placeholders
    }

    fn resolve_imports(&mut self) {
        if self.monotonic {
            self.cx.resolver.resolve_imports();
        }
    }

    /// Collects all macro invocations reachable at this time in this AST fragment, and replace
    /// them with "placeholders" - dummy macro invocations with specially crafted `NodeId`s.
    /// Then call into resolver that builds a skeleton ("reduced graph") of the fragment and
    /// prepares data for resolving paths of macro invocations.
    fn collect_invocations(
        &mut self,
        mut fragment: AstFragment,
        extra_placeholders: &[NodeId],
    ) -> (AstFragment, Vec<(Invocation, Option<Arc<SyntaxExtension>>)>) {
        // Resolve `$crate`s in the fragment for pretty-printing.
        self.cx.resolver.resolve_dollar_crates();

        let mut invocations = {
            let mut collector = InvocationCollector {
                // Non-derive macro invocations cannot see the results of cfg expansion - they
                // will either be removed along with the item, or invoked before the cfg/cfg_attr
                // attribute is expanded. Therefore, we don't need to configure the tokens
                // Derive macros *can* see the results of cfg-expansion - they are handled
                // specially in `fully_expand_fragment`
                cx: self.cx,
                invocations: Vec::new(),
                monotonic: self.monotonic,
            };
            fragment.mut_visit_with(&mut collector);
            fragment.add_placeholders(extra_placeholders);
            collector.invocations
        };

        if self.monotonic {
            self.cx
                .resolver
                .visit_ast_fragment_with_placeholders(self.cx.current_expansion.id, &fragment);

            if self.cx.sess.opts.incremental.is_some() {
                for (invoc, _) in invocations.iter_mut() {
                    let expn_id = invoc.expansion_data.id;
                    let parent_def = self.cx.resolver.invocation_parent(expn_id);
                    let span = invoc.span_mut();
                    *span = span.with_parent(Some(parent_def));
                }
            }
        }

        (fragment, invocations)
    }

    fn error_recursion_limit_reached(&mut self) -> ErrorGuaranteed {
        let expn_data = self.cx.current_expansion.id.expn_data();
        let suggested_limit = match self.cx.ecfg.recursion_limit {
            Limit(0) => Limit(2),
            limit => limit * 2,
        };

        let guar = self.cx.dcx().emit_err(RecursionLimitReached {
            span: expn_data.call_site,
            descr: expn_data.kind.descr(),
            suggested_limit,
            crate_name: self.cx.ecfg.crate_name,
        });

        self.cx.macro_error_and_trace_macros_diag();
        guar
    }

    /// A macro's expansion does not fit in this fragment kind.
    /// For example, a non-type macro in a type position.
    fn error_wrong_fragment_kind(
        &mut self,
        kind: AstFragmentKind,
        mac: &ast::MacCall,
        span: Span,
    ) -> ErrorGuaranteed {
        let guar =
            self.cx.dcx().emit_err(WrongFragmentKind { span, kind: kind.name(), name: &mac.path });
        self.cx.macro_error_and_trace_macros_diag();
        guar
    }

    fn expand_invoc(
        &mut self,
        invoc: Invocation,
        ext: &SyntaxExtensionKind,
    ) -> ExpandResult<AstFragment, Invocation> {
        let recursion_limit = match self.cx.reduced_recursion_limit {
            Some((limit, _)) => limit,
            None => self.cx.ecfg.recursion_limit,
        };

        if !recursion_limit.value_within_limit(self.cx.current_expansion.depth) {
            let guar = match self.cx.reduced_recursion_limit {
                Some((_, guar)) => guar,
                None => self.error_recursion_limit_reached(),
            };

            // Reduce the recursion limit by half each time it triggers.
            self.cx.reduced_recursion_limit = Some((recursion_limit / 2, guar));

            return ExpandResult::Ready(invoc.fragment_kind.dummy(invoc.span(), guar));
        }

        let macro_stats = self.cx.sess.opts.unstable_opts.macro_stats;

        let (fragment_kind, span) = (invoc.fragment_kind, invoc.span());
        ExpandResult::Ready(match invoc.kind {
            InvocationKind::Bang { mac, span } => {
                if let SyntaxExtensionKind::Bang(expander) = ext {
                    match expander.expand(self.cx, span, mac.args.tokens.clone()) {
                        Ok(tok_result) => {
                            let fragment =
                                self.parse_ast_fragment(tok_result, fragment_kind, &mac.path, span);
                            if macro_stats {
                                update_bang_macro_stats(
                                    self.cx,
                                    fragment_kind,
                                    span,
                                    mac,
                                    &fragment,
                                );
                            }
                            fragment
                        }
                        Err(guar) => return ExpandResult::Ready(fragment_kind.dummy(span, guar)),
                    }
                } else if let Some(expander) = ext.as_legacy_bang() {
                    let tok_result = match expander.expand(self.cx, span, mac.args.tokens.clone()) {
                        ExpandResult::Ready(tok_result) => tok_result,
                        ExpandResult::Retry(_) => {
                            // retry the original
                            return ExpandResult::Retry(Invocation {
                                kind: InvocationKind::Bang { mac, span },
                                ..invoc
                            });
                        }
                    };
                    if let Some(fragment) = fragment_kind.make_from(tok_result) {
                        if macro_stats {
                            update_bang_macro_stats(self.cx, fragment_kind, span, mac, &fragment);
                        }
                        fragment
                    } else {
                        let guar = self.error_wrong_fragment_kind(fragment_kind, &mac, span);
                        fragment_kind.dummy(span, guar)
                    }
                } else {
                    unreachable!();
                }
            }
            InvocationKind::Attr { attr, pos, mut item, derives } => {
                if let Some(expander) = ext.as_attr() {
                    self.gate_proc_macro_input(&item);
                    self.gate_proc_macro_attr_item(span, &item);
                    let tokens = match &item {
                        // FIXME: Collect tokens and use them instead of generating
                        // fake ones. These are unstable, so it needs to be
                        // fixed prior to stabilization
                        // Fake tokens when we are invoking an inner attribute, and
                        // we are invoking it on an out-of-line module or crate.
                        Annotatable::Crate(krate) => {
                            rustc_parse::fake_token_stream_for_crate(&self.cx.sess.psess, krate)
                        }
                        Annotatable::Item(item_inner)
                            if matches!(attr.style, AttrStyle::Inner)
                                && matches!(
                                    item_inner.kind,
                                    ItemKind::Mod(
                                        _,
                                        _,
                                        ModKind::Unloaded
                                            | ModKind::Loaded(_, Inline::No { .. }, _),
                                    )
                                ) =>
                        {
                            rustc_parse::fake_token_stream_for_item(&self.cx.sess.psess, item_inner)
                        }
                        _ => item.to_tokens(),
                    };
                    let attr_item = attr.get_normal_item();
                    let safety = attr_item.unsafety;
                    if let AttrArgs::Eq { .. } = attr_item.args {
                        self.cx.dcx().emit_err(UnsupportedKeyValue { span });
                    }
                    let inner_tokens = attr_item.args.inner_tokens();
                    match expander.expand_with_safety(self.cx, safety, span, inner_tokens, tokens) {
                        Ok(tok_result) => {
                            let fragment = self.parse_ast_fragment(
                                tok_result,
                                fragment_kind,
                                &attr_item.path,
                                span,
                            );
                            if macro_stats {
                                update_attr_macro_stats(
                                    self.cx,
                                    fragment_kind,
                                    span,
                                    &attr_item.path,
                                    &attr,
                                    item,
                                    &fragment,
                                );
                            }
                            fragment
                        }
                        Err(guar) => return ExpandResult::Ready(fragment_kind.dummy(span, guar)),
                    }
                } else if let SyntaxExtensionKind::LegacyAttr(expander) = ext {
                    // `LegacyAttr` is only used for builtin attribute macros, which have their
                    // safety checked by `check_builtin_meta_item`, so we don't need to check
                    // `unsafety` here.
                    match validate_attr::parse_meta(&self.cx.sess.psess, &attr) {
                        Ok(meta) => {
                            let item_clone = macro_stats.then(|| item.clone());
                            let items = match expander.expand(self.cx, span, &meta, item, false) {
                                ExpandResult::Ready(items) => items,
                                ExpandResult::Retry(item) => {
                                    // Reassemble the original invocation for retrying.
                                    return ExpandResult::Retry(Invocation {
                                        kind: InvocationKind::Attr { attr, pos, item, derives },
                                        ..invoc
                                    });
                                }
                            };
                            if matches!(
                                fragment_kind,
                                AstFragmentKind::Expr | AstFragmentKind::MethodReceiverExpr
                            ) && items.is_empty()
                            {
                                let guar = self.cx.dcx().emit_err(RemoveExprNotSupported { span });
                                fragment_kind.dummy(span, guar)
                            } else {
                                let fragment = fragment_kind.expect_from_annotatables(items);
                                if macro_stats {
                                    update_attr_macro_stats(
                                        self.cx,
                                        fragment_kind,
                                        span,
                                        &meta.path,
                                        &attr,
                                        item_clone.unwrap(),
                                        &fragment,
                                    );
                                }
                                fragment
                            }
                        }
                        Err(err) => {
                            let _guar = err.emit();
                            fragment_kind.expect_from_annotatables(iter::once(item))
                        }
                    }
                } else if let SyntaxExtensionKind::NonMacroAttr = ext {
                    // `-Zmacro-stats` ignores these because they don't do any real expansion.
                    self.cx.expanded_inert_attrs.mark(&attr);
                    item.visit_attrs(|attrs| attrs.insert(pos, attr));
                    fragment_kind.expect_from_annotatables(iter::once(item))
                } else {
                    unreachable!();
                }
            }
            InvocationKind::Derive { path, item, is_const } => match ext {
                SyntaxExtensionKind::Derive(expander)
                | SyntaxExtensionKind::LegacyDerive(expander) => {
                    if let SyntaxExtensionKind::Derive(..) = ext {
                        self.gate_proc_macro_input(&item);
                    }
                    // The `MetaItem` representing the trait to derive can't
                    // have an unsafe around it (as of now).
                    let meta = ast::MetaItem {
                        unsafety: ast::Safety::Default,
                        kind: MetaItemKind::Word,
                        span,
                        path,
                    };
                    let items = match expander.expand(self.cx, span, &meta, item, is_const) {
                        ExpandResult::Ready(items) => items,
                        ExpandResult::Retry(item) => {
                            // Reassemble the original invocation for retrying.
                            return ExpandResult::Retry(Invocation {
                                kind: InvocationKind::Derive { path: meta.path, item, is_const },
                                ..invoc
                            });
                        }
                    };
                    let fragment = fragment_kind.expect_from_annotatables(items);
                    if macro_stats {
                        update_derive_macro_stats(
                            self.cx,
                            fragment_kind,
                            span,
                            &meta.path,
                            &fragment,
                        );
                    }
                    fragment
                }
                SyntaxExtensionKind::MacroRules(expander)
                    if expander.kinds().contains(MacroKinds::DERIVE) =>
                {
                    if is_const {
                        let guar = self
                            .cx
                            .dcx()
                            .span_err(span, "macro `derive` does not support const derives");
                        return ExpandResult::Ready(fragment_kind.dummy(span, guar));
                    }
                    let body = item.to_tokens();
                    match expander.expand_derive(self.cx, span, &body) {
                        Ok(tok_result) => {
                            let fragment =
                                self.parse_ast_fragment(tok_result, fragment_kind, &path, span);
                            if macro_stats {
                                update_derive_macro_stats(
                                    self.cx,
                                    fragment_kind,
                                    span,
                                    &path,
                                    &fragment,
                                );
                            }
                            fragment
                        }
                        Err(guar) => return ExpandResult::Ready(fragment_kind.dummy(span, guar)),
                    }
                }
                _ => unreachable!(),
            },
            InvocationKind::GlobDelegation { item, of_trait } => {
                let AssocItemKind::DelegationMac(deleg) = &item.kind else { unreachable!() };
                let suffixes = match ext {
                    SyntaxExtensionKind::GlobDelegation(expander) => match expander.expand(self.cx)
                    {
                        ExpandResult::Ready(suffixes) => suffixes,
                        ExpandResult::Retry(()) => {
                            // Reassemble the original invocation for retrying.
                            return ExpandResult::Retry(Invocation {
                                kind: InvocationKind::GlobDelegation { item, of_trait },
                                ..invoc
                            });
                        }
                    },
                    SyntaxExtensionKind::Bang(..) => {
                        let msg = "expanded a dummy glob delegation";
                        let guar = self.cx.dcx().span_delayed_bug(span, msg);
                        return ExpandResult::Ready(fragment_kind.dummy(span, guar));
                    }
                    _ => unreachable!(),
                };

                type Node = AstNodeWrapper<Box<ast::AssocItem>, ImplItemTag>;
                let single_delegations = build_single_delegations::<Node>(
                    self.cx, deleg, &item, &suffixes, item.span, true,
                );
                // `-Zmacro-stats` ignores these because they don't seem important.
                fragment_kind.expect_from_annotatables(single_delegations.map(|item| {
                    Annotatable::AssocItem(Box::new(item), AssocCtxt::Impl { of_trait })
                }))
            }
        })
    }

    #[allow(rustc::untranslatable_diagnostic)] // FIXME: make this translatable
    fn gate_proc_macro_attr_item(&self, span: Span, item: &Annotatable) {
        let kind = match item {
            Annotatable::Item(_)
            | Annotatable::AssocItem(..)
            | Annotatable::ForeignItem(_)
            | Annotatable::Crate(..) => return,
            Annotatable::Stmt(stmt) => {
                // Attributes are stable on item statements,
                // but unstable on all other kinds of statements
                if stmt.is_item() {
                    return;
                }
                "statements"
            }
            Annotatable::Expr(_) => "expressions",
            Annotatable::Arm(..)
            | Annotatable::ExprField(..)
            | Annotatable::PatField(..)
            | Annotatable::GenericParam(..)
            | Annotatable::Param(..)
            | Annotatable::FieldDef(..)
            | Annotatable::Variant(..)
            | Annotatable::WherePredicate(..) => panic!("unexpected annotatable"),
        };
        if self.cx.ecfg.features.proc_macro_hygiene() {
            return;
        }
        feature_err(
            &self.cx.sess,
            sym::proc_macro_hygiene,
            span,
            format!("custom attributes cannot be applied to {kind}"),
        )
        .emit();
    }

    fn gate_proc_macro_input(&self, annotatable: &Annotatable) {
        struct GateProcMacroInput<'a> {
            sess: &'a Session,
        }

        impl<'ast, 'a> Visitor<'ast> for GateProcMacroInput<'a> {
            fn visit_item(&mut self, item: &'ast ast::Item) {
                match &item.kind {
                    ItemKind::Mod(_, _, mod_kind)
                        if !matches!(mod_kind, ModKind::Loaded(_, Inline::Yes, _)) =>
                    {
                        feature_err(
                            self.sess,
                            sym::proc_macro_hygiene,
                            item.span,
                            fluent_generated::expand_file_modules_in_proc_macro_input_are_unstable,
                        )
                        .emit();
                    }
                    _ => {}
                }

                visit::walk_item(self, item);
            }
        }

        if !self.cx.ecfg.features.proc_macro_hygiene() {
            annotatable.visit_with(&mut GateProcMacroInput { sess: &self.cx.sess });
        }
    }

    fn parse_ast_fragment(
        &mut self,
        toks: TokenStream,
        kind: AstFragmentKind,
        path: &ast::Path,
        span: Span,
    ) -> AstFragment {
        let mut parser = self.cx.new_parser_from_tts(toks);
        match parse_ast_fragment(&mut parser, kind) {
            Ok(fragment) => {
                ensure_complete_parse(&parser, path, kind.name(), span);
                fragment
            }
            Err(mut err) => {
                if err.span.is_dummy() {
                    err.span(span);
                }
                annotate_err_with_kind(&mut err, kind, span);
                let guar = err.emit();
                self.cx.macro_error_and_trace_macros_diag();
                kind.dummy(span, guar)
            }
        }
    }
}

pub fn parse_ast_fragment<'a>(
    this: &mut Parser<'a>,
    kind: AstFragmentKind,
) -> PResult<'a, AstFragment> {
    Ok(match kind {
        AstFragmentKind::Items => {
            let mut items = SmallVec::new();
            while let Some(item) = this.parse_item(ForceCollect::No)? {
                items.push(item);
            }
            AstFragment::Items(items)
        }
        AstFragmentKind::TraitItems => {
            let mut items = SmallVec::new();
            while let Some(item) = this.parse_trait_item(ForceCollect::No)? {
                items.extend(item);
            }
            AstFragment::TraitItems(items)
        }
        AstFragmentKind::ImplItems => {
            let mut items = SmallVec::new();
            while let Some(item) = this.parse_impl_item(ForceCollect::No)? {
                items.extend(item);
            }
            AstFragment::ImplItems(items)
        }
        AstFragmentKind::TraitImplItems => {
            let mut items = SmallVec::new();
            while let Some(item) = this.parse_impl_item(ForceCollect::No)? {
                items.extend(item);
            }
            AstFragment::TraitImplItems(items)
        }
        AstFragmentKind::ForeignItems => {
            let mut items = SmallVec::new();
            while let Some(item) = this.parse_foreign_item(ForceCollect::No)? {
                items.extend(item);
            }
            AstFragment::ForeignItems(items)
        }
        AstFragmentKind::Stmts => {
            let mut stmts = SmallVec::new();
            // Won't make progress on a `}`.
            while this.token != token::Eof && this.token != token::CloseBrace {
                if let Some(stmt) = this.parse_full_stmt(AttemptLocalParseRecovery::Yes)? {
                    stmts.push(stmt);
                }
            }
            AstFragment::Stmts(stmts)
        }
        AstFragmentKind::Expr => AstFragment::Expr(this.parse_expr()?),
        AstFragmentKind::MethodReceiverExpr => AstFragment::MethodReceiverExpr(this.parse_expr()?),
        AstFragmentKind::OptExpr => {
            if this.token != token::Eof {
                AstFragment::OptExpr(Some(this.parse_expr()?))
            } else {
                AstFragment::OptExpr(None)
            }
        }
        AstFragmentKind::Ty => AstFragment::Ty(this.parse_ty()?),
        AstFragmentKind::Pat => AstFragment::Pat(Box::new(this.parse_pat_allow_top_guard(
            None,
            RecoverComma::No,
            RecoverColon::Yes,
            CommaRecoveryMode::LikelyTuple,
        )?)),
        AstFragmentKind::Crate => AstFragment::Crate(this.parse_crate_mod()?),
        AstFragmentKind::Arms
        | AstFragmentKind::ExprFields
        | AstFragmentKind::PatFields
        | AstFragmentKind::GenericParams
        | AstFragmentKind::Params
        | AstFragmentKind::FieldDefs
        | AstFragmentKind::Variants
        | AstFragmentKind::WherePredicates => panic!("unexpected AST fragment kind"),
    })
}

pub(crate) fn ensure_complete_parse<'a>(
    parser: &Parser<'a>,
    macro_path: &ast::Path,
    kind_name: &str,
    span: Span,
) {
    if parser.token != token::Eof {
        let descr = token_descr(&parser.token);
        // Avoid emitting backtrace info twice.
        let def_site_span = parser.token.span.with_ctxt(SyntaxContext::root());

        let semi_span = parser.psess.source_map().next_point(span);
        let add_semicolon = match &parser.psess.source_map().span_to_snippet(semi_span) {
            Ok(snippet) if &snippet[..] != ";" && kind_name == "expression" => {
                Some(span.shrink_to_hi())
            }
            _ => None,
        };

        let expands_to_match_arm = kind_name == "pattern" && parser.token == token::FatArrow;

        parser.dcx().emit_err(IncompleteParse {
            span: def_site_span,
            descr,
            label_span: span,
            macro_path,
            kind_name,
            expands_to_match_arm,
            add_semicolon,
        });
    }
}

/// Wraps a call to `walk_*` / `walk_flat_map_*`
/// for an AST node that supports attributes
/// (see the `Annotatable` enum)
/// This method assigns a `NodeId`, and sets that `NodeId`
/// as our current 'lint node id'. If a macro call is found
/// inside this AST node, we will use this AST node's `NodeId`
/// to emit lints associated with that macro (allowing
/// `#[allow]` / `#[deny]` to be applied close to
/// the macro invocation).
///
/// Do *not* call this for a macro AST node
/// (e.g. `ExprKind::MacCall`) - we cannot emit lints
/// at these AST nodes, since they are removed and
/// replaced with the result of macro expansion.
///
/// All other `NodeId`s are assigned by `visit_id`.
/// * `self` is the 'self' parameter for the current method,
/// * `id` is a mutable reference to the `NodeId` field
///    of the current AST node.
/// * `closure` is a closure that executes the
///   `walk_*` / `walk_flat_map_*` method
///   for the current AST node.
macro_rules! assign_id {
    ($self:ident, $id:expr, $closure:expr) => {{
        let old_id = $self.cx.current_expansion.lint_node_id;
        if $self.monotonic {
            debug_assert_eq!(*$id, ast::DUMMY_NODE_ID);
            let new_id = $self.cx.resolver.next_node_id();
            *$id = new_id;
            $self.cx.current_expansion.lint_node_id = new_id;
        }
        let ret = ($closure)();
        $self.cx.current_expansion.lint_node_id = old_id;
        ret
    }};
}


