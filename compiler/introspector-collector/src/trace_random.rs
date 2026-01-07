/// Reproducible trace-based random system for self-compilation equivalence
/// All random values are recorded in traces for perfect reproducibility

use std::collections::HashMap;

/// Trace entry for random value generation
#[derive(Debug, Clone)]
pub struct RandomTraceEntry {
    pub call_site: String,
    pub value: f64,
    pub seed: u64,
    pub step: usize,
}

/// Global trace-based random system
pub struct TraceRandom {
    pub seed: u64,
    pub step: usize,
    pub trace: Vec<RandomTraceEntry>,
    pub replay_mode: bool,
    pub replay_index: usize,
}

impl TraceRandom {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            step: 0,
            trace: vec![],
            replay_mode: false,
            replay_index: 0,
        }
    }
    
    pub fn gen_f64(&mut self, call_site: &str) -> f64 {
        if self.replay_mode {
            if self.replay_index < self.trace.len() {
                let entry = &self.trace[self.replay_index];
                self.replay_index += 1;
                return entry.value;
            }
        }
        
        // Simple LCG for reproducibility
        self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        let value = (self.seed as f64) / (u64::MAX as f64);
        
        let entry = RandomTraceEntry {
            call_site: call_site.to_string(),
            value,
            seed: self.seed,
            step: self.step,
        };
        
        self.trace.push(entry);
        self.step += 1;
        value
    }
    
    pub fn enable_replay(&mut self) {
        self.replay_mode = true;
        self.replay_index = 0;
    }
}

/// Global trace random instance
static mut TRACE_RANDOM: Option<TraceRandom> = None;

pub fn init_trace_random(seed: u64) {
    unsafe {
        TRACE_RANDOM = Some(TraceRandom::new(seed));
    }
}

pub fn trace_random_f64(call_site: &str) -> f64 {
    unsafe {
        if let Some(ref mut tr) = TRACE_RANDOM {
            tr.gen_f64(call_site)
        } else {
            init_trace_random(42);
            TRACE_RANDOM.as_mut().unwrap().gen_f64(call_site)
        }
    }
}

/// Macro for traced random generation
#[macro_export]
macro_rules! mkdwim_random {
    ($range:expr) => {{
        let call_site = format!("{}:{}:{}", file!(), line!(), column!());
        crate::trace_random::trace_random_f64(&call_site) * $range
    }};
}
