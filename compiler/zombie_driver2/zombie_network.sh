#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"

# Zombie Network Server

export RUSTC_WRAPPER=~/.cargo/bin/sccache
export CFG_COMPILER_HOST_TRIPLE=x86_64-unknown-linux-gnu
export LD_LIBRARY_PATH=target/debug:$LD_LIBRARY_PATH

echo "🧟 Starting zombie P2P network server..."
echo "🌐 Listening on port 4001"
echo "🧠 Press Ctrl+C to stop"

exec ./target/debug/zombie_rustc_driver --network-mode
