#!/bin/bash
# Zombie Rustc Driver with sccache and all triples

export RUSTC_WRAPPER=~/.cargo/bin/sccache
export CFG_COMPILER_HOST_TRIPLE=x86_64-unknown-linux-gnu
export LD_LIBRARY_PATH=target/debug:$LD_LIBRARY_PATH

# Build if needed
if [ ! -f target/debug/zombie_rustc_driver ]; then
    echo "🧟 Building zombie driver..."
    cargo build
fi

# Run zombie with all arguments passed through
CFG_COMPILER_HOST_TRIPLE=x86_64-unknown-linux-gnu \
LD_LIBRARY_PATH=target/debug:$LD_LIBRARY_PATH \
RUST_BACKTRACE=full \
./target/debug/zombie_rustc_driver "$@"
