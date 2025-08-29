use std::path::PathBuf;
use cargo::core::Workspace;
use cargo::GlobalContext;
use cargo::util::CargoResult;

pub fn init_workspace<'gctx>(gctx: &'gctx GlobalContext, rust_root: &PathBuf) -> CargoResult<Workspace<'gctx>> {
    let manifest_path = rust_root.join("Cargo.toml");
    println!("init_workspace: manifest_path: {:?}", manifest_path);
    Workspace::new(&manifest_path, gctx)
}
