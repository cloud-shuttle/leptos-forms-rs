#!/bin/bash

# WASM Analysis Script
set -e

echo "🔍 Analyzing WASM package..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

# Check if wasm-pack build exists
if [ ! -f "leptos-forms-rs/pkg/leptos_forms_rs_bg.wasm" ]; then
    echo "WASM package not found. Building first..."
    ./scripts/build-wasm.sh
fi

cd leptos-forms-rs/pkg

print_status "Analyzing WASM file..."

# Basic file information
echo "📊 File Information:"
echo "==================="
ls -lh *.wasm *.js *.d.ts 2>/dev/null || true

# WASM analysis with wasm-objdump if available
if command -v wasm-objdump &> /dev/null; then
    print_status "Detailed WASM analysis..."
    wasm-objdump -h leptos_forms_rs_bg.wasm
    echo ""
    wasm-objdump -x leptos_forms_rs_bg.wasm
fi

# Check for wasm-opt optimizations
if command -v wasm-opt &> /dev/null; then
    print_status "Checking optimization opportunities..."
    wasm-opt --help | grep -q "optimization" && echo "wasm-opt available for further optimization"
fi

# Generate size report
echo ""
echo "📈 Size Analysis:"
echo "================="
WASM_SIZE=$(du -b leptos_forms_rs_bg.wasm | cut -f1)
JS_SIZE=$(du -b leptos_forms_rs.js | cut -f1)
TS_SIZE=$(du -b leptos_forms_rs.d.ts | cut -f1)
TOTAL_SIZE=$((WASM_SIZE + JS_SIZE + TS_SIZE))

echo "WASM file: $(numfmt --to=iec $WASM_SIZE)"
echo "JS file:   $(numfmt --to=iec $JS_SIZE)"
echo "TS file:   $(numfmt --to=iec $TS_SIZE)"
echo "Total:     $(numfmt --to=iec $TOTAL_SIZE)"

# Compression analysis
if command -v gzip &> /dev/null; then
    print_status "Compression analysis..."
    GZIP_SIZE=$(gzip -c leptos_forms_rs_bg.wasm | wc -c)
    echo "Gzipped WASM: $(numfmt --to=iec $GZIP_SIZE)"
    echo "Compression ratio: $(echo "scale=2; $GZIP_SIZE * 100 / $WASM_SIZE" | bc)%"
fi

print_success "Analysis completed!"
