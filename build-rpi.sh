#!/bin/bash
echo "Building examples for Raspberry Pi Model B..."
echo ""
echo "=> linux"
cargo build --release --example linux --target=arm-unknown-linux-musleabihf
echo "=> monitor"
cargo build --release --example monitor --target=arm-unknown-linux-musleabihf
