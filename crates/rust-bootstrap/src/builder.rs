use std::error::Error;
use crate::build_state::BuildState;

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
}
