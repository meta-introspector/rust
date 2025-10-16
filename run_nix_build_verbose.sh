#!/usr/bin/env bash

LOG_FILE="nix_build_verbose.log"
FLAKE_DIR="/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/test-rust"

echo "Running nix build -vvv for ${FLAKE_DIR} and logging to ${LOG_FILE}"

if nix build -vvv "${FLAKE_DIR}" 2>&1 | tee "${LOG_FILE}"; then
    echo "nix build completed successfully. Logs saved to ${LOG_FILE}"
else
    echo "nix build failed. Check ${LOG_FILE} for details."
fi
