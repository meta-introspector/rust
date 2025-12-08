impl Target {
    pub fn is_abi_supported(&self, abi: ExternAbi) -> bool {
        let abi_map = AbiMap::from_target(self);
        abi_map.canonize_abi(abi, false).is_mapped()
    }

    /// Minimum integer size in bits that this target can perform atomic
    /// operations on.
    pub fn min_atomic_width(&self) -> u64 {
        self.min_atomic_width.unwrap_or(8)
    }

    /// Maximum integer size in bits that this target can perform atomic
    /// operations on.
    pub fn max_atomic_width(&self) -> u64 {
        self.max_atomic_width.unwrap_or_else(|| self.pointer_width.into())
    }

    /// Check some basic consistency of the current target. For JSON targets we are less strict;
    /// some of these checks are more guidelines than strict rules.
    fn check_consistency(&self, kind: TargetKind) -> Result<(), String> {
        macro_rules! check {
            ($b:expr, $($msg:tt)*) => {
                if !$b {
                    return Err(format!($($msg)*));
                }
            }
        }
        macro_rules! check_eq {
            ($left:expr, $right:expr, $($msg:tt)*) => {
                if ($left) != ($right) {
                    return Err(format!($($msg)*));
                }
            }
        }
        macro_rules! check_ne {
            ($left:expr, $right:expr, $($msg:tt)*) => {
                if ($left) == ($right) {
                    return Err(format!($($msg)*));
                }
            }
        }
        macro_rules! check_matches {
            ($left:expr, $right:pat, $($msg:tt)*) => {
                if !matches!($left, $right) {
                    return Err(format!($($msg)*));
                }
            }
        }

        check_eq!(
            self.is_like_darwin,
            self.vendor == "apple",
            "`is_like_darwin` must be set if and only if `vendor` is `apple`"
        );
        check_eq!(
            self.is_like_solaris,
            matches!(self.os, Os::Solaris | Os::Illumos),
            "`is_like_solaris` must be set if and only if `os` is `solaris` or `illumos`"
        );
        check_eq!(
            self.is_like_gpu,
            self.arch == Arch::Nvptx64 || self.arch == Arch::AmdGpu,
            "`is_like_gpu` must be set if and only if `target` is `nvptx64` or `amdgcn`"
        );
        check_eq!(
            self.is_like_windows,
            matches!(self.os, Os::Windows | Os::Uefi | Os::Cygwin),
            "`is_like_windows` must be set if and only if `os` is `windows`, `uefi` or `cygwin`"
        );
        check_eq!(
            self.is_like_wasm,
            matches!(self.arch, Arch::Wasm32 | Arch::Wasm64),
            "`is_like_wasm` must be set if and only if `arch` is `wasm32` or `wasm64`"
        );
        if self.is_like_msvc {
            check!(self.is_like_windows, "if `is_like_msvc` is set, `is_like_windows` must be set");
        }
        if self.os == Os::Emscripten {
            check!(self.is_like_wasm, "the `emcscripten` os only makes sense on wasm-like targets");
        }

        // Check that default linker flavor is compatible with some other key properties.
        check_eq!(
            self.is_like_darwin,
            matches!(self.linker_flavor, LinkerFlavor::Darwin(..)),
            "`linker_flavor` must be `darwin` if and only if `is_like_darwin` is set"
        );
        check_eq!(
            self.is_like_msvc,
            matches!(self.linker_flavor, LinkerFlavor::Msvc(..)),
            "`linker_flavor` must be `msvc` if and only if `is_like_msvc` is set"
        );
        check_eq!(
            self.is_like_wasm && self.os != Os::Emscripten,
            matches!(self.linker_flavor, LinkerFlavor::WasmLld(..)),
            "`linker_flavor` must be `wasm-lld` if and only if `is_like_wasm` is set and the `os` is not `emscripten`",
        );
        check_eq!(
            self.os == Os::Emscripten,
            matches!(self.linker_flavor, LinkerFlavor::EmCc),
            "`linker_flavor` must be `em-cc` if and only if `os` is `emscripten`"
        );
        check_eq!(
            self.arch == Arch::Bpf,
            matches!(self.linker_flavor, LinkerFlavor::Bpf),
            "`linker_flavor` must be `bpf` if and only if `arch` is `bpf`"
        );
        check_eq!(
            self.arch == Arch::Nvptx64,
            matches!(self.linker_flavor, LinkerFlavor::Ptx),
            "`linker_flavor` must be `ptc` if and only if `arch` is `nvptx64`"
        );

        for args in [
            &self.pre_link_args,
            &self.late_link_args,
            &self.late_link_args_dynamic,
            &self.late_link_args_static,
            &self.post_link_args,
        ] {
            for (&flavor, flavor_args) in args {
                check!(
                    !flavor_args.is_empty() || self.arch == Arch::Avr,
                    "linker flavor args must not be empty"
                );
                // Check that flavors mentioned in link args are compatible with the default flavor.
                match self.linker_flavor {
                    LinkerFlavor::Gnu(..) => {
                        check_matches!(
                            flavor,
                            LinkerFlavor::Gnu(..),
                            "mixing GNU and non-GNU linker flavors"
                        );
                    }
                    LinkerFlavor::Darwin(..) => {
                        check_matches!(
                            flavor,
                            LinkerFlavor::Darwin(..),
                            "mixing Darwin and non-Darwin linker flavors"
                        )
                    }
                    LinkerFlavor::WasmLld(..) => {
                        check_matches!(
                            flavor,
                            LinkerFlavor::WasmLld(..),
                            "mixing wasm and non-wasm linker flavors"
                        )
                    }
                    LinkerFlavor::Unix(..) => {
                        check_matches!(
                            flavor,
                            LinkerFlavor::Unix(..),
                            "mixing unix and non-unix linker flavors"
                        );
                    }
                    LinkerFlavor::Msvc(..) => {
                        check_matches!(
                            flavor,
                            LinkerFlavor::Msvc(..),
                            "mixing MSVC and non-MSVC linker flavors"
                        );
                    }
                    LinkerFlavor::EmCc
                    | LinkerFlavor::Bpf
                    | LinkerFlavor::Ptx
                    | LinkerFlavor::Llbc => {
                        check_eq!(flavor, self.linker_flavor, "mixing different linker flavors")
                    }
                }

                // Check that link args for cc and non-cc versions of flavors are consistent.
                let check_noncc = |noncc_flavor| -> Result<(), String> {
                    if let Some(noncc_args) = args.get(&noncc_flavor) {
                        for arg in flavor_args {
                            if let Some(suffix) = arg.strip_prefix("-Wl,") {
                                check!(
                                    noncc_args.iter().any(|a| a == suffix),
                                    " link args for cc and non-cc versions of flavors are not consistent"
                                );
                            }
                        }
                    }
                    Ok(())
                };

                match self.linker_flavor {
                    LinkerFlavor::Gnu(Cc::Yes, lld) => check_noncc(LinkerFlavor::Gnu(Cc::No, lld))?,
                    LinkerFlavor::WasmLld(Cc::Yes) => check_noncc(LinkerFlavor::WasmLld(Cc::No))?,
                    LinkerFlavor::Unix(Cc::Yes) => check_noncc(LinkerFlavor::Unix(Cc::No))?,
                    _ => {}
                }
            }

            // Check that link args for lld and non-lld versions of flavors are consistent.
            for cc in [Cc::No, Cc::Yes] {
                check_eq!(
                    args.get(&LinkerFlavor::Gnu(cc, Lld::No)),
                    args.get(&LinkerFlavor::Gnu(cc, Lld::Yes)),
                    "link args for lld and non-lld versions of flavors are not consistent",
                );
                check_eq!(
                    args.get(&LinkerFlavor::Darwin(cc, Lld::No)),
                    args.get(&LinkerFlavor::Darwin(cc, Lld::Yes)),
                    "link args for lld and non-lld versions of flavors are not consistent",
                );
            }
            check_eq!(
                args.get(&LinkerFlavor::Msvc(Lld::No)),
                args.get(&LinkerFlavor::Msvc(Lld::Yes)),
                "link args for lld and non-lld versions of flavors are not consistent",
            );
        }

        if self.link_self_contained.is_disabled() {
            check!(
                self.pre_link_objects_self_contained.is_empty()
                    && self.post_link_objects_self_contained.is_empty(),
                "if `link_self_contained` is disabled, then `pre_link_objects_self_contained` and `post_link_objects_self_contained` must be empty",
            );
        }

        // If your target really needs to deviate from the rules below,
        // except it and document the reasons.
        // Keep the default "unknown" vendor instead.
        check_ne!(self.vendor, "", "`vendor` cannot be empty");
        if let Os::Other(s) = &self.os {
            check!(!s.is_empty(), "`os` cannot be empty");
        }
        if !self.can_use_os_unknown() {
            // Keep the default "none" for bare metal targets instead.
            check_ne!(
                self.os,
                Os::Unknown,
                "`unknown` os can only be used on particular targets; use `none` for bare-metal targets"
            );
        }

        // Check dynamic linking stuff.
        // We skip this for JSON targets since otherwise, our default values would fail this test.
        // These checks are not critical for correctness, but more like default guidelines.
        // FIXME (https://github.com/rust-lang/rust/issues/133459): do we want to change the JSON
        // target defaults so that they pass these checks?
        if kind == TargetKind::Builtin {
            // BPF: when targeting user space vms (like rbpf), those can load dynamic libraries.
            // hexagon: when targeting QuRT, that OS can load dynamic libraries.
            // wasm{32,64}: dynamic linking is inherent in the definition of the VM.
            if self.os == Os::None
                && !matches!(self.arch, Arch::Bpf | Arch::Hexagon | Arch::Wasm32 | Arch::Wasm64)
            {
                check!(
                    !self.dynamic_linking,
                    "dynamic linking is not supported on this OS/architecture"
                );
            }
            if self.only_cdylib
                || self.crt_static_allows_dylibs
                || !self.late_link_args_dynamic.is_empty()
            {
                check!(
                    self.dynamic_linking,
                    "dynamic linking must be allowed when `only_cdylib` or `crt_static_allows_dylibs` or `late_link_args_dynamic` are set"
                );
            }
            // Apparently PIC was slow on wasm at some point, see comments in wasm_base.rs
            if self.dynamic_linking && !self.is_like_wasm {
                check_eq!(
                    self.relocation_model,
                    RelocModel::Pic,
                    "targets that support dynamic linking must use the `pic` relocation model"
                );
            }
            if self.position_independent_executables {
                check_eq!(
                    self.relocation_model,
                    RelocModel::Pic,
                    "targets that support position-independent executables must use the `pic` relocation model"
                );
            }
            // The UEFI targets do not support dynamic linking but still require PIC (#101377).
            if self.relocation_model == RelocModel::Pic && self.os != Os::Uefi {
                check!(
                    self.dynamic_linking || self.position_independent_executables,
                    "when the relocation model is `pic`, the target must support dynamic linking or use position-independent executables. \
                Set the relocation model to `static` to avoid this requirement"
                );
            }
            if self.static_position_independent_executables {
                check!(
                    self.position_independent_executables,
                    "if `static_position_independent_executables` is set, then `position_independent_executables` must be set"
                );
            }
            if self.position_independent_executables {
                check!(
                    self.executables,
                    "if `position_independent_executables` is set then `executables` must be set"
                );
            }
        }

        // Check crt static stuff
        if self.crt_static_default || self.crt_static_allows_dylibs {
            check!(
                self.crt_static_respected,
                "static CRT can be enabled but `crt_static_respected` is not set"
            );
        }

        // Check that RISC-V targets always specify which ABI they use,
        // and that ARM targets specify their float ABI.
        match self.arch {
            Arch::RiscV32 => {
                check_matches!(
                    &*self.llvm_abiname,
                    "ilp32" | "ilp32f" | "ilp32d" | "ilp32e",
                    "invalid RISC-V ABI name: {}",
                    self.llvm_abiname,
                );
            }
            Arch::RiscV64 => {
                // Note that the `lp64e` is still unstable as it's not (yet) part of the ELF psABI.
                check_matches!(
                    &*self.llvm_abiname,
                    "lp64" | "lp64f" | "lp64d" | "lp64e",
                    "invalid RISC-V ABI name: {}",
                    self.llvm_abiname,
                );
            }
            Arch::Arm => {
                check!(
                    self.llvm_floatabi.is_some(),
                    "ARM targets must set `llvm-floatabi` to `hard` or `soft`",
                )
            }
            _ => {}
        }

        // Check consistency of Rust ABI declaration.
        if let Some(rust_abi) = self.rustc_abi {
            match rust_abi {
                RustcAbi::X86Sse2 => check_matches!(
                    self.arch,
                    Arch::X86,
                    "`x86-sse2` ABI is only valid for x86-32 targets"
                ),
                RustcAbi::X86Softfloat => check_matches!(
                    self.arch,
                    Arch::X86 | Arch::X86_64,
                    "`x86-softfloat` ABI is only valid for x86 targets"
                ),
            }
        }

        // Check that the given target-features string makes some basic sense.
        if !self.features.is_empty() {
            let mut features_enabled = FxHashSet::default();
            let mut features_disabled = FxHashSet::default();
            for feat in self.features.split(',') {
                if let Some(feat) = feat.strip_prefix("+") {
                    features_enabled.insert(feat);
                    if features_disabled.contains(feat) {
                        return Err(format!(
                            "target feature `{feat}` is both enabled and disabled"
                        ));
                    }
                } else if let Some(feat) = feat.strip_prefix("-") {
                    features_disabled.insert(feat);
                    if features_enabled.contains(feat) {
                        return Err(format!(
                            "target feature `{feat}` is both enabled and disabled"
                        ));
                    }
                } else {
                    return Err(format!(
                        "target feature `{feat}` is invalid, must start with `+` or `-`"
                    ));
                }
            }
            // Check that we don't mis-set any of the ABI-relevant features.
            let abi_feature_constraints = self.abi_required_features();
            for feat in abi_feature_constraints.required {
                // The feature might be enabled by default so we can't *require* it to show up.
                // But it must not be *disabled*.
                if features_disabled.contains(feat) {
                    return Err(format!(
                        "target feature `{feat}` is required by the ABI but gets disabled in target spec"
                    ));
                }
            }
            for feat in abi_feature_constraints.incompatible {
                // The feature might be disabled by default so we can't *require* it to show up.
                // But it must not be *enabled*.
                if features_enabled.contains(feat) {
                    return Err(format!(
                        "target feature `{feat}` is incompatible with the ABI but gets enabled in target spec"
                    ));
                }
            }
        }

        Ok(())
    }

    /// Test target self-consistency and JSON encoding/decoding roundtrip.
    #[cfg(test)]
    fn test_target(mut self) {
        let recycled_target =
            Target::from_json(&serde_json::to_string(&self.to_json()).unwrap()).map(|(j, _)| j);
        self.update_to_cli();
        self.check_consistency(TargetKind::Builtin).unwrap();
        assert_eq!(recycled_target, Ok(self));
    }

    // Add your target to the whitelist if it has `std` library
    // and you certainly want "unknown" for the OS name.
    fn can_use_os_unknown(&self) -> bool {
        self.llvm_target == "wasm32-unknown-unknown"
            || self.llvm_target == "wasm64-unknown-unknown"
            || (self.env == Env::Sgx && self.vendor == "fortanix")
    }

    /// Load a built-in target
    pub fn expect_builtin(target_tuple: &TargetTuple) -> Target {
        match *target_tuple {
            TargetTuple::TargetTuple(ref target_tuple) => {
                load_builtin(target_tuple).expect("built-in target")
            }
            TargetTuple::TargetJson { .. } => {
                panic!("built-in targets doesn't support target-paths")
            }
        }
    }

    /// Load all built-in targets
    pub fn builtins() -> impl Iterator<Item = Target> {
        load_all_builtins()
    }

    /// Search for a JSON file specifying the given target tuple.
    ///
    /// If none is found in `$RUST_TARGET_PATH`, look for a file called `target.json` inside the
    /// sysroot under the target-tuple's `rustlib` directory. Note that it could also just be a
    /// bare filename already, so also check for that. If one of the hardcoded targets we know
    /// about, just return it directly.
    ///
    /// The error string could come from any of the APIs called, including filesystem access and
    /// JSON decoding.
    pub fn search(
        target_tuple: &TargetTuple,
        sysroot: &Path,
    ) -> Result<(Target, TargetWarnings), String> {
        use std::{env, fs};

        fn load_file(path: &Path) -> Result<(Target, TargetWarnings), String> {
            let contents = fs::read_to_string(path).map_err(|e| e.to_string())?;
            Target::from_json(&contents)
        }

        match *target_tuple {
            TargetTuple::TargetTuple(ref target_tuple) => {
                // check if tuple is in list of built-in targets
                if let Some(t) = load_builtin(target_tuple) {
                    return Ok((t, TargetWarnings::empty()));
                }

                // search for a file named `target_tuple`.json in RUST_TARGET_PATH
                let path = {
                    let mut target = target_tuple.to_string();
                    target.push_str(".json");
                    PathBuf::from(target)
                };

                let target_path = env::var_os("RUST_TARGET_PATH").unwrap_or_default();

                for dir in env::split_paths(&target_path) {
                    let p = dir.join(&path);
                    if p.is_file() {
                        return load_file(&p);
                    }
                }

                // Additionally look in the sysroot under `lib/rustlib/<tuple>/target.json`
                // as a fallback.
                let rustlib_path = crate::relative_target_rustlib_path(sysroot, target_tuple);
                let p = PathBuf::from_iter([
                    Path::new(sysroot),
                    Path::new(&rustlib_path),
                    Path::new("target.json"),
                ]);
                if p.is_file() {
                    return load_file(&p);
                }

                Err(format!("could not find specification for target {target_tuple:?}"))
            }
            TargetTuple::TargetJson { ref contents, .. } => Target::from_json(contents),
        }
    }

    /// Return the target's small data threshold support, converting
    /// `DefaultForArch` into a concrete value.
    pub fn small_data_threshold_support(&self) -> SmallDataThresholdSupport {
        match &self.options.small_data_threshold_support {
            // Avoid having to duplicate the small data support in every
            // target file by supporting a default value for each
            // architecture.
            SmallDataThresholdSupport::DefaultForArch => match self.arch {
                Arch::Mips | Arch::Mips64 | Arch::Mips32r6 => {
                    SmallDataThresholdSupport::LlvmArg("mips-ssection-threshold".into())
                }
                Arch::Hexagon => {
                    SmallDataThresholdSupport::LlvmArg("hexagon-small-data-threshold".into())
                }
                Arch::M68k => SmallDataThresholdSupport::LlvmArg("m68k-ssection-threshold".into()),
                Arch::RiscV32 | Arch::RiscV64 => {
                    SmallDataThresholdSupport::LlvmModuleFlag("SmallDataLimit".into())
                }
                _ => SmallDataThresholdSupport::None,
            },
            s => s.clone(),
        }
    }

    pub fn object_architecture(
        &self,
        unstable_target_features: &FxIndexSet<Symbol>,
    ) -> Option<(object::Architecture, Option<object::SubArchitecture>)> {
        use object::Architecture;
        Some(match self.arch {
            Arch::Arm => (Architecture::Arm, None),
            Arch::AArch64 => (
                if self.pointer_width == 32 {
                    Architecture::Aarch64_Ilp32
                } else {
                    Architecture::Aarch64
                },
                None,
            ),
            Arch::X86 => (Architecture::I386, None),
            Arch::S390x => (Architecture::S390x, None),
            Arch::M68k => (Architecture::M68k, None),
            Arch::Mips | Arch::Mips32r6 => (Architecture::Mips, None),
            Arch::Mips64 | Arch::Mips64r6 => (
                // While there are currently no builtin targets
                // using the N32 ABI, it is possible to specify
                // it using a custom target specification. N32
                // is an ILP32 ABI like the Aarch64_Ilp32
                // and X86_64_X32 cases above and below this one.
                if self.options.llvm_abiname.as_ref() == "n32" {
                    Architecture::Mips64_N32
                } else {
                    Architecture::Mips64
                },
                None,
            ),
            Arch::X86_64 => (
                if self.pointer_width == 32 {
                    Architecture::X86_64_X32
                } else {
                    Architecture::X86_64
                },
                None,
            ),
            Arch::PowerPC => (Architecture::PowerPc, None),
            Arch::PowerPC64 => (Architecture::PowerPc64, None),
            Arch::RiscV32 => (Architecture::Riscv32, None),
            Arch::RiscV64 => (Architecture::Riscv64, None),
            Arch::Sparc => {
                if unstable_target_features.contains(&sym::v8plus) {
                    // Target uses V8+, aka EM_SPARC32PLUS, aka 64-bit V9 but in 32-bit mode
                    (Architecture::Sparc32Plus, None)
                } else {
                    // Target uses V7 or V8, aka EM_SPARC
                    (Architecture::Sparc, None)
                }
            }
            Arch::Sparc64 => (Architecture::Sparc64, None),
            Arch::Avr => (Architecture::Avr, None),
            Arch::Msp430 => (Architecture::Msp430, None),
            Arch::Hexagon => (Architecture::Hexagon, None),
            Arch::Xtensa => (Architecture::Xtensa, None),
            Arch::Bpf => (Architecture::Bpf, None),
            Arch::LoongArch32 => (Architecture::LoongArch32, None),
            Arch::LoongArch64 => (Architecture::LoongArch64, None),
            Arch::CSky => (Architecture::Csky, None),
            Arch::Arm64EC => (Architecture::Aarch64, Some(object::SubArchitecture::Arm64EC)),
            Arch::AmdGpu
            | Arch::Nvptx64
            | Arch::PowerPC64LE
            | Arch::SpirV
            | Arch::Wasm32
            | Arch::Wasm64
            | Arch::Other(_) => return None,
        })
    }

    /// Returns whether this target is known to have unreliable alignment:
    /// native C code for the target fails to align some data to the degree
    /// required by the C standard. We can't *really* do anything about that
    /// since unsafe Rust code may assume alignment any time, but we can at least
    /// inhibit some optimizations, and we suppress the alignment checks that
    /// would detect this unsoundness.
    ///
    /// Every target that returns less than `Align::MAX` here is still has a soundness bug.
    pub fn max_reliable_alignment(&self) -> Align {
        // FIXME(#112480) MSVC on x86-32 is unsound and fails to properly align many types with
        // more-than-4-byte-alignment on the stack. This makes alignments larger than 4 generally
        // unreliable on 32bit Windows.
        if self.is_like_windows && self.arch == Arch::X86 {
            Align::from_bytes(4).unwrap()
        } else {
            Align::MAX
        }
    }

    pub fn vendor_symbol(&self) -> Symbol {
        Symbol::intern(&self.vendor)
    }
}
