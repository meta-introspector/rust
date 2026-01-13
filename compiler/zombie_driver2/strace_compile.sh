#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"

# Zombie compile with strace capture

echo "🔍 Running zombie compile with strace..."
strace -f -e trace=openat -o zombie_compile_strace.log ./zombie.sh --no-net hello_zombie.rs --crate-type bin -o hello_zombie 2>&1
echo "📋 Strace saved to: zombie_compile_strace.log"
