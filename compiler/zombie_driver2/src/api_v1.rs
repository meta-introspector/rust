// API registry - single source of truth for all wrapped types

use crate::abi_wrapper::*;

// Opaque marker types
pub struct HirTy;
pub struct HirDefId;
pub struct HirItem;
pub struct HirSpan;
pub struct HirSession;

wraprustc_types! {
    /// Wrapper for Ty<'tcx>
    type TyRef => HirTy;
    /// Wrapper for DefId
    type DefIdRef => HirDefId;
    /// Wrapper for Item<'tcx>
    type ItemRef => HirItem;
    /// Wrapper for Span
    type SpanRef => HirSpan;
    /// Wrapper for Session
    type SessionRef => HirSession;
}

wraprustc_traits! {
    type TyRef => HirTy;
    type DefIdRef => HirDefId;
    type ItemRef => HirItem;
    type SpanRef => HirSpan;
    type SessionRef => HirSession;
}

wraprustc_host_vtable! {
    type TyRef => HirTy;
    type DefIdRef => HirDefId;
    type ItemRef => HirItem;
    type SpanRef => HirSpan;
    type SessionRef => HirSession;
}

pub const ABI_VERSION: u64 = 1;
