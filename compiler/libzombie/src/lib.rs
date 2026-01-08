#![feature(new_zeroed_alloc)]

pub use rustc_span::*;

pub fn zombie_init() {
    println!("Zombie library initialized");
}
