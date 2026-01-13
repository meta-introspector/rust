// Split-Decls Layer: Interface
// IO Signature: trait → impl
// Generated from: ./nix/vendor/rust/cargo2nix/submodules/split-decls-genesis

// function: wrap_item
fn wrap_item (item : & Item) -> proc_macro2 :: TokenStream { match item { Item :: Mod (item_mod) => { let mod_name = & item_mod . ident ; let content = if let Some ((_ , items)) = & item_mod . content { let wrapped_items : Vec < _ > = items . iter () . map (wrap_item) . collect () ; quote ! { # (# wrapped_items) * } } else { quote ! { } } ; quote ! { mkmod ! (# mod_name , { # content }) } } Item :: Use (item_use) => { quote ! { mkuse ! (# item_use) } } Item :: Fn (item_fn) => { quote ! { mkitem ! (mkfn ! (# item_fn)) } } Item :: Struct (item_struct) => { quote ! { mkitem ! (mkstruct ! (# item_struct)) } } Item :: Enum (item_enum) => { quote ! { mkitem ! (mkenum ! (# item_enum)) } } Item :: Trait (item_trait) => { quote ! { mkitem ! (mktrait ! (# item_trait)) } } Item :: Impl (item_impl) => { quote ! { mkitem ! (mkimpl ! (# item_impl)) } } _ => quote ! { mkitem ! (# item) } } }

