
use crate::spec::{Target, TargetOptions};
use crate::spec::target::target_template; // Corrected import

pub fn target() -> Target {
    let mut base = target_template(); // Call directly
    // TODO: Customize target options here
    base.options = TargetOptions {
        // Example:
        // cpu: "cortex-a7".into(),
        // features: "+vfp3".into(),
        ..base.options
    };
    base
}
