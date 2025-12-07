use crate::json::ToJson;
use rustc_macros::{Decodable, Encodable, HashStable_Generic};
use rustc_span::{Symbol, kw, sym};

crate::target_spec_enum! {
    pub enum Arch {
        AArch64 = "aarch64",
        AmdGpu = "amdgpu",
        Arm = "arm",
        Arm64EC = "arm64ec",
        Avr = "avr",
        Bpf = "bpf",
        CSky = "csky",
        Hexagon = "hexagon",
        LoongArch32 = "loongarch32",
        LoongArch64 = "loongarch64",
        M68k = "m68k",
        Mips = "mips",
        Mips32r6 = "mips32r6",
        Mips64 = "mips64",
        Mips64r6 = "mips64r6",
        Msp430 = "msp430",
        Nvptx64 = "nvptx64",
        PowerPC = "powerpc",
        PowerPC64 = "powerpc64",
        PowerPC64LE = "powerpc64le",
        RiscV32 = "riscv32",
        RiscV64 = "riscv64",
        S390x = "s390x",
        Sparc = "sparc",
        Sparc64 = "sparc64",
        SpirV = "spirv",
        Wasm32 = "wasm32",
        Wasm64 = "wasm64",
        X86 = "x86",
        X86_64 = "x86_64",
        Xtensa = "xtensa",
    }
    other_variant = Other;
}

impl Arch {
    pub fn desc_symbol(&self) -> Symbol {
        match self {
            Self::AArch64 => sym::aarch64,
            Self::AmdGpu => sym::amdgpu,
            Self::Arm => sym::arm,
            Self::Arm64EC => sym::arm64ec,
            Self::Avr => sym::avr,
            Self::Bpf => sym::bpf,
            Self::CSky => sym::csky,
            Self::Hexagon => sym::hexagon,
            Self::LoongArch32 => sym::loongarch32,
            Self::LoongArch64 => sym::loongarch64,
            Self::M68k => sym::m68k,
            Self::Mips => sym::mips,
            Self::Mips32r6 => sym::mips32r6,
            Self::Mips64 => sym::mips64,
            Self::Mips64r6 => sym::mips64r6,
            Self::Msp430 => sym::msp430,
            Self::Nvptx64 => sym::nvptx64,
            Self::PowerPC => sym::powerpc,
            Self::PowerPC64 => sym::powerpc64,
            Self::PowerPC64LE => sym::powerpc64le,
            Self::RiscV32 => sym::riscv32,
            Self::RiscV64 => sym::riscv64,
            Self::S390x => sym::s390x,
            Self::Sparc => sym::sparc,
            Self::Sparc64 => sym::sparc64,
            Self::SpirV => sym::spirv,
            Self::Wasm32 => sym::wasm32,
            Self::Wasm64 => sym::wasm64,
            Self::X86 => sym::x86,
            Self::X86_64 => sym::x86_64,
            Self::Xtensa => sym::xtensa,
            Self::Other(name) => rustc_span::Symbol::intern(name),
        }
    }
}

impl crate::json::ToJson for Arch {
    fn to_json(&self) -> crate::json::Json {
        self.desc().to_json()
    }
}