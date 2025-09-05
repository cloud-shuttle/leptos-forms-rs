#!/bin/bash

# WASM Build Script with Optimizations
set -e

echo "🚀 Building optimized WASM package for leptos-forms-rs..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    print_error "wasm-pack is not installed. Installing..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Check if wasm-opt is installed
if ! command -v wasm-opt &> /dev/null; then
    print_warning "wasm-opt is not installed. Installing binaryen..."
    if command -v npm &> /dev/null; then
        npm install -g binaryen
    else
        print_warning "npm not found. Please install wasm-opt manually for optimal size reduction."
    fi
fi

# Clean previous builds
print_status "Cleaning previous builds..."
cargo clean
rm -rf leptos-forms-rs/pkg

# Build with wasm-pack
print_status "Building WASM package with wasm-pack..."
cd leptos-forms-rs

# Build for release with optimizations
wasm-pack build --target web --release --out-dir pkg --scope leptos-forms

# Check if wasm-opt is available for further optimization
if command -v wasm-opt &> /dev/null; then
    print_status "Applying wasm-opt optimizations..."

    # Apply aggressive size optimizations
    wasm-opt -Os --enable-bulk-memory --enable-mutable-globals \
        --enable-nontrapping-float-to-int --enable-sign-ext \
        pkg/leptos_forms_rs_bg.wasm -o pkg/leptos_forms_rs_bg.wasm

    print_success "wasm-opt optimizations applied"
else
    print_warning "wasm-opt not available, skipping additional optimizations"
fi

# Get file sizes
WASM_SIZE=$(du -h pkg/leptos_forms_rs_bg.wasm | cut -f1)
JS_SIZE=$(du -h pkg/leptos_forms_rs.js | cut -f1)
TS_SIZE=$(du -h pkg/leptos_forms_rs.d.ts | cut -f1)

print_success "Build completed!"
echo "📦 Package sizes:"
echo "   WASM: $WASM_SIZE"
echo "   JS:   $JS_SIZE"
echo "   TS:   $TS_SIZE"

# Generate bundle analysis if webpack-bundle-analyzer is available
if command -v npx &> /dev/null; then
    print_status "Generating bundle analysis..."
    cd ..
    npx webpack-bundle-analyzer leptos-forms-rs/pkg/leptos_forms_rs_bg.wasm 2>/dev/null || true
fi

print_success "WASM build completed successfully!"
