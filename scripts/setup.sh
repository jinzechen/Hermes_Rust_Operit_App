#!/bin/bash
# Download pre-built binaries for Hermes_Rust_Operit_App
# Run: bash scripts/setup.sh

set -e
BIN_DIR="$(dirname "$0")/../bin"
mkdir -p "$BIN_DIR"

echo "=== Downloading obscura (headless browser) ==="
if [ ! -f "$BIN_DIR/obscura.exe" ]; then
    gh release download v0.1.10 -R h4ckf0r0day/obscura -p "*windows*" -D "$BIN_DIR"
    cd "$BIN_DIR" && unzip -o obscura-*.zip && rm obscura-*.zip
    echo "  obscura OK"
else
    echo "  obscura already exists, skip"
fi

echo "=== Building UA Rust (code analyzer) ==="
UA_DIR="$(dirname "$0")/../../Understand_Anything_Rust"
if [ -d "$UA_DIR" ]; then
    cd "$UA_DIR" && cargo build --release
    cp target/release/ua.exe "$BIN_DIR/"
    echo "  ua.exe OK"
else
    echo "  UA Rust not found at $UA_DIR, skip"
fi

echo "=== Done ==="
ls -la "$BIN_DIR"/*.exe 2>/dev/null || echo "  (no binaries yet)"
