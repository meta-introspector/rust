fn main() {
    println!("cargo:rustc-cfg=llvm_component=\"aarch64\"");
    println!("cargo:rustc-cfg=llvm_component=\"ipo\"");
    println!("cargo:rustc-cfg=llvm_component=\"bitreader\"");
    println!("cargo:rustc-cfg=llvm_component=\"bitwriter\"");
    println!("cargo:rustc-cfg=llvm_component=\"linker\"");
    println!("cargo:rustc-cfg=llvm_component=\"asmparser\"");
    println!("cargo:rustc-cfg=llvm_component=\"lto\"");
    println!("cargo:rustc-cfg=llvm_component=\"coverage\"");
    println!("cargo:rustc-cfg=llvm_component=\"instrumentation\"");
}