use cargo::GlobalContext;
use cargo::util::CargoResult;
use std::env;
use std::path::PathBuf;
use cargo::core::shell::Shell;

pub fn init_global_context() -> CargoResult<GlobalContext> {
    // Hardcoding homedir to bypass potential issues with `home` crate in Termux.
    // This is a known and stable path in the Termux environment.
    let homedir = PathBuf::from("/data/data/com.termux/files/home/.cargo");
    let cwd = env::current_dir()?;
    let gctx = GlobalContext::new(Shell::new(), cwd, homedir);
    println!("GlobalContext homedir: {:?}", gctx.home().as_path_unlocked());
    Ok(gctx)
}