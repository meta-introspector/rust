
use crate::bootstrap_stages::stage0_detector::Stage0;

pub fn setup_build_environment(stage0: &Stage0) {
    set_rustc_bootstrap::set_rustc_bootstrap();
    set_rustc_stage0::set_rustc_stage0(&stage0.rustc);
    set_cargo_stage0::set_cargo_stage0(&stage0.cargo);
}

pub mod set_rustc_bootstrap;
pub mod set_rustc_stage0;
pub mod set_cargo_stage0;