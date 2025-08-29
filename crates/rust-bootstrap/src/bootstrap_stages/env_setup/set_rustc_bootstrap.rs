use std::env;

pub fn set_rustc_bootstrap() {
    env::set_var("RUSTC_BOOTSTRAP", "1");
}