// Wrapper for rustc_span crate - only this module uses rustc_span
use rustc_span::Span;

#[repr(C)]
pub struct SpanRef { pub raw: *mut u8 }

#[repr(u32)]
pub enum SpanCmd { ToRef, FromRef }

pub trait SpanOps {
    fn to_ref(&self) -> SpanRef;
}

impl SpanOps for Span {
    fn to_ref(&self) -> SpanRef {
        SpanRef { raw: self as *const _ as *mut u8 }
    }
}

// C ABI interface for .so loading
#[no_mangle]
pub extern "C" fn span_execute_c(cmd: u32, data: *const u8) -> SpanRef {
    let cmd = match cmd {
        0 => SpanCmd::ToRef,
        1 => SpanCmd::FromRef,
        _ => return SpanRef { raw: std::ptr::null_mut() },
    };
    span_execute(cmd, data)
}

pub fn span_execute(cmd: SpanCmd, data: *const u8) -> SpanRef {
    match cmd {
        SpanCmd::ToRef => unsafe { (*(data as *const Span)).to_ref() },
        SpanCmd::FromRef => SpanRef { raw: std::ptr::null_mut() },
    }
}
