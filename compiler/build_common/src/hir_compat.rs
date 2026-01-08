// HIR API compatibility shim - maps old .hir() methods to new direct TyCtxt methods

use rustc_middle::ty::TyCtxt;
use rustc_hir::def_id::LocalDefId;
use rustc_hir::{HirId, BodyId, Body};

/// Compatibility wrapper that provides the old .hir() API
pub struct HirCompat<'tcx> {
    tcx: TyCtxt<'tcx>,
}

impl<'tcx> HirCompat<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>) -> Self {
        Self { tcx }
    }

    // Old: tcx.hir().maybe_body_owned_by(def_id)
    // New: tcx.hir().maybe_body_owned_by(def_id) - still exists
    pub fn maybe_body_owned_by(&self, def_id: LocalDefId) -> Option<BodyId> {
        self.tcx.hir().maybe_body_owned_by(def_id)
    }

    // Old: tcx.hir().body(body_id)  
    // New: tcx.hir_body(body_id)
    pub fn body(&self, body_id: BodyId) -> &'tcx Body<'tcx> {
        self.tcx.hir_body(body_id)
    }

    // Old: tcx.hir().node(hir_id)
    // New: tcx.hir_node(hir_id)  
    pub fn node(&self, hir_id: HirId) -> rustc_hir::Node<'tcx> {
        self.tcx.hir_node(hir_id)
    }

    // Old: tcx.hir().expect_item(def_id)
    // New: tcx.hir_expect_item(def_id)
    pub fn expect_item(&self, def_id: LocalDefId) -> &'tcx rustc_hir::Item<'tcx> {
        self.tcx.hir_expect_item(def_id)
    }

    // Old: tcx.hir().span(hir_id)
    // New: tcx.hir_span(hir_id) 
    pub fn span(&self, hir_id: HirId) -> rustc_span::Span {
        self.tcx.hir_span(hir_id)
    }

    // Old: tcx.hir().crate_items()
    // New: tcx.hir_crate_items()
    pub fn crate_items(&self) -> rustc_middle::hir::ModuleItems {
        self.tcx.hir_crate_items(())
    }
}

/// Extension trait to add .hir() method back to TyCtxt
pub trait TyCtxtHirExt<'tcx> {
    fn hir(&self) -> HirCompat<'tcx>;
}

impl<'tcx> TyCtxtHirExt<'tcx> for TyCtxt<'tcx> {
    fn hir(&self) -> HirCompat<'tcx> {
        HirCompat::new(*self)
    }
}

// Compatibility macros for common patterns
#[macro_export]
macro_rules! hir_compat {
    // tcx.hir().maybe_body_owned_by(def_id)
    ($tcx:expr, maybe_body_owned_by, $def_id:expr) => {
        $tcx.hir().maybe_body_owned_by($def_id)
    };
    
    // tcx.hir().body(body_id)
    ($tcx:expr, body, $body_id:expr) => {
        $tcx.hir_body($body_id)
    };
    
    // tcx.hir().node(hir_id)
    ($tcx:expr, node, $hir_id:expr) => {
        $tcx.hir_node($hir_id)
    };
    
    // tcx.hir().expect_item(def_id)
    ($tcx:expr, expect_item, $def_id:expr) => {
        $tcx.hir_expect_item($def_id)
    };
    
    // tcx.hir().span(hir_id)
    ($tcx:expr, span, $hir_id:expr) => {
        $tcx.hir_span($hir_id)
    };
    
    // tcx.hir().crate_items()
    ($tcx:expr, crate_items) => {
        $tcx.hir_crate_items(())
    };
}
