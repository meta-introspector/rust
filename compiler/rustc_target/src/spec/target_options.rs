use std::borrow::Cow;
use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

use super::{
    Abi, BinaryFormat, CanonAbi, Cc, CodeModel, CrtObjects, DebuginfoKind,
    Env, Endian, FloatAbi, FramePointer, LinkerFlavor, LinkerFlavorCli, LinkSelfContainedDefault,
    Lld, LldFlavor, MergeFunctions, Os, PanicStrategy, RelocModel, RelroLevel, RustcAbi,
    SanitizerSet, SmallDataThresholdSupport, SplitDebuginfo, StackProbeType, SymbolVisibility,
    TlsModel, Target
};

use rustc_abi::AddressSpace;
use rustc_span::Symbol;
use rustc_target::abi::Align;

pub type LinkArgs = BTreeMap<LinkerFlavor, Vec<StaticCow<str>>>;
pub type LinkArgsCli = BTreeMap<LinkerFlavorCli, Vec<StaticCow<str>>>;

/// `TargetOptions` as a separate structure is mostly an implementation detail of `Target`
/// construction, all its fields logically belong to `Target` and available from `Target`
/// through `Deref` impls.
#[derive(PartialEq, Clone, Debug)]

pub struct TargetOptions {
    /// Used as the `target_endian` `cfg` variable. Defaults to little endian.
    pub endian: Endian,
    /// Width of c_int type. Defaults to "32".
    pub c_int_width: u16,
    /// OS name to use for conditional compilation (`target_os`). Defaults to [`Os::None`].
    /// [`Os::None`] implies a bare metal target without `std` library.
    /// A couple of targets having `std` also use [`Os::Unknown`] as their `os` value,
    /// but they are exceptions.
    pub os: Os,
    /// Environment name to use for conditional compilation (`target_env`). Defaults to [`Env::Unspecified`].
    pub env: Env,
    /// ABI name to distinguish multiple ABIs on the same OS and architecture. For instance, `"eabi"`
    /// or `"eabihf"`. Defaults to [`Abi::Unspecified`].
    /// This field is *not* forwarded directly to LLVM; its primary purpose is `cfg(target_abi)`.
    /// However, parts of the backend do check this field for specific values to enable special behavior.
    pub abi: Abi,

    vendor: StaticCow<str>,

    /// Linker to invoke
    pub linker: Option<StaticCow<str>>,
    /// Default linker flavor used if `-C linker-flavor` or `-C linker` are not passed
    /// on the command line. Defaults to `LinkerFlavor::Gnu(Cc::Yes, Lld::No)`.
    pub linker_flavor: LinkerFlavor,
    linker_flavor_json: LinkerFlavorCli,
    lld_flavor_json: LldFlavor,
    linker_is_gnu_json: bool,

    /// Objects to link before and after all other object code.
    pub pre_link_objects: CrtObjects,
    pub post_link_objects: CrtObjects,
    /// Same as `(pre|post)_link_objects`, but when self-contained linking mode is enabled.
    pub pre_link_objects_self_contained: CrtObjects,
    pub post_link_objects_self_contained: CrtObjects,
    /// Behavior for the self-contained linking mode: inferred for some targets, or explicitly
    /// enabled (in bulk, or with individual components).
    pub link_self_contained: LinkSelfContainedDefault,

    /// Linker arguments that are passed *before* any user-defined libraries.
    pub pre_link_args: LinkArgs,
    pre_link_args_json: LinkArgsCli,
    /// Linker arguments that are unconditionally passed after any
    /// user-defined but before post-link objects. Standard platform
    /// libraries that should be always be linked to, usually go here.
    pub late_link_args: LinkArgs,
    late_link_args_json: LinkArgsCli,
    /// Linker arguments used in addition to `late_link_args` if at least one
    /// Rust dependency is dynamically linked.
    pub late_link_args_dynamic: LinkArgs,
    late_link_args_dynamic_json: LinkArgsCli,
    /// Linker arguments used in addition to `late_link_args` if all Rust
    /// dependencies are statically linked.
    pub late_link_args_static: LinkArgs,
    late_link_args_static_json: LinkArgsCli,
    /// Linker arguments that are unconditionally passed *after* any
    /// user-defined libraries.
    pub post_link_args: LinkArgs,
    post_link_args_json: LinkArgsCli,

    /// Optional link script applied to `dylib` and `executable` crate types.
    /// This is a string containing the script, not a path. Can only be applied
    /// to linkers where linker flavor matches `LinkerFlavor::Gnu(..)`.
    pub link_script: Option<StaticCow<str>>,
    /// Environment variables to be set for the linker invocation.
    pub link_env: StaticCow<[(StaticCow<str>, StaticCow<str>)]>,
    /// Environment variables to be removed for the linker invocation.
    pub link_env_remove: StaticCow<[StaticCow<str>]>,

    /// Extra arguments to pass to the external assembler (when used)
    pub asm_args: StaticCow<[StaticCow<str>]>,

    /// Default CPU to pass to LLVM. Corresponds to `llc -mcpu=$cpu`. Defaults
    /// to "generic".
    pub cpu: StaticCow<str>,
    /// Whether a cpu needs to be explicitly set.
    /// Set to true if there is no default cpu. Defaults to false.
    pub need_explicit_cpu: bool,
    /// Default target features to pass to LLVM. These features overwrite
    /// `-Ctarget-cpu` but can be overwritten with `-Ctarget-features`.
    /// Corresponds to `llc -mattr=$features`.
    /// Note that these are LLVM feature names, not Rust feature names!
    ///
    /// Generally it is a bad idea to use negative target features because they often interact very
    /// poorly with how `-Ctarget-cpu` works. Instead, try to use a lower "base CPU" and enable the
    /// features you want to use.
    pub features: StaticCow<str>,
    /// Direct or use GOT indirect to reference external data symbols
    pub direct_access_external_data: Option<bool>,
    /// Whether dynamic linking is available on this target. Defaults to false.
    pub dynamic_linking: bool,
    /// Whether dynamic linking can export TLS globals. Defaults to true.
    pub dll_tls_export: bool,
    /// If dynamic linking is available, whether only cdylibs are supported.
    pub only_cdylib: bool,
    /// Whether executables are available on this target. Defaults to true.
    pub executables: bool,
    /// Relocation model to use in object file. Corresponds to `llc
    /// -relocation-model=$relocation_model`. Defaults to `Pic`.
    pub relocation_model: RelocModel,
    /// Code model to use. Corresponds to `llc -code-model=$code_model`.
    /// Defaults to `None` which means "inherited from the base LLVM target".
    pub code_model: Option<CodeModel>,
    /// TLS model to use. Options are "global-dynamic" (default), "local-dynamic", "initial-exec"
    /// and "local-exec". This is similar to the -ftls-model option in GCC/Clang.
    pub tls_model: TlsModel,
    /// Do not emit code that uses the "red zone", if the ABI has one. Defaults to false.
    pub disable_redzone: bool,
    /// Frame pointer mode for this target. Defaults to `MayOmit`.
    pub frame_pointer: FramePointer,
    /// Emit each function in its own section. Defaults to true.
    pub function_sections: bool,
    /// String to prepend to the name of every dynamic library. Defaults to "lib".
    pub dll_prefix: StaticCow<str>,
    /// String to append to the name of every dynamic library. Defaults to ".so".
    pub dll_suffix: StaticCow<str>,
    /// String to append to the name of every executable.
    pub exe_suffix: StaticCow<str>,
    /// String to prepend to the name of every static library. Defaults to "lib".
    pub staticlib_prefix: StaticCow<str>,
    /// String to append to the name of every static library. Defaults to ".a".
    pub staticlib_suffix: StaticCow<str>,
    /// Values of the `target_family` cfg set for this target.
    ///
    /// Common options are: "unix", "windows". Defaults to no families.
    ///
    /// See <https://doc.rust-lang.org/reference/conditional-compilation.html#target_family>.
    pub families: StaticCow<[StaticCow<str>]>,
    /// Whether the target toolchain's ABI supports returning small structs as an integer.
    pub abi_return_struct_as_int: bool,
    /// Whether the target toolchain is like AIX's. Linker options on AIX are special and it uses
    /// XCOFF as binary format. Defaults to false.
    pub is_like_aix: bool,
    /// Whether the target toolchain is like macOS's. Only useful for compiling against iOS/macOS,
    /// in particular running dsymutil and some other stuff like `-dead_strip`. Defaults to false.
    /// Also indicates whether to use Apple-specific ABI changes, such as extending function
    /// parameters to 32-bits.
    pub is_like_darwin: bool,
    /// Whether the target is a GPU (e.g. NVIDIA, AMD, Intel).
    pub is_like_gpu: bool,
    /// Whether the target toolchain is like Solaris's.
    /// Only useful for compiling against Illumos/Solaris,
    /// as they have a different set of linker flags. Defaults to false.
    pub is_like_solaris: bool,
    /// Whether the target is like Windows.
    /// This is a combination of several more specific properties represented as a single flag:
    ///   - The target uses a Windows ABI,
    ///   - uses PE/COFF as a format for object code,
    ///   - uses Windows-style dllexport/dllimport for shared libraries,
    ///   - uses import libraries and .def files for symbol exports,
    ///   - executables support setting a subsystem.
    pub is_like_windows: bool,
    /// Whether the target is like MSVC.
    /// This is a combination of several more specific properties represented as a single flag:
    ///   - The target has all the properties from `is_like_windows`
    ///     (for in-tree targets "is_like_msvc ⇒ is_like_windows" is ensured by a unit test),
    ///   - has some MSVC-specific Windows ABI properties,
    ///   - uses a link.exe-like linker,
    ///   - uses CodeView/PDB for debuginfo and natvis for its visualization,
    ///   - uses SEH-based unwinding,
    ///   - supports control flow guard mechanism.
    pub is_like_msvc: bool,
    /// Whether a target toolchain is like WASM.
    pub is_like_wasm: bool,
    /// Whether a target toolchain is like Android, implying a Linux kernel and a Bionic libc
    pub is_like_android: bool,
    /// Whether a target toolchain is like VEXos, the operating system used by the VEX Robotics V5 Brain.
    pub is_like_vexos: bool,
    /// Target's binary file format. Defaults to BinaryFormat::Elf
    pub binary_format: BinaryFormat,
    /// Default supported version of DWARF on this platform.
    /// Useful because some platforms (osx, bsd) only want up to DWARF2.
    pub default_dwarf_version: u32,
    /// The MinGW toolchain has a known issue that prevents it from correctly
    /// handling COFF object files with more than 2<sup>15</sup> sections. Since each weak
    /// symbol needs its own COMDAT section, weak linkage implies a large
    /// number sections that easily exceeds the given limit for larger
    /// codebases. Consequently we want a way to disallow weak linkage on some
    /// platforms.
    pub allows_weak_linkage: bool,
    /// Whether the linker support rpaths or not. Defaults to false.
    pub has_rpath: bool,
    /// Whether to disable linking to the default libraries, typically corresponds
    /// to `-nodefaultlibs`. Defaults to true.
    pub no_default_libraries: bool,
    /// Dynamically linked executables can be compiled as position independent
    /// if the default relocation model of position independent code is not
    /// changed. This is a requirement to take advantage of ASLR, as otherwise
    /// the functions in the executable are not randomized and can be used
    /// during an exploit of a vulnerability in any code.
    pub position_independent_executables: bool,
    /// Executables that are both statically linked and position-independent are supported.
    pub static_position_independent_executables: bool,
    /// Determines if the target always requires using the PLT for indirect
    /// library calls or not. This controls the default value of the `-Z plt` flag.
    pub plt_by_default: bool,
    /// Either partial, full, or off. Full RELRO makes the dynamic linker
    /// resolve all symbols at startup and marks the GOT read-only before
    /// starting the program, preventing overwriting the GOT.
    pub relro_level: RelroLevel,
    /// Format that archives should be emitted in. This affects whether we use
    /// LLVM to assemble an archive or fall back to the system linker, and
    /// currently only "gnu" is used to fall into LLVM. Unknown strings cause
    /// the system linker to be used.
    pub archive_format: StaticCow<str>,
    /// Is asm!() allowed? Defaults to true.
    pub allow_asm: bool,
    /// Whether the runtime startup code requires the `main` function be passed
    /// `argc` and `argv` values.
    pub main_needs_argc_argv: bool,

    /// Flag indicating whether #[thread_local] is available for this target.
    pub has_thread_local: bool,
    /// This is mainly for easy compatibility with emscripten.
    /// If we give emcc .o files that are actually .bc files it
    /// will 'just work'.
    pub obj_is_bitcode: bool,

    /// Don't use this field; instead use the `.min_atomic_width()` method.
    pub min_atomic_width: Option<u64>,

    /// Don't use this field; instead use the `.max_atomic_width()` method.
    pub max_atomic_width: Option<u64>,

    /// Whether the target supports atomic CAS operations natively
    pub atomic_cas: bool,

    /// Panic strategy: "unwind" or "abort"
    pub panic_strategy: PanicStrategy,

    /// Whether or not linking dylibs to a static CRT is allowed.
    pub crt_static_allows_dylibs: bool,
    /// Whether or not the CRT is statically linked by default.
    pub crt_static_default: bool,
    /// Whether or not crt-static is respected by the compiler (or is a no-op).
    pub crt_static_respected: bool,

    /// The implementation of stack probes to use.
    pub stack_probes: StackProbeType,

    /// The minimum alignment for global symbols.
    pub min_global_align: Option<Align>,

    /// Default number of codegen units to use in debug mode
    pub default_codegen_units: Option<u64>,

    /// Default codegen backend used for this target. Defaults to `None`.
    ///
    /// If `None`, then `CFG_DEFAULT_CODEGEN_BACKEND` environmental variable captured when
    /// compiling `rustc` will be used instead (or llvm if it is not set).
    ///
    /// N.B. when *using* the compiler, backend can always be overridden with `-Zcodegen-backend`.
    ///
    /// This was added by WaffleLapkin in #116793. The motivation is a rustc fork that requires a
    /// custom codegen backend for a particular target.
    pub default_codegen_backend: Option<StaticCow<str>>,

    /// Whether to generate trap instructions in places where optimization would
    /// otherwise produce control flow that falls through into unrelated memory.
    pub trap_unreachable: bool,

    /// This target requires everything to be compiled with LTO to emit a final
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


impl Default for TargetOptions {
    /// Creates a set of "sane defaults" for any target. This is still
    /// incomplete, and if used for compilation, will certainly not work.
    fn default() -> TargetOptions {
        TargetOptions {
            endian: Endian::Little,
            c_int_width: 32,
            os: Os::None,
            env: Env::Unspecified,
            abi: Abi::Unspecified,
            vendor: "unknown".into(),
            linker: option_env!("CFG_DEFAULT_LINKER").map(|s| s.into()),
            linker_flavor: LinkerFlavor::Gnu(Cc::Yes, Lld::No),
            linker_flavor_json: LinkerFlavorCli::Gcc,
            lld_flavor_json: LldFlavor::Ld,
            linker_is_gnu_json: true,
            link_script: None,
            asm_args: cvs![],
            cpu: "generic".into(),
            need_explicit_cpu: false,
            features: "".into(),
            direct_access_external_data: None,
            dynamic_linking: false,
            dll_tls_export: true,
            only_cdylib: false,
            executables: true,
            relocation_model: RelocModel::Pic,
            code_model: None,
            tls_model: TlsModel::GeneralDynamic,
            disable_redzone: false,
            frame_pointer: FramePointer::MayOmit,
            function_sections: true,
            dll_prefix: "lib".into(),
            dll_suffix: ".so".into(),
            exe_suffix: "".into(),
            staticlib_prefix: "lib".into(),
            staticlib_suffix: ".a".into(),
            families: cvs![],
            abi_return_struct_as_int: false,
            is_like_aix: false,
            is_like_darwin: false,
            is_like_gpu: false,
            is_like_solaris: false,
            is_like_windows: false,
            is_like_msvc: false,
            is_like_wasm: false,
            is_like_android: false,
            is_like_vexos: false,
            binary_format: BinaryFormat::Elf,
            default_dwarf_version: 4,
            allows_weak_linkage: true,
            has_rpath: false,
            no_default_libraries: true,
            position_independent_executables: false,
            static_position_independent_executables: false,
            plt_by_default: true,
            relro_level: RelroLevel::None,
            pre_link_objects: Default::default(),
            post_link_objects: Default::default(),
            pre_link_objects_self_contained: Default::default(),
            post_link_objects_self_contained: Default::default(),
            link_self_contained: LinkSelfContainedDefault::False,
            pre_link_args: LinkArgs::new(),
            pre_link_args_json: LinkArgsCli::new(),
            late_link_args: LinkArgs::new(),
            late_link_args_json: LinkArgsCli::new(),
            late_link_args_dynamic: LinkArgs::new(),
            late_link_args_dynamic_json: LinkArgsCli::new(),
            late_link_args_static: LinkArgs::new(),
            late_link_args_static_json: LinkArgsCli::new(),
            post_link_args: LinkArgs::new(),
            post_link_args_json: LinkArgsCli::new(),
            link_env: cvs![],
            link_env_remove: cvs![],
            archive_format: "gnu".into(),
            main_needs_argc_argv: true,
            allow_asm: true,
            has_thread_local: false,
            obj_is_bitcode: false,
            min_atomic_width: None,
            max_atomic_width: None,
            atomic_cas: true,
            panic_strategy: PanicStrategy::Unwind,
            crt_static_allows_dylibs: false,
            crt_static_default: false,
            crt_static_respected: false,
            stack_probes: StackProbeType::None,
            min_global_align: None,
            default_codegen_units: None,
            default_codegen_backend: None,
            trap_unreachable: true,
            requires_lto: false,
            singlethread: false,
            no_builtins: false,
            default_visibility: None,
            emit_debug_gdb_scripts: true,
            requires_uwtable: false,
            default_uwtable: false,
            simd_types_indirect: true,
            limit_rdylib_exports: true,
            override_export_symbols: None,
            merge_functions: MergeFunctions::Aliases,
            mcount: "mcount".into(),
            llvm_mcount_intrinsic: None,
            llvm_abiname: "".into(),
            llvm_floatabi: None,
            rustc_abi: None,
            relax_elf_relocations: false,
            llvm_args: cvs![],
            use_ctors_section: false,
            eh_frame_header: true,
            has_thumb_interworking: false,
            debuginfo_kind: Default::default(),
            split_debuginfo: Default::default(),
            // `Off` is supported by default, but targets can remove this manually, e.g. Windows.
            supported_split_debuginfo: Cow::Borrowed(&[SplitDebuginfo::Off]),
            supported_sanitizers: SanitizerSet::empty(),
            default_sanitizers: SanitizerSet::empty(),
            c_enum_min_bits: None,
            generate_arange_section: true,
            supports_stack_protector: true,
            entry_name: "main".into(),
            entry_abi: CanonAbi::C,
            supports_xray: false,
            default_address_space: rustc_abi::AddressSpace::ZERO,
            small_data_threshold_support: SmallDataThresholdSupport::DefaultForArch,
        }
    }
}

impl TargetOptions {
    pub fn supports_comdat(&self) -> bool {
        // XCOFF and MachO don't support COMDAT.
        !self.is_like_aix && !self.is_like_darwin
    }
}

impl TargetOptions {
    fn link_args(flavor: LinkerFlavor, args: &[&'static str]) -> LinkArgs {
        let mut link_args = LinkArgs::new();
        add_link_args(&mut link_args, flavor, args);
        link_args
    }

    fn add_pre_link_args(&mut self, flavor: LinkerFlavor, args: &[&'static str]) {
        add_link_args(&mut self.pre_link_args, flavor, args);
    }

    fn update_from_cli(&mut self) {
        self.linker_flavor = LinkerFlavor::from_cli_json(
            self.linker_flavor_json,
            self.lld_flavor_json,
            self.linker_is_gnu_json,
        );
        for (args, args_json) in [
            (&mut self.pre_link_args, &self.pre_link_args_json),
            (&mut self.late_link_args, &self.late_link_args_json),
            (&mut self.late_link_args_dynamic, &self.late_link_args_dynamic_json),
            (&mut self.late_link_args_static, &self.late_link_args_static_json),
            (&mut self.post_link_args, &self.post_link_args_json),
        ] {
            args.clear();
            for (flavor, args_json) in args_json {
                let linker_flavor = self.linker_flavor.with_cli_hints(*flavor);
                // Normalize to no lld to avoid asserts.
                let linker_flavor = match linker_flavor {
                    LinkerFlavor::Gnu(cc, _) => LinkerFlavor::Gnu(cc, Lld::No),
                    LinkerFlavor::Darwin(cc, _) => LinkerFlavor::Darwin(cc, Lld::No),
                    LinkerFlavor::Msvc(_) => LinkerFlavor::Msvc(Lld::No),
                    _ => linker_flavor,
                };
                if !args.contains_key(&linker_flavor) {
                    add_link_args_iter(args, linker_flavor, args_json.iter().cloned());
                }
            }
        }
    }

    fn update_to_cli(&mut self) {
        self.linker_flavor_json = self.linker_flavor.to_cli_counterpart();
        self.lld_flavor_json = self.linker_flavor.lld_flavor();
        self.linker_is_gnu_json = self.linker_flavor.is_gnu();
        for (args, args_json) in [
            (&self.pre_link_args, &mut self.pre_link_args_json),
            (&self.late_link_args, &mut self.late_link_args_json),
            (&self.late_link_args_dynamic, &mut self.late_link_args_dynamic_json),
            (&self.late_link_args_static, &mut self.late_link_args_static_json),
            (&self.post_link_args, &mut self.post_link_args_json),
        ] {
            *args_json = args
                .iter()
                .map(|(flavor, args)| (flavor.to_cli_counterpart(), args.clone()))
                .collect();
        }
    }
}

