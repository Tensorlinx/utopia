# Makefile for CZXT OS

.PHONY: all build run clean test

# Default target
all: build

# Build the kernel
build:
	cargo build

# Build and run in QEMU
run:
	cargo run

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean

# Install required tools
install-tools:
	rustup component add rust-src
	rustup component add llvm-tools-preview
	cargo install bootimage
	cargo install cargo-xbuild

# Run with QEMU directly
qemu: build
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-kernel.bin -serial stdio

# Debug with GDB
debug: build
	qemu-system-x86_64 -s -S -drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-czxt-os.bin