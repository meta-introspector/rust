use std::error::Error;
use std::path::PathBuf;
use crate::BuildState;

pub struct Builder<'a> {
    build: &'a BuildState,
}

impl<'a> Builder<'a> {
    pub fn new(build: &'a BuildState) -> Builder<'a> {
        Builder { build }
    }

    pub fn build(&self) -> Result<(), Box<dyn Error>> {
        println!("Building with the bootstrap compiler...");

        // TODO: Implement the build logic here

        Ok(())
    }

    pub fn bootstrap_binary(&self) -> PathBuf {
        self.build.creation_args.build_dir.join("bootstrap").join("debug").join("bootstrap")
    }
}
