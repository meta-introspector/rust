// Minimal hir API test case
use rustc_hir::def_id::DefId;

#[repr(C)]
pub struct DefIdRef {
    pub raw: *mut u8,
}

pub trait HirOps {
    fn to_ref(&self) -> DefIdRef;
    fn from_ref(r: DefIdRef) -> Option<Self> where Self: Sized;
}

impl HirOps for DefId {
    fn to_ref(&self) -> DefIdRef {
        DefIdRef { raw: self as *const _ as *mut u8 }
    }
    
    fn from_ref(r: DefIdRef) -> Option<Self> {
        unsafe { Some(*(r.raw as *const DefId)) }
    }
}

#[repr(u32)]
pub enum HirCmd {
    ToRef,
    FromRef,
}

pub struct HirApi;

impl HirApi {
    pub fn execute(cmd: HirCmd, def_id: DefId) -> DefIdRef {
        match cmd {
            HirCmd::ToRef => def_id.to_ref(),
            HirCmd::FromRef => DefIdRef { raw: std::ptr::null_mut() },
        }
    }
}
