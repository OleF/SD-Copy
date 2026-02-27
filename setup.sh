#!/bin/bash

# SD Copy - Setup Script
# This script helps set up the development environment

set -e

echo "🚀 SD Copy - Setup Script"
echo "================================"
echo ""

# Check if Rust is installed
echo "Checking for Rust..."
if command -v cargo &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    echo "✅ Rust is installed: $RUST_VERSION"
else
    echo "❌ Rust is not installed"
    echo ""
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
    echo "✅ Rust installed successfully"
fi

echo ""

# Check if Node.js is installed
echo "Checking for Node.js..."
if command -v node &> /dev/null; then
    NODE_VERSION=$(node --version)
    echo "✅ Node.js is installed: $NODE_VERSION"
else
    echo "❌ Node.js is not installed"
    echo "Please install Node.js from https://nodejs.org/"
    exit 1
fi

echo ""

# Install npm dependencies
echo "Installing npm dependencies..."
npm install
echo "✅ npm dependencies installed"

echo ""
echo "================================"
echo "✅ Setup complete!"
echo ""
echo "To start the app in development mode:"
echo "  npm run tauri:dev"
echo ""
echo "To build for production:"
echo "  npm run tauri:build"
echo ""
echo "See README.md and QUICKSTART.md for more information."

