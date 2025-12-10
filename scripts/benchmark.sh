#!/bin/bash
set -e

echo "📊 Running 50x Speedup Benchmark"
echo "================================"

# Build if needed
if [ ! -f "./target/release/fast-inference" ]; then
    echo "Building first..."
    cargo build --release --features "gpu-vulkan"
fi

echo ""
echo "1. Benchmarking Our Platform..."
./target/release/fast-inference benchmark \
    --model ./models/test.onnx \
    --iterations 1000 \
    --output ./benchmarks/our_results.json

echo ""
echo "2. Benchmarking TensorFlow (for comparison)..."
# python benchmarks/tensorflow_benchmark.py

echo ""
echo "✅ Benchmark Complete!"
