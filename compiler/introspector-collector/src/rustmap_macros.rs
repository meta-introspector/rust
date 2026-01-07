/// Macro to map our types to Rust equivalents and generate lifting transformations
#[macro_export]
macro_rules! rustmap {
    // Map type as duplicate
    (duplicate $our_type:ty => $rust_type:path) => {
        impl From<$rust_type> for $our_type {
            fn from(rust_val: $rust_type) -> Self {
                // Direct isomorphism - types are equivalent
                unsafe { std::mem::transmute(rust_val) }
            }
        }
        
        impl Into<$rust_type> for $our_type {
            fn into(self) -> $rust_type {
                unsafe { std::mem::transmute(self) }
            }
        }
    };
    
    // Map type as equivalent with field lifting
    (equivalent $our_type:ty => $rust_type:path { $($field:ident),* }) => {
        impl From<$rust_type> for $our_type {
            fn from(rust_val: $rust_type) -> Self {
                Self {
                    $($field: rust_val.$field.into(),)*
                }
            }
        }
    };
    
    // Map function with projection
    (function $our_fn:ident => $rust_fn:path) => {
        pub fn $our_fn<T, R>(input: T) -> R 
        where 
            T: Into<$rust_fn::Input>,
            R: From<$rust_fn::Output>
        {
            let rust_input = input.into();
            let rust_output = $rust_fn(rust_input);
            rust_output.into()
        }
    };
    
    // Map enum with variant lifting
    (enum $our_enum:ty => $rust_enum:path { 
        $($variant:ident $(($($field:ty),*))? ),* 
    }) => {
        impl From<$rust_enum> for $our_enum {
            fn from(rust_val: $rust_enum) -> Self {
                match rust_val {
                    $(
                        $rust_enum::$variant $(($(ref $field),*))? => {
                            Self::$variant $(($((*$field).clone().into()),*))?
                        }
                    )*
                }
            }
        }
    };
}

/// Macro to generate visitor projections/weaves
#[macro_export]
macro_rules! visitor_weave {
    ($visitor_name:ident : $source_space:ty => $target_space:ty) => {
        pub struct $visitor_name {
            projection: Box<dyn Fn($source_space) -> $target_space>,
        }
        
        impl $visitor_name {
            pub fn new<F>(proj: F) -> Self 
            where F: Fn($source_space) -> $target_space + 'static 
            {
                Self {
                    projection: Box::new(proj),
                }
            }
            
            pub fn weave(&self, source: $source_space) -> $target_space {
                (self.projection)(source)
            }
        }
    };
}

/// Generate metaprogram for lifting structures
#[macro_export]
macro_rules! lift_structure {
    ($rust_struct:path => $our_struct:ident {
        $($field:ident: $field_type:ty),*
    }) => {
        #[derive(Debug, Clone)]
        pub struct $our_struct {
            $(pub $field: $field_type,)*
        }
        
        // rustmap!(equivalent $our_struct => $rust_struct { $($field),* });
        
        impl $our_struct {
            pub fn from_rust_projection(_rust_val: $rust_struct) -> Self {
                // rust_val.into()
                Self {
                    $($field: Default::default(),)*
                }
            }
            
            pub fn eigenform(&self) -> EigenForm<$our_struct> {
                EigenForm::new(self.clone())
            }
        }
    };
}
