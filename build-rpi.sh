#!/bin/bash
target=arm-unknown-linux-musleabihf
echo "Building examples for Raspberry Pi ($target)..."
echo ""
echo "=> linux-shtc1"
cargo build --release --example linux-shtc1 --target=$target
echo "=> linux-shtc3"
cargo build --release --example linux-shtc3 --target=$target
echo "=> monitor-shtc3"
cargo build --release --example monitor-shtc3 --target=$target
