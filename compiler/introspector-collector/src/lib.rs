#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_hir;
extern crate rustc_ast;

pub mod data_structures;
pub mod file_manager;
pub mod visitors;
pub mod usage_collector;
pub mod collectors;

//pub use usage_collector::UsageCollector;
pub use data_structures::*;
