#!/bin/bash
# Modular Rustc Build System

echo "🧬 Building Modular Rustc with Prime Components"
echo "============================================="

# Available configurations:
build_minimal() {
  cargo build --features "prime_2,prime_3,prime_5"
}

build_standard() {
  cargo build --features "prime_2,prime_3,prime_5,prime_7,prime_11,prime_19"
}

build_full() {
  cargo build --features "prime_2,prime_3,prime_5,prime_7,prime_11,prime_13,prime_17,prime_19"
}

# Build based on argument
case "$1" in
  minimal) build_minimal ;;
  standard) build_standard ;;
  full) build_full ;;
  *) echo "Usage: $0 {minimal|standard|full}" ;;
esac
