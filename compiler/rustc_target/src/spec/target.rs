use core::result::Result;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

use rustc_abi::{
    Align, CanonAbi, Endian, Integer, Size, TargetDataLayout, TargetDataLayoutErrors,
};
use rustc_macros::{Decodable, Encodable, HashStable_Generic};
use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
use rustc_span::{Symbol, kw, sym};

use crate::json::{Json, ToJson};
use crate::spec::linker_flavor::{LinkerFlavor, Lld};
use crate::spec::target_options::TargetOptions;
use std::str::FromStr;
use crate::spec::Arch; // Assuming Arch is now in arch.rs
use crate::spec::linker_flavor::StaticCow;



/// `TargetOptions` being a separate type is basically an implementation detail of `Target` that is
/// used for providing defaults. Perhaps there's a way to merge `TargetOptions` into `Target` so
/// this `Deref` implementation is no longer necessary.
impl Deref for Target {
    type Target = TargetOptions;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.options
    }
}
impl DerefMut for Target {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.options
    }
}

// Every field here must be specified, and has no default value.
#[derive(PartialEq, Clone, Debug)]
pub struct Target {
    // Unversioned target tuple to pass to LLVM.
    //
    // Target tuples can optionally contain an OS version (notably Apple targets), which rustc
    // cannot know without querying the environment.
    //
    // Use `rustc_codegen_ssa::back::versioned_llvm_target` if you need the full LLVM target.
    pub llvm_target: StaticCow<str>,
    // Metadata about a target, for example the description or tier.
    // Used for generating target documentation.
    pub metadata: TargetMetadata,
    // Number of bits in a pointer. Influences the `target_pointer_width` `cfg` variable.
    pub pointer_width: u16,
    // Architecture to use for ABI considerations. Valid options include: "x86",
    // "x86_64", "arm", "aarch64", "mips", "powerpc", "powerpc64", and others.
    pub arch: Arch,
    // [Data layout](https://llvm.org/docs/LangRef.html#data-layout) to pass to LLVM.
    pub data_layout: StaticCow<str>,
    // Optional settings with defaults.
    pub options: TargetOptions,
}

// Metadata about a target like the description or tier.
// Part of #120745.
// All fields are optional for now, but intended to be required in the future.
#[derive(Default, PartialEq, Clone, Debug)]
pub struct TargetMetadata {
    // A short description of the target including platform requirements,
    // for example "64-bit Linux (kernel 3.2+, glibc 2.17+)".
    pub description: Option<StaticCow<str>>,
    // The tier of the target. 1, 2 or 3.
    pub tier: Option<u64>,
    // Whether the Rust project ships host tools for a target.
    pub host_tools: Option<bool>,
    // Whether a target has the `std` library. This is usually true for targets running
    // on an operating system.
    pub std: Option<bool>,
}

impl Target {
    pub fn parse_data_layout(&self) -> Result<TargetDataLayout, TargetDataLayoutErrors<'_>> {
        let mut dl = TargetDataLayout::parse_from_llvm_datalayout_string(
            &self.data_layout,
            self.options.default_address_space,
        )?;

        // Perform consistency checks against the Target information.
        if dl.endian != self.endian {
            return Err(TargetDataLayoutErrors::InconsistentTargetArchitecture {
                dl: dl.endian.as_str(),
                target: self.endian.as_str(),
            });
        }

        let target_pointer_width: u64 = self.pointer_width.into();
        let dl_pointer_size: u64 = dl.pointer_size().bits();
        if dl_pointer_size != target_pointer_width {
            return Err(TargetDataLayoutErrors::InconsistentTargetPointerWidth {
                pointer_size: dl_pointer_size,
                target: self.pointer_width,
            });
        }

        dl.c_enum_min_size = Integer::from_size(Size::from_bits(
            self.c_enum_min_bits.unwrap_or(self.c_int_width as _),
        ))
        .map_err(|err| TargetDataLayoutErrors::InvalidBitsSize { err })?;

        Ok(dl)
    }

    pub fn check_consistency(&self, kind: crate::spec::targets::TargetKind) -> Result<(), String> {
        // Placeholder implementation for now
        // This method would typically perform various checks to ensure the target's
        // configuration is consistent.
        // For example:
        // - Check if LLVM target is valid
        // - Check if pointer_width matches data_layout
        // - Check if ABI is supported for the current architecture
        Ok(())
    }
}

pub trait HasTargetSpec {
    fn target_spec(&self) -> &Target;
}

impl HasTargetSpec for Target {
    #[inline]
    fn target_spec(&self) -> &Target {
        self
    }
}

// x86 (32-bit) abi options.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct X86Abi {
    // On x86-32 targets, the regparm N causes the compiler to pass arguments
    // in registers EAX, EDX, and ECX instead of on the stack.
    pub regparm: Option<u32>,
    // Override the default ABI to return small structs in registers
    pub reg_struct_return: bool,
}

pub trait HasX86AbiOpt {
    fn x86_abi_opt(&self) -> X86Abi;
}

pub fn target_template() -> Target {
    Target {
        llvm_target: "".into(),
        metadata: Default::default(),
        pointer_width: 0,
        arch: crate::spec::Arch::Unknown, // Assuming Arch::Unknown exists or can be added
        data_layout: "".into(),
        options: TargetOptions::default(),
    }
}




crate::target_spec_enum! {
    pub enum Abi {
        EabiHf = "eabihf",
        ElfV1 = "elfv1",
        ElfV2 = "elfv2",
        Fortanix = "fortanix",
        Ilp32 = "ilp32",
        Ilp32e = "ilp32e",
        Llvm = "llvm",
        MacAbi = "macabi",
        Sim = "sim",
        SoftFloat = "softfloat",
        Spe = "spe",
        Uwp = "uwp",
        VecDefault = "vec-default",
        VecExtAbi = "vec-extabi",
        X32 = "x32",
        Unspecified = "",
    }
    other_variant = Other;
}

impl crate::json::ToJson for Abi {
    fn to_json(&self) -> crate::json::Json {
        self.desc().to_json()
    }
}
