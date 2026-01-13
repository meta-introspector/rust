#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"

set -euo pipefail

# Oracle Cloud deployment script for Unified P2P Server
# Run on Oracle Cloud Always Free VM (Ubuntu/ARM or x86)

echo "🌩️ Deploying Unified P2P Server to Oracle Cloud..."

# Update system
sudo apt update && sudo apt upgrade -y

# Install Rust
if ! command -v rustc &> /dev/null; then
    echo "🦀 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
fi

# Install dependencies
sudo apt install -y build-essential pkg-config libssl-dev git

# Clone or copy the project
if [ ! -d "unified-p2p-server" ]; then
    echo "📦 Setting up project..."
    mkdir -p unified-p2p-server
    # Copy your files here or clone from git
fi

cd unified-p2p-server

# Build the server
echo "🔨 Building server..."
cargo build --release --bin unified_p2p_server

# Setup as system service
echo "⚙️ Installing system service..."
sudo ./setup-service.sh

# Configure firewall for Oracle Cloud
echo "🔥 Configuring firewall..."
sudo iptables -I INPUT -p tcp --dport 8080 -j ACCEPT
sudo iptables-save | sudo tee /etc/iptables/rules.v4

# Enable service
sudo systemctl enable unified-p2p-server
sudo systemctl start unified-p2p-server

echo "✅ Deployment complete!"
echo "🌐 Server running on: http://$(curl -s ifconfig.me):8080"
echo "📊 Check status: sudo systemctl status unified-p2p-server"
