/// Optional aspects of a target specification.
///
/// This has an implementation of `Default`, see each field for what the default is. In general,
/// these try to take "minimal defaults" that don't assume anything about the runtime they run in.
///
    /// executable, aka there is no native linker for this target.
    pub requires_lto: bool,

    /// This target has no support for threads.
    pub singlethread: bool,

    /// Whether library functions call lowering/optimization is disabled in LLVM
    /// for this target unconditionally.
    pub no_builtins: bool,

    /// The default visibility for symbols in this target.
    ///
    /// This value typically shouldn't be accessed directly, but through the
    /// `rustc_session::Session::default_visibility` method, which allows `rustc` users to override
    /// this setting using cmdline flags.
    pub default_visibility: Option<SymbolVisibility>,

    /// Whether a .debug_gdb_scripts section will be added to the output object file
    pub emit_debug_gdb_scripts: bool,

    /// Whether or not to unconditionally `uwtable` attributes on functions,
    /// typically because the platform needs to unwind for things like stack
    /// unwinders.
    pub requires_uwtable: bool,

    /// Whether or not to emit `uwtable` attributes on functions if `-C force-unwind-tables`
    /// is not specified and `uwtable` is not required on this target.
    pub default_uwtable: bool,

    /// Whether or not SIMD types are passed by reference in the Rust ABI,
    /// typically required if a target can be compiled with a mixed set of
    /// target features. This is `true` by default, and `false` for targets like
    /// wasm32 where the whole program either has simd or not.
    pub simd_types_indirect: bool,

    /// Pass a list of symbol which should be exported in the dylib to the linker.
    pub limit_rdylib_exports: bool,

    /// If set, have the linker export exactly these symbols, instead of using
    /// the usual logic to figure this out from the crate itself.
    pub override_export_symbols: Option<StaticCow<[StaticCow<str>]>>,

    /// Determines how or whether the MergeFunctions LLVM pass should run for
    /// this target. Either "disabled", "trampolines", or "aliases".
    /// The MergeFunctions pass is generally useful, but some targets may need
    /// to opt out. The default is "aliases".
    ///
    /// Workaround for: <https://github.com/rust-lang/rust/issues/57356>
    pub merge_functions: MergeFunctions,

    /// Use platform dependent mcount function
    pub mcount: StaticCow<str>,

    /// Use LLVM intrinsic for mcount function name
    pub llvm_mcount_intrinsic: Option<StaticCow<str>>,

    /// LLVM ABI name, corresponds to the '-mabi' parameter available in multilib C compilers
    /// and the `-target-abi` flag in llc. In the LLVM API this is `MCOptions.ABIName`.
    pub llvm_abiname: StaticCow<str>,

    /// Control the float ABI to use, for architectures that support it. The only architecture we
    /// currently use this for is ARM. Corresponds to the `-float-abi` flag in llc. In the LLVM API
    /// this is `FloatABIType`. (clang's `-mfloat-abi` is similar but more complicated since it
    /// can also affect the `soft-float` target feature.)
    ///
    /// If not provided, LLVM will infer the float ABI from the target triple (`llvm_target`).
    pub llvm_floatabi: Option<FloatAbi>,

    /// Picks a specific ABI for this target. This is *not* just for "Rust" ABI functions,
    /// it can also affect "C" ABI functions; the point is that this flag is interpreted by
    /// rustc and not forwarded to LLVM.
    /// So far, this is only used on x86.
    pub rustc_abi: Option<RustcAbi>,

    /// Whether or not RelaxElfRelocation flag will be passed to the linker
    pub relax_elf_relocations: bool,

    /// Additional arguments to pass to LLVM, similar to the `-C llvm-args` codegen option.
    pub llvm_args: StaticCow<[StaticCow<str>]>,

    /// Whether to use legacy .ctors initialization hooks rather than .init_array. Defaults
    /// to false (uses .init_array).
    pub use_ctors_section: bool,

    /// Whether the linker is instructed to add a `GNU_EH_FRAME` ELF header
    /// used to locate unwinding information is passed
    /// (only has effect if the linker is `ld`-like).
    pub eh_frame_header: bool,

    /// Is true if the target is an ARM architecture using thumb v1 which allows for
    /// thumb and arm interworking.
    pub has_thumb_interworking: bool,

    /// Which kind of debuginfo is used by this target?
    pub debuginfo_kind: DebuginfoKind,
    /// How to handle split debug information, if at all. Specifying `None` has
    /// target-specific meaning.
    pub split_debuginfo: SplitDebuginfo,
    /// Which kinds of split debuginfo are supported by the target?
    pub supported_split_debuginfo: StaticCow<[SplitDebuginfo]>,

    /// The sanitizers supported by this target
    ///
    /// Note that the support here is at a codegen level. If the machine code with sanitizer
    /// enabled can generated on this target, but the necessary supporting libraries are not
    /// distributed with the target, the sanitizer should still appear in this list for the target.
    pub supported_sanitizers: SanitizerSet,

    /// The sanitizers that are enabled by default on this target.
    ///
    /// Note that the support here is at a codegen level. If the machine code with sanitizer
    /// enabled can generated on this target, but the necessary supporting libraries are not
    /// distributed with the target, the sanitizer should still appear in this list for the target.
    pub default_sanitizers: SanitizerSet,

    /// Minimum number of bits in #[repr(C)] enum. Defaults to the size of c_int
    pub c_enum_min_bits: Option<u64>,

    /// Whether or not the DWARF `.debug_aranges` section should be generated.
    pub generate_arange_section: bool,

    /// Whether the target supports stack canary checks. `true` by default,
    /// since this is most common among tier 1 and tier 2 targets.
    pub supports_stack_protector: bool,

    /// The name of entry function.
    /// Default value is "main"
    pub entry_name: StaticCow<str>,

    /// The ABI of the entry function.
    /// Default value is `CanonAbi::C`
    pub entry_abi: CanonAbi,

    /// Whether the target supports XRay instrumentation.
    pub supports_xray: bool,

    /// The default address space for this target. When using LLVM as a backend, most targets simply
    /// use LLVM's default address space (0). Some other targets, such as CHERI targets, use a
    /// custom default address space (in this specific case, `200`).
    pub default_address_space: rustc_abi::AddressSpace,

    /// Whether the targets supports -Z small-data-threshold
    small_data_threshold_support: SmallDataThresholdSupport,
}
