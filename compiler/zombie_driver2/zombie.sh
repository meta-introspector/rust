#!/bin/bash
# Zombie Rustc Driver Test Script

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"

cd "$(dirname "$0")"
export LD_LIBRARY_PATH="target/debug:$LD_LIBRARY_PATH"
exec ./target/debug/zombie_rustc_driver "$@"
