// Wrapper for rustc_middle crate - only this module uses rustc_middle
use rustc_middle::ty::{Ty, TyCtxt};

#[repr(C)]
pub struct TyRef { pub raw: *mut u8 }

#[repr(u32)]
pub enum TyCmd { ToRef, FromRef }

pub trait TyOps<'tcx> {
    fn to_ref(&self) -> TyRef;
}

impl<'tcx> TyOps<'tcx> for Ty<'tcx> {
    fn to_ref(&self) -> TyRef {
        TyRef { raw: self as *const _ as *mut u8 }
    }
}

// C ABI interface for .so loading
#[no_mangle]
pub extern "C" fn ty_execute_c(cmd: u32, data: *const u8) -> TyRef {
    let cmd = match cmd {
        0 => TyCmd::ToRef,
        1 => TyCmd::FromRef,
        _ => return TyRef { raw: std::ptr::null_mut() },
    };
    ty_execute(cmd, data)
}

pub fn ty_execute(cmd: TyCmd, data: *const u8) -> TyRef {
    match cmd {
        TyCmd::ToRef => TyRef { raw: data as *mut u8 },
        TyCmd::FromRef => TyRef { raw: std::ptr::null_mut() },
    }
}
