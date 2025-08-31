use crate::build_state::creation_args::BuildStateCreationArgs;
#[derive(Clone)]
pub struct BuildState {
    pub creation_args: BuildStateCreationArgs,
    // Add other fields as needed from RustBuild in bootstrap.py
}

impl BuildState {
    pub fn new(creation_args: BuildStateCreationArgs) -> Self {
        let clean = creation_args.clean;
        println!("DEBUG: clean = {}", clean);
        BuildState {
            creation_args,
        }
    }
}
