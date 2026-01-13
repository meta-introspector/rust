#!/bin/bash
set -e

echo "Setting up Rust Mathematical Analysis Compiler service..."

# Create runtime directories
mkdir -p /var/run/compiler/secrets
mkdir -p /var/run/compiler/data
mkdir -p /var/log

# Set up environment file
cat > /var/run/compiler/secrets/env << EOF
RUST_LOG=info
COMPILER_DATA_DIR=/var/run/compiler/data
REPORTS_DIR=/var/run/compiler/data/reports
EOF

# Build the Rust binaries
cd /opt/compiler/compiler/zombie_driver2
cargo build --release

# Copy systemd service from services repo
cp /opt/services/systemd/unified-p2p-server.service /etc/systemd/system/

# Enable and start the service
systemctl daemon-reload
systemctl enable unified-p2p-server.service
systemctl start unified-p2p-server.service

echo "✅ Rust Mathematical Analysis Compiler service installed and started"
systemctl status unified-p2p-server.service
