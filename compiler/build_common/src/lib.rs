#[macro_export]
macro_rules! mkbuild {
    () => {
        println!("cargo::rustc-check-cfg=cfg(bootstrap)");
        println!("cargo::rustc-check-cfg=cfg(llvm_enzyme)");
        println!("cargo:rustc-env=CFG_RELEASE_CHANNEL=dev");
        println!("cargo:rustc-env=RUSTC_INSTALL_BINDIR=/usr/local/bin/");
    };
}

pub mod core_lang;
