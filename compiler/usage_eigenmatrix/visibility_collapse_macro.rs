
// Visibility collapse macro: define!(private, public)
macro_rules! define {
    (private, public) => {
        // Transform all private items to public
        macro_rules! use_vis_priv {
            ($item:item) => {
                // Automatically label code that uses private visibility
                #[cfg(feature = "visibility-collapse")]
                pub $item
                
                #[cfg(not(feature = "visibility-collapse"))]
                $item
            };
        }
    };
}

// Usage: Automatically bisect code into private/non-private
macro_rules! auto_label_visibility {
    ($($item:item)*) => {
        $(
            use_vis_priv!($item);
        )*
    };
}
