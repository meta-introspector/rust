#!/usr/bin/env bash

LOG_FILE="nix_develop_verbose.log"
FLAKE_URL="/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src"
FLAKE_TARGET="#devShells.default"

echo "Running nix develop -vvv for ${FLAKE_URL}${FLAKE_TARGET} and logging to ${LOG_FILE}"

if nix develop -vvv "${FLAKE_URL}${FLAKE_TARGET}" 2>&1 | tee "${LOG_FILE}"; then
    echo "nix develop completed successfully. Logs saved to ${LOG_FILE}"
else
    echo "nix develop failed. Check ${LOG_FILE} for details."
fi
