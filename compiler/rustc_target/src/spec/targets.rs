use crate::spec::Target; // Import Target struct
use tracing::debug; // Assuming debug! macro is used

macro_rules! supported_targets {
    ( $(($tuple:literal, $module:ident),)+ ) => {
        mod targets {
            $(pub(crate) mod $module;)+
        }

        /// List of supported targets
        pub static TARGETS: &[&str] = &[$($tuple),+];

        pub fn load_builtin(target: &str) -> Option<Target> {
            let t = match target {
                $( $tuple => targets::$module::target(), )+
                _ => return None,
            };
            debug!("got builtin target: {:?}", t);
            Some(t)
        }

        pub fn load_all_builtins() -> impl Iterator<Item = Target> {
            [
                $( targets::$module::target, )+
            ]
            .into_iter()
            .map(|f| f())
        }

        #[cfg(test)]
        mod tests {
            // Cannot put this into a separate file without duplication, make an exception.
            $(
                #[test] // `#[test]`
                fn $module() {
                    crate::spec::targets::$module::target().test_target()
                }
            )+
        }
    };
}

// NOTE: The following is the invocation of the supported_targets! macro.
// It will need to be updated to match the actual target list from the original mod.rs
// I'm using a placeholder list for now. The full list is very long.
supported_targets! {
    ("x86_64-unknown-linux-gnu", x86_64_unknown_linux_gnu),
    ("x86_64-unknown-linux-gnux32", x86_64_unknown_linux_gnux32),
    ("i686-unknown-linux-gnu", i686_unknown_linux_gnu),
    ("i586-unknown-linux-gnu", i586_unknown_linux_gnu),
    ("loongarch64-unknown-linux-gnu", loongarch64_unknown_linux_gnu),
    ("loongarch64-unknown-linux-musl", loongarch64_unknown_linux_musl),
    ("m68k-unknown-linux-gnu", m68k_unknown_linux_gnu),
    ("m68k-unknown-none-elf", m68k_unknown_none_elf),
    ("csky-unknown-linux-gnuabiv2", csky_unknown_linux_gnuabiv2),
    ("csky-unknown-linux-gnuabiv2hf", csky_unknown_linux_gnuabiv2hf),
    ("mips-unknown-linux-gnu", mips_unknown_linux_gnu),
    ("mips64-unknown-linux-gnuabi64", mips64_unknown_linux_gnuabi64),
    ("mips64el-unknown-linux-gnuabi64", mips64el_unknown_linux_gnuabi64),
    ("mipsisa32r6-unknown-linux-gnu", mipsisa32r6_unknown_linux_gnu),
    ("mipsisa32r6el-unknown-linux-gnu", mipsisa32r6el_unknown_linux_gnu),
    ("mipsisa64r6-unknown-linux-gnuabi64", mipsisa64r6_unknown_linux_gnuabi64),
    ("mipsisa64r6el-unknown-linux-gnuabi64", mipsisa64r6el_unknown_linux_gnuabi64),
    ("mipsel-unknown-linux-gnu", mipsel_unknown_linux_gnu),
    ("powerpc-unknown-linux-gnu", powerpc_unknown_linux_gnu),
    ("powerpc-unknown-linux-gnuspe", powerpc_unknown_linux_gnuspe),
    ("powerpc-unknown-linux-musl", powerpc_unknown_linux_musl),
    ("powerpc-unknown-linux-muslspe", powerpc_unknown_linux_muslspe),
    ("powerpc64-ibm-aix", powerpc64_ibm_aix),
    ("powerpc64-unknown-linux-gnu", powerpc64_unknown_linux_gnu),
    ("powerpc64-unknown-linux-musl", powerpc64_unknown_linux_musl),
    ("powerpc64le-unknown-linux-gnu", powerpc64le_unknown_linux_gnu),
    ("powerpc64le-unknown-linux-musl", powerpc64le_unknown_linux_musl),
    ("s390x-unknown-linux-gnu", s390x_unknown_linux_gnu),
    ("s390x-unknown-linux-musl", s390x_unknown_linux_musl),
    ("sparc-unknown-linux-gnu", sparc_unknown_linux_gnu),
    ("sparc64-unknown-linux-gnu", sparc64_unknown_linux_gnu),
    ("arm-unknown-linux-gnueabi", arm_unknown_linux_gnueabi),
    ("arm-unknown-linux-gnueabihf", arm_unknown_linux_gnueabihf),
    ("armeb-unknown-linux-gnueabi", armeb_unknown_linux_gnueabi),
    ("arm-unknown-linux-musleabi", arm_unknown_linux_musleabi),
    ("arm-unknown-linux-musleabihf", arm_unknown_linux_musleabihf),
    ("armv4t-unknown-linux-gnueabi", armv4t_unknown_linux_gnueabi),
    ("armv5te-unknown-linux-gnueabi", armv5te_unknown_linux_gnueabi),
    ("armv5te-unknown-linux-musleabi", armv5te_unknown_linux_musleabi),
    ("armv5te-unknown-linux-uclibceabi", armv5te_unknown_linux_uclibceabi),
    ("armv7-unknown-linux-gnueabi", armv7_unknown_linux_gnueabi),
    ("armv7-unknown-linux-gnueabihf", armv7_unknown_linux_gnueabihf),
    ("thumbv7neon-unknown-linux-gnueabihf", thumbv7neon_unknown_linux_gnueabihf),
    ("thumbv7neon-unknown-linux-musleabihf", thumbv7neon_unknown_linux_musleabihf),
    ("armv7-unknown-linux-musleabi", armv7_unknown_linux_musleabi),
    ("armv7-unknown-linux-musleabihf", armv7_unknown_linux_musleabihf),
    ("aarch64-unknown-linux-gnu", aarch64_unknown_linux_gnu),
    ("aarch64-unknown-linux-musl", aarch64_unknown_linux_musl),
    ("aarch64_be-unknown-linux-musl", aarch64_be_unknown_linux_musl),
    ("x86_64-unknown-linux-musl", x86_64_unknown_linux_musl),
    ("i686-unknown-linux-musl", i686_unknown_linux_musl),
    ("i586-unknown-linux-musl", i586_unknown_linux_musl),
    ("mips-unknown-linux-musl", mips_unknown_linux_musl),
    ("mipsel-unknown-linux-musl", mipsel_unknown_linux_musl),
    ("mips64-unknown-linux-muslabi64", mips64_unknown_linux_muslabi64),
    ("mips64el-unknown-linux-muslabi64", mips64el_unknown_linux_muslabi64),
    ("hexagon-unknown-linux-musl", hexagon_unknown_linux_musl),
    ("hexagon-unknown-none-elf", hexagon_unknown_none_elf),
    ("hexagon-unknown-qurt", hexagon_unknown_qurt),

    ("mips-unknown-linux-uclibc", mips_unknown_linux_uclibc),
    ("mipsel-unknown-linux-uclibc", mipsel_unknown_linux_uclibc),

    ("i686-linux-android", i686_linux_android),
    ("x86_64-linux-android", x86_64_linux_android),
    ("arm-linux-androideabi", arm_linux_androideabi),
    ("armv7-linux-androideabi", armv7_linux_androideabi),
    ("thumbv7neon-linux-androideabi", thumbv7neon_linux_androideabi),
    ("aarch64-linux-android", aarch64_linux_android),
    ("riscv64-linux-android", riscv64_linux_android),

    ("aarch64-unknown-freebsd", aarch64_unknown_freebsd),
    ("armv6-unknown-freebsd", armv6_unknown_freebsd),
    ("armv7-unknown-freebsd", armv7_unknown_freebsd),
    ("i686-unknown-freebsd", i686_unknown_freebsd),
    ("powerpc-unknown-freebsd", powerpc_unknown_freebsd),
    ("powerpc64-unknown-freebsd", powerpc64_unknown_freebsd),
    ("powerpc64le-unknown-freebsd", powerpc64le_unknown_freebsd),
    ("riscv64gc-unknown-freebsd", riscv64gc_unknown_freebsd),
    ("x86_64-unknown-freebsd", x86_64_unknown_freebsd),

    ("x86_64-unknown-dragonfly", x86_64_unknown_dragonfly),

    ("aarch64-unknown-openbsd", aarch64_unknown_openbsd),
    ("i686-unknown-openbsd", i686_unknown_openbsd),
    ("powerpc-unknown-openbsd", powerpc_unknown_openbsd),
    ("powerpc64-unknown-openbsd", powerpc64_unknown_openbsd),
    ("riscv64gc-unknown-openbsd", riscv64gc_unknown_openbsd),
    ("sparc64-unknown-openbsd", sparc64_unknown_openbsd),
    ("x86_64-unknown-openbsd", x86_64_unknown_openbsd),

    ("aarch64-unknown-netbsd", aarch64_unknown_netbsd),
    ("aarch64_be-unknown-netbsd", aarch64_be_unknown_netbsd),
    ("armv6-unknown-netbsd-eabihf", armv6_unknown_netbsd_eabihf),
    ("armv7-unknown-netbsd-eabihf", armv7_unknown_netbsd_eabihf),
    ("i586-unknown-netbsd", i586_unknown_netbsd),
    ("i686-unknown-netbsd", i686_unknown_netbsd),
    ("mipsel-unknown-netbsd", mipsel_unknown_netbsd),
    ("powerpc-unknown-netbsd", powerpc_unknown_netbsd),
    ("riscv64gc-unknown-netbsd", riscv64gc_unknown_netbsd),
    ("sparc64-unknown-netbsd", sparc64_unknown_netbsd),
    ("x86_64-unknown-netbsd", x86_64_unknown_netbsd),

    ("i686-unknown-haiku", i686_unknown_haiku),
    ("x86_64-unknown-haiku", x86_64_unknown_haiku),

    ("aarch64-unknown-helenos", aarch64_unknown_helenos),
    ("i686-unknown-helenos", i686_unknown_helenos),
    ("powerpc-unknown-helenos", powerpc_unknown_helenos),
    ("sparc64-unknown-helenos", sparc64_unknown_helenos),
    ("x86_64-unknown-helenos", x86_64_unknown_helenos),

    ("i686-unknown-hurd-gnu", i686_unknown_hurd_gnu),
    ("x86_64-unknown-hurd-gnu", x86_64_unknown_hurd_gnu),

    ("aarch64-apple-darwin", aarch64_apple_darwin),
    ("arm64e-apple-darwin", arm64e_apple_darwin),
    ("x86_64-apple-darwin", x86_64_apple_darwin),
    ("x86_64h-apple-darwin", x86_64h_apple_darwin),
    ("i686-apple-darwin", i686_apple_darwin),

    ("aarch64-unknown-fuchsia", aarch64_unknown_fuchsia),
    ("riscv64gc-unknown-fuchsia", riscv64gc_unknown_fuchsia),
    ("x86_64-unknown-fuchsia", x86_64_unknown_fuchsia),

    ("avr-none", avr_none),

    ("x86_64-unknown-l4re-uclibc", x86_64_unknown_l4re_uclibc),

    ("aarch64-unknown-redox", aarch64_unknown_redox),
    ("i586-unknown-redox", i586_unknown_redox),
    ("riscv64gc-unknown-redox", riscv64gc_unknown_redox),
    ("x86_64-unknown-redox", x86_64_unknown_redox),

    ("x86_64-unknown-managarm-mlibc", x86_64_unknown_managarm_mlibc),
    ("aarch64-unknown-managarm-mlibc", aarch64_unknown_managarm_mlibc),
    ("riscv64gc-unknown-managarm-mlibc", riscv64gc_unknown_managarm_mlibc),

    ("i386-apple-ios", i386_apple_ios),
    ("x86_64-apple-ios", x86_64_apple_ios),
    ("aarch64-apple-ios", aarch64_apple_ios),
    ("arm64e-apple-ios", arm64e_apple_ios),
    ("armv7s-apple-ios", armv7s_apple_ios),
    ("x86_64-apple-ios-macabi", x86_64_apple_ios_macabi),
    ("aarch64-apple-ios-macabi", aarch64_apple_ios_macabi),
    ("aarch64-apple-ios-sim", aarch64_apple_ios_sim),

    ("aarch64-apple-tvos", aarch64_apple_tvos),
    ("aarch64-apple-tvos-sim", aarch64_apple_tvos_sim),
    ("arm64e-apple-tvos", arm64e_apple_tvos),
    ("x86_64-apple-tvos", x86_64_apple_tvos),

    ("armv7k-apple-watchos", armv7k_apple_watchos),
    ("arm64_32-apple-watchos", arm64_32_apple_watchos),
    ("x86_64-apple-watchos-sim", x86_64_apple_watchos_sim),
    ("aarch64-apple-watchos", aarch64_apple_watchos),
    ("aarch64-apple-watchos-sim", aarch64_apple_watchos_sim),

    ("aarch64-apple-visionos", aarch64_apple_visionos),
    ("aarch64-apple-visionos-sim", aarch64_apple_visionos_sim),

    ("armebv7r-none-eabi", armebv7r_none_eabi),
    ("armebv7r-none-eabihf", armebv7r_none_eabihf),
    ("armv7r-none-eabi", armv7r_none_eabi),
    ("armv7r-none-eabihf", armv7r_none_eabihf),
    ("armv8r-none-eabihf", armv8r_none_eabihf),

    ("armv7-rtems-eabihf", armv7_rtems_eabihf),

    ("x86_64-pc-solaris", x86_64_pc_solaris),
    ("sparcv9-sun-solaris", sparcv9_sun_solaris),

    ("x86_64-unknown-illumos", x86_64_unknown_illumos),
    ("aarch64-unknown-illumos", aarch64_unknown_illumos),

    ("x86_64-pc-windows-gnu", x86_64_pc_windows_gnu),
    ("x86_64-uwp-windows-gnu", x86_64_uwp_windows_gnu),
    ("x86_64-win7-windows-gnu", x86_64_win7_windows_gnu),
    ("i686-pc-windows-gnu", i686_pc_windows_gnu),
    ("i686-uwp-windows-gnu", i686_uwp_windows_gnu),
    ("i686-win7-windows-gnu", i686_win7_windows_gnu),

    ("aarch64-pc-windows-gnullvm", aarch64_pc_windows_gnullvm),
    ("i686-pc-windows-gnullvm", i686_pc_windows_gnullvm),
    ("x86_64-pc-windows-gnullvm", x86_64_pc_windows_gnullvm),

    ("aarch64-pc-windows-msvc", aarch64_pc_windows_msvc),
    ("aarch64-uwp-windows-msvc", aarch64_uwp_windows_msvc),
    ("arm64ec-pc-windows-msvc", arm64ec_pc_windows_msvc),
    ("x86_64-pc-windows-msvc", x86_64_pc_windows_msvc),
    ("x86_64-uwp-windows-msvc", x86_64_uwp_windows_msvc),
    ("x86_64-win7-windows-msvc", x86_64_win7_windows_msvc),
    ("i686-pc-windows-msvc", i686_pc_windows_msvc),
    ("i686-uwp-windows-msvc", i686_uwp_windows_msvc),
    ("i686-win7-windows-msvc", i686_win7_windows_msvc),
    ("thumbv7a-pc-windows-msvc", thumbv7a_pc_windows_msvc),
    ("thumbv7a-uwp-windows-msvc", thumbv7a_uwp_windows_msvc),

    ("wasm32-unknown-emscripten", wasm32_unknown_emscripten),
    ("wasm32-unknown-unknown", wasm32_unknown_unknown),
    ("wasm32v1-none", wasm32v1_none),
    ("wasm32-wasip1", wasm32_wasip1),
    ("wasm32-wasip2", wasm32_wasip2),
    ("wasm32-wasip3", wasm32_wasip3),
    ("wasm32-wasip1-threads", wasm32_wasip1_threads),
    ("wasm32-wali-linux-musl", wasm32_wali_linux_musl),
    ("wasm64-unknown-unknown", wasm64_unknown_unknown),

    ("thumbv6m-none-eabi", thumbv6m_none_eabi),
    ("thumbv7m-none-eabi", thumbv7m_none_eabi),
    ("thumbv7em-none-eabi", thumbv7em_none_eabi),
    ("thumbv7em-none-eabihf", thumbv7em_none_eabihf),
    ("thumbv8m.base-none-eabi", thumbv8m_base_none_eabi),
    ("thumbv8m.main-none-eabi", thumbv8m_main_none_eabi),
    ("thumbv8m.main-none-eabihf", thumbv8m_main_none_eabihf),

    ("armv7a-none-eabi", armv7a_none_eabi),
    ("armv7a-none-eabihf", armv7a_none_eabihf),
    ("armv7a-nuttx-eabi", armv7a_nuttx_eabi),
    ("armv7a-nuttx-eabihf", armv7a_nuttx_eabihf),
    ("armv7a-vex-v5", armv7a_vex_v5),

    ("msp430-none-elf", msp430_none_elf),

    ("aarch64_be-unknown-hermit", aarch64_be_unknown_hermit),
    ("aarch64-unknown-hermit", aarch64_unknown_hermit),
    ("riscv64gc-unknown-hermit", riscv64gc_unknown_hermit),
    ("x86_64-unknown-hermit", x86_64_unknown_hermit),
    ("x86_64-unknown-motor", x86_64_unknown_motor),

    ("x86_64-unikraft-linux-musl", x86_64_unikraft_linux_musl),

    ("armv7-unknown-trusty", armv7_unknown_trusty),
    ("aarch64-unknown-trusty", aarch64_unknown_trusty),
    ("x86_64-unknown-trusty", x86_64_unknown_trusty),

    ("riscv32i-unknown-none-elf", riscv32i_unknown_none_elf),
    ("riscv32im-risc0-zkvm-elf", riscv32im_risc0_zkvm_elf),
    ("riscv32im-unknown-none-elf", riscv32im_unknown_none_elf),
    ("riscv32ima-unknown-none-elf", riscv32ima_unknown_none_elf),
    ("riscv32imc-unknown-none-elf", riscv32imc_unknown_none_elf),
    ("riscv32imc-esp-espidf", riscv32imc_esp_espidf),
    ("riscv32imac-esp-espidf", riscv32imac_esp_espidf),
    ("riscv32imafc-esp-espidf", riscv32imafc_esp_espidf),

    ("riscv32e-unknown-none-elf", riscv32e_unknown_none_elf),
    ("riscv32em-unknown-none-elf", riscv32em_unknown_none_elf),
    ("riscv32emc-unknown-none-elf", riscv32emc_unknown_none_elf),

    ("riscv32imac-unknown-none-elf", riscv32imac_unknown_none_elf),
    ("riscv32imafc-unknown-none-elf", riscv32imafc_unknown_none_elf),
    ("riscv32imac-unknown-xous-elf", riscv32imac_unknown_xous_elf),
    ("riscv32gc-unknown-linux-gnu", riscv32gc_unknown_linux_gnu),
    ("riscv32gc-unknown-linux-musl", riscv32gc_unknown_linux_musl),
    ("riscv64imac-unknown-none-elf", riscv64imac_unknown_none_elf),
    ("riscv64gc-unknown-none-elf", riscv64gc_unknown_none_elf),
    ("riscv64gc-unknown-linux-gnu", riscv64gc_unknown_linux_gnu),
    ("riscv64gc-unknown-linux-musl", riscv64gc_unknown_linux_musl),
    ("riscv64a23-unknown-linux-gnu", riscv64a23_unknown_linux_gnu),

    ("sparc-unknown-none-elf", sparc_unknown_none_elf),

    ("loongarch32-unknown-none", loongarch32_unknown_none),
    ("loongarch32-unknown-none-softfloat", loongarch32_unknown_none_softfloat),
    ("loongarch64-unknown-none", loongarch64_unknown_none),
    ("loongarch64-unknown-none-softfloat", loongarch64_unknown_none_softfloat),

    ("aarch64-unknown-none", aarch64_unknown_none),
    ("aarch64-unknown-none-softfloat", aarch64_unknown_none_softfloat),
    ("aarch64_be-unknown-none-softfloat", aarch64_be_unknown_none_softfloat),
    ("aarch64-unknown-nuttx", aarch64_unknown_nuttx),

    ("x86_64-fortanix-unknown-sgx", x86_64_fortanix_unknown_sgx),

    ("x86_64-unknown-uefi", x86_64_unknown_uefi),
    ("i686-unknown-uefi", i686_unknown_uefi),
    ("aarch64-unknown-uefi", aarch64_unknown_uefi),

    ("nvptx64-nvidia-cuda", nvptx64_nvidia_cuda),

    ("amdgcn-amd-amdhsa", amdgcn_amd_amdhsa),

    ("xtensa-esp32-none-elf", xtensa_esp32_none_elf),
    ("xtensa-esp32-espidf", xtensa_esp32_espidf),
    ("xtensa-esp32s2-none-elf", xtensa_esp32s2_none_elf),
    ("xtensa-esp32s2-espidf", xtensa_esp32s2_espidf),
    ("xtensa-esp32s3-none-elf", xtensa_esp32s3_none_elf),
    ("xtensa-esp32s3-espidf", xtensa_esp32s3_espidf),

    ("i686-wrs-vxworks", i686_wrs_vxworks),
    ("x86_64-wrs-vxworks", x86_64_wrs_vxworks),
    ("armv7-wrs-vxworks-eabihf", armv7_wrs_vxworks_eabihf),
    ("aarch64-wrs-vxworks", aarch64_wrs_vxworks),
    ("powerpc-wrs-vxworks", powerpc_wrs_vxworks),
    ("powerpc-wrs-vxworks-spe", powerpc_wrs_vxworks_spe),
    ("powerpc64-wrs-vxworks", powerpc64_wrs_vxworks),
    ("riscv32-wrs-vxworks", riscv32_wrs_vxworks),
    ("riscv64-wrs-vxworks", riscv64_wrs_vxworks),

    ("aarch64-kmc-solid_asp3", aarch64_kmc_solid_asp3),
    ("armv7a-kmc-solid_asp3-eabi", armv7a_kmc_solid_asp3_eabi),
    ("armv7a-kmc-solid_asp3-eabihf", armv7a_kmc_solid_asp3_eabihf),

    ("mipsel-sony-psp", mipsel_sony_psp),
    ("mipsel-sony-psx", mipsel_sony_psx),
    ("mipsel-unknown-none", mipsel_unknown_none),
    ("mips-mti-none-elf", mips_mti_none_elf),
    ("mipsel-mti-none-elf", mipsel_mti_none_elf),
    ("thumbv4t-none-eabi", thumbv4t_none_eabi),
    ("armv4t-none-eabi", armv4t_none_eabi),
    ("thumbv5te-none-eabi", thumbv5te_none_eabi),
    ("armv5te-none-eabi", armv5te_none_eabi),

    ("aarch64_be-unknown-linux-gnu", aarch64_be_unknown_linux_gnu),
    ("aarch64-unknown-linux-gnu_ilp32", aarch64_unknown_linux_gnu_ilp32),
    ("aarch64_be-unknown-linux-gnu_ilp32", aarch64_be_unknown_linux_gnu_ilp32),

    ("bpfeb-unknown-none", bpfeb_unknown_none),
    ("bpfel-unknown-none", bpfel_unknown_none),

    ("armv6k-nintendo-3ds", armv6k_nintendo_3ds),

    ("aarch64-nintendo-switch-freestanding", aarch64_nintendo_switch_freestanding),

    ("armv7-sony-vita-newlibeabihf", armv7_sony_vita_newlibeabihf),

    ("armv7-unknown-linux-uclibceabi", armv7_unknown_linux_uclibceabi),
    ("armv7-unknown-linux-uclibceabihf", armv7_unknown_linux_uclibceabihf),

    ("x86_64-unknown-none", x86_64_unknown_none),

    ("aarch64-unknown-teeos", aarch64_unknown_teeos),

    ("mips64-openwrt-linux-musl", mips64_openwrt_linux_musl),

    ("aarch64-unknown-nto-qnx700", aarch64_unknown_nto_qnx700),
    ("aarch64-unknown-nto-qnx710", aarch64_unknown_nto_qnx710),
    ("aarch64-unknown-nto-qnx710_iosock", aarch64_unknown_nto_qnx710_iosock),
    ("aarch64-unknown-nto-qnx800", aarch64_unknown_nto_qnx800),
    ("x86_64-pc-nto-qnx710", x86_64_pc_nto_qnx710),
    ("x86_64-pc-nto-qnx710_iosock", x86_64_pc_nto_qnx710_iosock),
    ("x86_64-pc-nto-qnx800", x86_64_pc_nto_qnx800),
    ("i686-pc-nto-qnx700", i686_pc_nto_qnx700),

    ("aarch64-unknown-linux-ohos", aarch64_unknown_linux_ohos),
    ("armv7-unknown-linux-ohos", armv7_unknown_linux_ohos),
    ("loongarch64-unknown-linux-ohos", loongarch64_unknown_linux_ohos),
    ("x86_64-unknown-linux-ohos", x86_64_unknown_linux_ohos),

    ("x86_64-unknown-linux-none", x86_64_unknown_linux_none),

    ("thumbv6m-nuttx-eabi", thumbv6m_nuttx_eabi),
    ("thumbv7a-nuttx-eabi", thumbv7a_nuttx_eabi),
    ("thumbv7a-nuttx-eabihf", thumbv7a_nuttx_eabihf),
    ("thumbv7m-nuttx-eabi", thumbv7m_nuttx_eabi),
    ("thumbv7em-nuttx-eabi", thumbv7em_nuttx_eabi),
    ("thumbv7em-nuttx-eabihf", thumbv7em_nuttx_eabihf),
    ("thumbv8m.base-nuttx-eabi", thumbv8m_base_nuttx_eabi),
    ("thumbv8m.main-nuttx-eabi", thumbv8m_main_nuttx_eabi),
    ("thumbv8m.main-nuttx-eabihf", thumbv8m_main_nuttx_eabihf),
    ("riscv32imc-unknown-nuttx-elf", riscv32imc_unknown_nuttx_elf),
    ("riscv32imac-unknown-nuttx-elf", riscv32imac_unknown_nuttx_elf),
    ("riscv32imafc-unknown-nuttx-elf", riscv32imafc_unknown_nuttx_elf),
    ("riscv64imac-unknown-nuttx-elf", riscv64imac_unknown_nuttx_elf),
    ("riscv64gc-unknown-nuttx-elf", riscv64gc_unknown_nuttx_elf),
    ("x86_64-lynx-lynxos178", x86_64_lynx_lynxos178),

    ("x86_64-pc-cygwin", x86_64_pc_cygwin),
}





// For the [`Target::check_consistency`] function, determines whether the given target is a builtin or a JSON
// target.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TargetKind {
    Json,
    Builtin,
}






