#!/usr/bin/env bash

LOG_FILE="test_iso_test_develop.log"
FLAKE_URL="git+file:///data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/test-rust"
FLAKE_TARGET="#devShells.default"

echo "Running nix develop -vvv for ${FLAKE_URL}${FLAKE_TARGET} and logging to ${LOG_FILE}"

if nix develop -vvv "${FLAKE_URL}${FLAKE_TARGET}" 2>&1 | tee "${LOG_FILE}"; then
    echo "nix develop completed successfully. Logs saved to ${LOG_FILE}"
else
    echo "nix develop failed. Check ${LOG_FILE} for details."
fi