// Wrapper for rustc_hir crate - only this module uses rustc_hir
use rustc_hir::def_id::DefId;

#[repr(C)]
pub struct DefIdRef { pub raw: *mut u8 }

#[repr(u32)]
pub enum HirCmd { ToRef, FromRef }

pub trait HirOps {
    fn to_ref(&self) -> DefIdRef;
}

impl HirOps for DefId {
    fn to_ref(&self) -> DefIdRef {
        DefIdRef { raw: self as *const _ as *mut u8 }
    }
}

// C ABI interface for .so loading
#[no_mangle]
pub extern "C" fn hir_execute_c(cmd: u32, data: *const u8) -> DefIdRef {
    let cmd = match cmd {
        0 => HirCmd::ToRef,
        1 => HirCmd::FromRef,
        _ => return DefIdRef { raw: std::ptr::null_mut() },
    };
    hir_execute(cmd, data)
}

pub fn hir_execute(cmd: HirCmd, data: *const u8) -> DefIdRef {
    match cmd {
        HirCmd::ToRef => unsafe { (*(data as *const DefId)).to_ref() },
        HirCmd::FromRef => DefIdRef { raw: std::ptr::null_mut() },
    }
}
