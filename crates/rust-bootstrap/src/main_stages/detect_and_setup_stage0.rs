use crate::bootstrap_stages::stage0_detector;
use crate::bootstrap_stages::env_setup;

pub fn detect_and_setup_stage0() -> stage0_detector::Stage0 {
    let stage0 = stage0_detector::Stage0::detect();
    println!("Detected Stage0: {:?}\n", stage0);
    env_setup::setup_build_environment(&stage0);
    stage0
}
