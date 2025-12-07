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
