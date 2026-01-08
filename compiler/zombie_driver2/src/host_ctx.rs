// Host context for mapping between rustc types and ABI wrappers

use crate::api_v1::*;
use rustc_middle::ty::{Ty, TyCtxt};
use rustc_hir::def_id::DefId;
use rustc_span::Span;
use rustc_session::Session;
use std::collections::HashMap;

pub struct HostCtx<'tcx> {
    pub tcx: TyCtxt<'tcx>,
    ty_store: HashMap<usize, Ty<'tcx>>,
    def_store: HashMap<usize, DefId>,
    span_store: HashMap<usize, Span>,
    next_id: usize,
}

impl<'tcx> HostCtx<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>) -> Self {
        Self {
            tcx,
            ty_store: HashMap::new(),
            def_store: HashMap::new(),
            span_store: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn to_ty_ref(&mut self, ty: Ty<'tcx>) -> TyRef {
        let id = self.next_id;
        self.next_id += 1;
        self.ty_store.insert(id, ty);
        TyRef { raw: id as *mut _ }
    }

    pub fn from_ty_ref(&self, r: TyRef) -> Option<Ty<'tcx>> {
        let id = r.raw as usize;
        self.ty_store.get(&id).copied()
    }

    pub fn to_def_ref(&mut self, def: DefId) -> DefIdRef {
        let id = self.next_id;
        self.next_id += 1;
        self.def_store.insert(id, def);
        DefIdRef { raw: id as *mut _ }
    }

    pub fn from_def_ref(&self, r: DefIdRef) -> Option<DefId> {
        let id = r.raw as usize;
        self.def_store.get(&id).copied()
    }

    pub fn to_span_ref(&mut self, span: Span) -> SpanRef {
        let id = self.next_id;
        self.next_id += 1;
        self.span_store.insert(id, span);
        SpanRef { raw: id as *mut _ }
    }

    pub fn from_span_ref(&self, r: SpanRef) -> Option<Span> {
        let id = r.raw as usize;
        self.span_store.get(&id).copied()
    }
}
