// Generated wrappers for API compatibility testing
include!(concat!(env!("OUT_DIR"), "/main_wrapper.rs"));
include!(concat!(env!("OUT_DIR"), "/plugin_driver_wrapper.rs"));
include!(concat!(env!("OUT_DIR"), "/p2p_server_wrapper.rs"));
include!(concat!(env!("OUT_DIR"), "/ty_wrapper.rs"));

pub mod char_analyzer;
pub mod crate_loader;
pub mod hierarchical_extractor;
pub mod periodic_table;
pub mod spectral_profiler;
pub mod topological_analyzer;

pub use self::rustc_driver::*;
pub use self::rustc_interface::*;
pub use char_analyzer::CharLevelAnalyzer;
pub use crate_loader::ZombieSOSystem;
pub use hierarchical_extractor::HierarchicalSignatureExtractor;
pub use periodic_table::RustPeriodicTable;
pub use spectral_profiler::ModuleSpectralProfiler;
pub use topological_analyzer::TopologicalCompilationAnalyzer;
