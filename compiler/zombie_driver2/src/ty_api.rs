// Minimal ty API test case
use rustc_middle::ty::{Ty, TyCtxt};

#[repr(C)]
pub struct TyRef {
    pub raw: *mut u8,
}

pub trait TyOps<'tcx> {
    fn to_ref(&self) -> TyRef;
    fn from_ref(r: TyRef) -> Option<Self> where Self: Sized;
}

impl<'tcx> TyOps<'tcx> for Ty<'tcx> {
    fn to_ref(&self) -> TyRef {
        TyRef { raw: self as *const _ as *mut u8 }
    }
    
    fn from_ref(r: TyRef) -> Option<Self> {
        unsafe { Some(*(r.raw as *const Ty)) }
    }
}

#[repr(u32)]
pub enum TyCmd {
    ToRef,
    FromRef,
}

pub struct TyApi;

impl TyApi {
    pub fn execute<'tcx>(cmd: TyCmd, ty: Ty<'tcx>) -> TyRef {
        match cmd {
            TyCmd::ToRef => ty.to_ref(),
            TyCmd::FromRef => TyRef { raw: std::ptr::null_mut() },
        }
    }
}
