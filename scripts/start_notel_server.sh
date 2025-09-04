#!/bin/bash

cd /data/data/com.termux/files/home/storage/github/rustc/vendor/telemetry-server/notel/rust-server
RUST_LOG=debug cargo run -- sqlite &
echo "notel server started in background on port 4318."
