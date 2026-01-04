
// Auto-generated mkrust! macro definitions
// Each enum becomes a macro argument for selective language construction

#[macro_export]
macro_rules! mkrust {
    // Privacy flags
    (mk-priv-flag!, enum = $enum:ident, size = $size:expr) => {
        #[cfg(feature = "privacy")]
        pub enum $enum {
            // Generated variants based on size
        }
        
        #[cfg(not(feature = "privacy"))]
        pub enum $enum {
            Public, // Collapsed to single variant
        }
    };
    
    // Level enums
    (mk-level!, enum = $enum:ident, size = $size:expr) => {
        #[cfg(feature = "levels")]
        pub enum $enum {
            // Size-based level variants
        }
    };
    
    // Kind enums  
    (mk-kind!, enum = $enum:ident, size = $size:expr) => {
        #[cfg(feature = "kinds")]
        pub enum $enum {
            // Type kind variants
        }
    };
    
    // Generic enum constructor
    ($macro_name:ident, enum_size = $size:expr, variants = [$($variant:ident),*], strings = [$($string:literal),*], usage_count = $count:expr) => {
        compile_time_assert!($size == count_variants!($($variant),*));
        
        pub enum GeneratedEnum {
            $($variant),*
        }
        
        impl GeneratedEnum {
            pub fn to_string(&self) -> &'static str {
                match self {
                    $(Self::$variant => $string),*
                }
            }
        }
    };
}

// Helper macros
macro_rules! count_variants {
    () => { 0 };
    ($head:ident $(, $tail:ident)*) => { 1 + count_variants!($($tail),*) };
}

macro_rules! compile_time_assert {
    ($condition:expr) => {
        const _: () = assert!($condition);
    };
}

