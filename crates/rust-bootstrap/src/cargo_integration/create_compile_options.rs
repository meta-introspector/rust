use cargo::GlobalContext;
use cargo::ops::CompileOptions;
use cargo::util::CargoResult;
use cargo::core::compiler::UserIntent;

pub fn create_compile_options(gctx: &GlobalContext) -> CargoResult<CompileOptions> {
    CompileOptions::new(gctx, UserIntent::Build)
}
