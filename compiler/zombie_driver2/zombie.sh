#!/bin/bash
# Zombie Rustc Driver Test Script

cd "$(dirname "$0")"
export LD_LIBRARY_PATH="target/debug:$LD_LIBRARY_PATH"
exec ./target/debug/zombie_rustc_driver "$@"
