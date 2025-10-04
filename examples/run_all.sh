#!/bin/bash
# Run all Claude SDK Rust examples

set -e

echo "==============================================="
echo "Claude SDK Rust Examples - Running All"
echo "==============================================="
echo ""

# Build all examples first
echo "Building all examples..."
cargo build --examples
echo "✓ Build complete"
echo ""

# Run each example
examples=("types_demo" "message_parser_demo" "error_handling_demo")

for example in "${examples[@]}"; do
    echo "==============================================="
    echo "Running: $example"
    echo "==============================================="
    cargo run --example "$example" --quiet
    echo ""
    echo "✓ $example complete"
    echo ""
done

echo "==============================================="
echo "All Examples Complete!"
echo "==============================================="
echo ""
echo "Summary:"
echo "  - types_demo: Type definitions and serialization"
echo "  - message_parser_demo: Message parsing from JSON"
echo "  - error_handling_demo: Error types and handling"
echo ""
echo "Total: ${#examples[@]} examples"
