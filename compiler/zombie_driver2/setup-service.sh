#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"

set -euo pipefail

# Setup script for Unified P2P Rust Compilation Server
# Run as root: sudo ./setup-service.sh

echo "🚀 Setting up Unified P2P Server as system service..."

# Create dedicated user and group
if ! id "p2p-rust" &>/dev/null; then
    echo "👤 Creating p2p-rust user..."
    useradd --system --home-dir /opt/unified-p2p-server --create-home --shell /bin/false p2p-rust
fi

# Create service directory
echo "📁 Creating service directory..."
mkdir -p /opt/unified-p2p-server
chown p2p-rust:p2p-rust /opt/unified-p2p-server

# Build the server
echo "🔨 Building unified P2P server..."
cargo build --release --bin unified_p2p_server

# Copy binary to service directory
echo "📦 Installing binary..."
cp target/release/unified_p2p_server /opt/unified-p2p-server/
chown p2p-rust:p2p-rust /opt/unified-p2p-server/unified_p2p_server
chmod +x /opt/unified-p2p-server/unified_p2p_server

# Copy necessary libraries and dependencies
echo "📚 Setting up runtime dependencies..."
mkdir -p /opt/unified-p2p-server/lib
if [ -f "target/release/deps/librustc_driver.so" ]; then
    cp target/release/deps/librustc_driver.so /opt/unified-p2p-server/lib/
    chown p2p-rust:p2p-rust /opt/unified-p2p-server/lib/librustc_driver.so
fi

# Copy configuration
echo "⚙️ Setting up configuration..."
mkdir -p /etc/unified-p2p-server
cp systemd/unified-p2p-server.env /etc/unified-p2p-server/config.env
chown root:p2p-rust /etc/unified-p2p-server/config.env
chmod 640 /etc/unified-p2p-server/config.env

# Install systemd service
echo "⚙️ Installing systemd service..."
cp systemd/unified-p2p-server.service /etc/systemd/system/
systemctl daemon-reload

# Enable and start service
echo "🎯 Enabling and starting service..."
systemctl enable unified-p2p-server.service
systemctl start unified-p2p-server.service

# Show status
echo "📊 Service status:"
systemctl status unified-p2p-server.service --no-pager

echo "✅ Setup complete!"
echo "📋 Useful commands:"
echo "  sudo systemctl status unified-p2p-server    # Check status"
echo "  sudo systemctl restart unified-p2p-server   # Restart service"
echo "  sudo journalctl -u unified-p2p-server -f    # Follow logs"
echo "  sudo systemctl stop unified-p2p-server      # Stop service"
