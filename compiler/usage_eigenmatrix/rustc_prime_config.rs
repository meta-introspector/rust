// Auto-generated Rustc Prime Component Configuration

#[cfg(feature = "prime_2")]
pub mod algebraic {
    // Prime 2 component: C(opt|none|ok|err)+
    // DefIds: ["core::option::Option", "core::result::Result"]
    pub const ENABLED: bool = true;
}

#[cfg(not(feature = "prime_2"))]
pub mod algebraic {
    pub const ENABLED: bool = false;
}

#[cfg(feature = "prime_13")]
pub mod memory {
    // Prime 13 component: C(alloc|box|rc)*
    // DefIds: ["alloc::boxed::Box", "alloc::rc::Rc"]
    pub const ENABLED: bool = true;
}

#[cfg(not(feature = "prime_13"))]
pub mod memory {
    pub const ENABLED: bool = false;
}

#[cfg(feature = "prime_11")]
pub mod presentation {
    // Prime 11 component: C(fmt|display)+
    // DefIds: ["core::fmt::Display", "core::fmt::Debug"]
    pub const ENABLED: bool = true;
}

#[cfg(not(feature = "prime_11"))]
pub mod presentation {
    pub const ENABLED: bool = false;
}

#[cfg(feature = "prime_3")]
pub mod control {
    // Prime 3 component: C(op|flow)*
    // DefIds: ["core::ops::ControlFlow", "core::ops::Try"]
    pub const ENABLED: bool = true;
}

#[cfg(not(feature = "prime_3"))]
pub mod control {
    pub const ENABLED: bool = false;
}

#[cfg(feature = "prime_17")]
pub mod concurrency {
    // Prime 17 component: C(sync|async|future)*
    // DefIds: ["core::future::Future", "std::sync::Mutex"]
    pub const ENABLED: bool = true;
}

#[cfg(not(feature = "prime_17"))]
pub mod concurrency {
    pub const ENABLED: bool = false;
}

#[cfg(feature = "prime_5")]
pub mod relational {
    // Prime 5 component: C(eq|ord|cmp)*
    // DefIds: ["core::cmp::PartialEq", "core::cmp::PartialOrd"]
    pub const ENABLED: bool = true;
}

#[cfg(not(feature = "prime_5"))]
pub mod relational {
    pub const ENABLED: bool = false;
}

#[cfg(feature = "prime_19")]
pub mod collections {
    // Prime 19 component: C(vec|map|set)+
    // DefIds: ["alloc::vec::Vec", "std::collections::HashMap"]
    pub const ENABLED: bool = true;
}

#[cfg(not(feature = "prime_19"))]
pub mod collections {
    pub const ENABLED: bool = false;
}

#[cfg(feature = "prime_7")]
pub mod recursive {
    // Prime 7 component: C(it|iter)+
    // DefIds: ["core::iter::Iterator", "core::iter::IntoIterator"]
    pub const ENABLED: bool = true;
}

#[cfg(not(feature = "prime_7"))]
pub mod recursive {
    pub const ENABLED: bool = false;
}

