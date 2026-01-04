#!/bin/bash
# Build mklang! WASM system for browser deployment

echo "🚀 Building MKLANG! WASM Browser Sandbox..."

# Install wasm-pack if needed
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build WASM package
echo "Building WASM package..."
wasm-pack build --target web --out-dir pkg

# Create simple HTTP server
echo "Starting development server..."
echo "Open http://localhost:8000 to see MKLANG! in browser"

# Use Python's built-in server
python3 -m http.server 8000
