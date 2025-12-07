use crate::json::ToJson;
use rustc_macros::{Decodable, Encodable, HashStable_Generic};
use rustc_span::{Symbol, kw, sym};

crate::target_spec_enum! {
    pub enum Os {
        Aix = "aix",
        AmdHsa = "amdhsa",
        Android = "android",
        Cuda = "cuda",
        Cygwin = "cygwin",
        Dragonfly = "dragonfly",
        Emscripten = "emscripten",
        EspIdf = "espidf",
        FreeBsd = "freebsd",
        Fuchsia = "fuchsia",
        Haiku = "haiku",
        HelenOs = "helenos",
        Hermit = "hermit",
        Horizon = "horizon",
        Hurd = "hurd",
        Illumos = "illumos",
        IOs = "ios",
        L4Re = "l4re",
        Linux = "linux",
        LynxOs178 = "lynxos178",
        MacOs = "macos",
        Managarm = "managarm",
        Motor = "motor",
        NetBsd = "netbsd",
        None = "none",
        Nto = "nto",
        NuttX = "nuttx",
        OpenBsd = "openbsd",
        Psp = "psp",
        Psx = "psx",
        Qurt = "qurt",
        Redox = "redox",
        Rtems = "rtems",
        Solaris = "solaris",
        SolidAsp3 = "solid_asp3",
        TeeOs = "teeos",
        Trusty = "trusty",
        TvOs = "tvos",
        Uefi = "uefi",
        VexOs = "vexos",
        VisionOs = "visionos",
        Vita = "vita",
        VxWorks = "vxworks",
        Wasi = "wasi",
        WatchOs = "watchos",
        Windows = "windows",
        Xous = "xous",
        Zkvm = "zkvm",
        Unknown = "unknown",
    }
    other_variant = Other;
}

impl Os {
    pub fn desc_symbol(&self) -> Symbol {
        Symbol::intern(self.desc())
    }
}

impl crate::json::ToJson for Os {
    fn to_json(&self) -> crate::json::Json {
        self.desc().to_json()
    }
}