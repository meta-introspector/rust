// ABI wrapper macros to isolate rustc dependencies

#[macro_export]
macro_rules! wraprustc_types {
    (
        $(
            $(#[$meta:meta])*
            type $Name:ident => $Opaque:ident ;
        )*
    ) => {
        $(
            $(#[$meta])*
            #[repr(C)]
            pub struct $Name {
                pub raw: *mut $Opaque,
            }

            impl $Name {
                pub fn null() -> Self {
                    Self { raw: core::ptr::null_mut() }
                }
            }
        )*

        #[repr(u32)]
        pub enum WrappedKind {
            $(
                $Name,
            )*
        }
    }
}

#[macro_export]
macro_rules! wraprustc_traits {
    (
        $(
            type $Name:ident => $Opaque:ident ;
        )*
    ) => {
        $(
            pub trait $NameExt {
                fn kind(&self) -> WrappedKind;
            }

            impl $NameExt for $Name {
                fn kind(&self) -> WrappedKind {
                    WrappedKind::$Name
                }
            }
        )*
    }
}

#[macro_export]
macro_rules! wraprustc_host_vtable {
    (
        $(
            type $Name:ident => $Opaque:ident ;
        )*
    ) => {
        #[repr(C)]
        pub struct HostVTable {
            $(
                pub drop_$Name: unsafe extern "C" fn($Name),
            )*
        }
    }
}
