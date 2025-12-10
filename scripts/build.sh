#!/bin/bash
set -e

echo "🔨 Building Fast AI Inference Platform..."
echo "Target: 50x faster than TensorFlow"

if ! command -v rustc &> /dev/null; then
    echo "Installing Rust..."
    # curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    # source $HOME/.cargo/env
    echo "Rust not found, please install rust."
    exit 1
fi

echo "Compiling in release mode..."
cargo build --release --features "gpu-vulkan,rest-api,monitoring"

BINARY_SIZE=$(stat -c%s target/release/fast-inference 2>/dev/null || echo 0)
echo "✅ Build complete!"
echo "   Binary size: $(($BINARY_SIZE / 1024 / 1024))MB"
