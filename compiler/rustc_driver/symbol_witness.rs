use std::collections::HashMap;
use serde_json;

macro_rules! witness_symbol {
    ($symbol:expr, $origin:expr, $resolution_type:expr) => {
        eprintln!("WITNESS: {{\"symbol\":\"{}\",\"origin\":\"{}\",\"type\":\"{}\"}}", $symbol, $origin, $resolution_type);
    };
}

macro_rules! witness_resolution {
    ($from:expr, $to:expr) => {
        eprintln!("RESOLUTION: {{\"from\":\"{}\",\"to\":\"{}\"}}", $from, $to);
    };
}

pub fn init_witness() {
    eprintln!("WITNESS_START: {{\"timestamp\":\"{}\"}}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
}

pub fn finalize_witness() {
    eprintln!("WITNESS_END: {{\"timestamp\":\"{}\"}}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
}
