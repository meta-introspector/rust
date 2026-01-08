// Auto-generated source filters
#[cfg(not(feature_visibility))]
macro_rules! zap_visibility {
    (pub $item:item) => { $item };
    ($item:item) => { $item };
}

#[cfg(not(feature_generics))]
macro_rules! zap_generics {
    ($name:ident<$($gen:tt)*>) => { $name };
}

#[cfg(not(feature_lifetimes))]
macro_rules! zap_lifetimes {
    ($name:ident<$lt:lifetime>) => { $name };
}

#[cfg(not(feature_traits))]
macro_rules! zap_traits {
    (impl $trait:path for $type:ty { $($body:tt)* }) => {};
}

#[cfg(not(feature_async))]
macro_rules! zap_async {
    (async fn $name:ident($($args:tt)*) -> $ret:ty { $($body:tt)* }) => {
        fn $name($($args)*) -> $ret { $($body)* }
    };
}

#[cfg(not(feature_unsafe))]
macro_rules! zap_unsafe {
    (unsafe $item:item) => { $item };
}

#[cfg(not(feature_macros))]
macro_rules! zap_macros {
    (macro_rules! $name:ident { $($body:tt)* }) => {};
}
