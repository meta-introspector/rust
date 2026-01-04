// Enhanced mkbuild! macro with feature filtering - this replaces the simple one in lib.rs
    () => {
        mkbuild!(features = full)
    };
    
    (features = $lang:ident) => {{
        // Standard build configuration
        println!("cargo::rustc-check-cfg=cfg(bootstrap)");
        println!("cargo::rustc-check-cfg=cfg(llvm_enzyme)");
        println!("cargo:rustc-env=CFG_RELEASE_CHANNEL=dev");
        println!("cargo:rustc-env=RUSTC_INSTALL_BINDIR=/usr/local/bin/");
        
        // Feature-specific configuration
        let matrix = match stringify!($lang) {
            "minimal" => FeatureMatrix::minimal(),
            "safe" => FeatureMatrix::safe_rust(),
            "full" => FeatureMatrix::full_rust(),
            _ => FeatureMatrix::full_rust(),
        };
        
        // Configure feature flags
        for flag in matrix.to_cfg_flags() {
            println!("cargo:rustc-cfg=feature_{}", flag);
        }
        
        // Generate source filtering
        generate_source_filters(&matrix);
    }};
    
    (level = $n:expr) => {{
        println!("cargo::rustc-check-cfg=cfg(bootstrap)");
        println!("cargo::rustc-check-cfg=cfg(llvm_enzyme)");
        println!("cargo:rustc-env=CFG_RELEASE_CHANNEL=dev");
        println!("cargo:rustc-env=RUSTC_INSTALL_BINDIR=/usr/local/bin/");
        
        let matrix = FeatureMatrix::level($n);
        for flag in matrix.to_cfg_flags() {
            println!("cargo:rustc-cfg=feature_{}", flag);
        }
        
        generate_source_filters(&matrix);
    }}

