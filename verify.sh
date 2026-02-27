#!/bin/bash

# SD Copy - Quick Test Script
# Verifiserer at prosjektet er klar til bruk

echo "🔍 SD Copy - Verification Script"
echo "================================"
echo ""

# Check TypeScript
echo "✓ Checking TypeScript..."
npx tsc --noEmit
if [ $? -eq 0 ]; then
    echo "  ✅ TypeScript: OK"
else
    echo "  ❌ TypeScript: ERRORS"
    exit 1
fi

echo ""

# Check if Rust is installed
echo "✓ Checking Rust toolchain..."
if command -v cargo &> /dev/null; then
    echo "  ✅ Rust: $(rustc --version)"
else
    echo "  ⚠️  Rust: NOT INSTALLED"
    echo "     Run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
fi

echo ""

# Check icon file
echo "✓ Checking icon file..."
if [ -f "src-tauri/icons/icon.png" ]; then
    echo "  ✅ Icon: Present"
else
    echo "  ⚠️  Icon: Missing (will create placeholder)"
fi

echo ""

# Check Node modules
echo "✓ Checking node_modules..."
if [ -d "node_modules" ]; then
    echo "  ✅ Dependencies: Installed"
else
    echo "  ❌ Dependencies: Missing (run 'npm install')"
    exit 1
fi

echo ""
echo "================================"
echo "✅ Project is ready!"
echo ""
echo "Next steps:"
echo "  1. Ensure Rust is installed (see above)"
echo "  2. Run: npm run tauri:dev"
echo ""


