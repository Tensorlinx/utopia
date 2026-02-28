# Makefile for Utopia OS

.PHONY: all build run clean test

# Default target
all: build

# Build the kernel
build:
	cargo build --target x86_64-unknown-none -p utopia_kernel

# Build and run in QEMU
run:
	cargo run -p utopia_bootloader

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

# Run with QEMU directly (using bootloader project)
qemu:
	cargo run -p utopia_bootloader

# Debug with GDB
debug:
	cargo run -p utopia_bootloader