use std::collections::HashMap;
use std::vec::Vec;

mod utils {
    use std::fmt::Display;
    
    pub struct Helper {
        data: Vec<i32>,
    }
    
    impl Helper {
        pub fn new() -> Self {
            Self { data: Vec::new() }
        }
    }
}

fn process_data() -> HashMap<String, i32> {
    let helper = utils::Helper::new();
    HashMap::new()
}
