// Compatibility shim layer - maps new API to existing code automatically
// This preserves all existing usage patterns without modification

#[macro_export]
macro_rules! mkshim {
    // Auto-detect and map to existing mkbuild usage
    (build_common::mkbuild) => {
        mkbuild!()
    };
    
    (build_common::mkbuild()) => {
        mkbuild!()
    };
    
    // Map any crate usage automatically
    ($crate_name:ident::mkbuild) => {
        mkbuild!()
    };
    
    ($crate_name:ident::mkbuild()) => {
        mkbuild!()
    };
    
    // Direct usage mapping
    (mkbuild) => {
        mkbuild!()
    };
    
    (mkbuild()) => {
        mkbuild!()
    };
}

// Re-export the original mkbuild for compatibility
pub use crate::mkbuild;

// Auto-shim any missing imports
#[macro_export]
macro_rules! auto_use_mkbuild {
    () => {
        use build_common::mkbuild;
    };
}

// Automatic compatibility layer - just include this and everything works
#[macro_export]
macro_rules! mkcompat {
    () => {
        // Automatically provide mkbuild if not found
        #[allow(unused_imports)]
        use build_common::mkbuild;
        
        // Provide shim layer
        macro_rules! __mkbuild_shim {
            () => { mkbuild!() };
            ($($args:tt)*) => { mkbuild!($($args)*) };
        }
        
        // Make it available as mkbuild if needed
        #[allow(unused_macros)]
        macro_rules! mkbuild {
            () => { $crate::mkbuild!() };
            ($($args:tt)*) => { $crate::mkbuild!($($args)*) };
        }
    };
}
