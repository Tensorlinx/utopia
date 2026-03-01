# Makefile for Utopia OS

.PHONY: all build run run-uefi clean test run-limine build-limine

# Default target
all: build

# Build the kernel (default with bootloader_api)
build:
	cargo build --target x86_64-unknown-none -p utopia_kernel

# Build the kernel with limine support
build-kernel-limine:
	cargo build --target x86_64-unknown-none -p utopia_kernel --no-default-features --features limine

# Build and run in QEMU (BIOS mode - default)
run:
	cargo run -p utopia_bootloader --bin utopia_bootloader

# Build and run in QEMU (UEFI mode)
run-uefi:
	cargo run -p utopia_bootloader --bin utopia_bootloader_uefi --features uefi_mode

# Build and run with Limine bootloader
run-limine:
	cargo run -p utopia_limine --bin utopia_limine

# Build limine bootloader
build-limine:
	cargo build -p utopia_limine --bin utopia_limine

# Build limine ISO only
build-limine-iso:
	cargo build -p utopia_limine --bin utopia_limine

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

# Build bootloader (BIOS)
build-bootloader:
	cargo build -p utopia_bootloader --bin utopia_bootloader

# Build bootloader (UEFI)
build-bootloader-uefi:
	cargo build -p utopia_bootloader --bin utopia_bootloader_uefi --features uefi_mode

# Help
help:
	@echo "Utopia OS Build System"
	@echo "======================"
	@echo ""
	@echo "Available targets:"
	@echo "  make build              - Build kernel with default bootloader_api"
	@echo "  make run                - Run with bootloader_api (BIOS)"
	@echo "  make run-uefi           - Run with bootloader_api (UEFI)"
	@echo "  make build-limine       - Build with Limine bootloader"
	@echo "  make run-limine         - Run with Limine bootloader"
	@echo "  make test               - Run tests"
	@echo "  make clean              - Clean build artifacts"
	@echo "  make install-tools      - Install required Rust tools"
