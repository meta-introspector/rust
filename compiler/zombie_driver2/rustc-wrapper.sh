#!/bin/bash
if [[ "$1" == "-vV" ]]; then
    rustc -vV
else
    exec /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust-build/compiler/zombie_driver2/target/release/zombie-rustc "$@"
fi
