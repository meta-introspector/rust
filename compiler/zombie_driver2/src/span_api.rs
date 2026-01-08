// Minimal span API test case
use rustc_span::Span;

#[repr(C)]
pub struct SpanRef {
    pub raw: *mut u8,
}

pub trait SpanOps {
    fn to_ref(&self) -> SpanRef;
    fn from_ref(r: SpanRef) -> Option<Self> where Self: Sized;
}

impl SpanOps for Span {
    fn to_ref(&self) -> SpanRef {
        SpanRef { raw: self as *const _ as *mut u8 }
    }
    
    fn from_ref(r: SpanRef) -> Option<Self> {
        unsafe { Some(*(r.raw as *const Span)) }
    }
}

#[repr(u32)]
pub enum SpanCmd {
    ToRef,
    FromRef,
}

pub struct SpanApi;

impl SpanApi {
    pub fn execute(cmd: SpanCmd, span: Span) -> SpanRef {
        match cmd {
            SpanCmd::ToRef => span.to_ref(),
            SpanCmd::FromRef => SpanRef { raw: std::ptr::null_mut() },
        }
    }
}
