# UTOPIA Operating System Kernel (DEPRECATED)

> ⚠️ **DEPRECATED**: This version is no longer maintained and has been abandoned. The new kernel has been migrated to [utopia2026](https://github.com/UtopiaKernel/utopia_2026).

[![1752970302233.png](https://i.postimg.cc/Vv79JnMS/1752970302233.png)](https://postimg.cc/8f6f3FZD)
[![License: GPL v2](https://img.shields.io/badge/License-GPL%20v2-blue.svg)](https://www.gnu.org/licenses/old-licenses/gpl-2.0.en.html)
[![Rust](https://img.shields.io/badge/rust-nightly-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-x86__64-lightgrey.svg)](https://en.wikipedia.org/wiki/X86-64)
[![Boot](https://img.shields.io/badge/boot-UEFI%2FBIOS-green.svg)](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface)
[![Status](https://img.shields.io/badge/status-deprecated-red.svg)](https://github.com/)

## 🚀 Quick Start

### Using Cargo Commands (Recommended)

The project has configured Cargo aliases. You can use the following commands:

#### Build Commands
```bash
# Build the entire project
cargo build

# Build kernel only
cargo kernel

# Build BIOS bootloader
cargo build -p utopia_bootloader --bin utopia_bootloader

# Build UEFI bootloader
cargo build -p utopia_bootloader --bin utopia_bootloader_uefi --features uefi_mode

# Build kernel (Multiboot2 support, for Limine)
cargo build --target x86_64-unknown-none -p utopia_kernel --no-default-features --features multiboot2
```

#### Run Commands
```bash
# Run kernel - BIOS mode (default)
cargo run -p utopia_bootloader --bin utopia_bootloader

# Run kernel - UEFI mode
cargo run -p utopia_bootloader --bin utopia_bootloader_uefi --features uefi_mode

# Run kernel - Limine bootloader (recommended for debugging)
cargo run -p utopia_limine

# Or use aliases
cargo qemu
cargo debug
cargo dev
cargo start
```

#### Limine Quick Start (Recommended)
```bash
# Windows PowerShell
.\build_and_run_limine.ps1

# Linux/macOS/WSL
./build_and_run_limine.sh
```

#### Development Tools
```bash
# Run tests
cargo test

# Clean build artifacts
cargo clean

# Install development tools
cargo install-tools
```

### Traditional Method (Makefile)

The project still supports traditional Makefile commands:

```bash
# Build project
make build

# Run kernel - BIOS mode (default)
make run

# Run kernel - UEFI mode
make run-uefi

# Run kernel - Limine bootloader
make run-limine

# Clean
make clean
```

## 📁 Project Structure

```
utopia/
├── kernel/                      # Kernel source code
│   ├── src/
│   │   ├── main.rs             # Kernel main entry
│   │   ├── multiboot2.rs       # Multiboot2 protocol support
│   │   ├── limine_entry.rs     # Limine boot support
│   │   ├── serial.rs           # Serial communication
│   │   ├── logging.rs          # Logging system
│   │   └── ...
│   └── linker.ld               # Linker script
├── bootloader/                  # Custom bootloader
├── limine/                      # Limine bootloader support
│   ├── limine.conf             # Limine configuration
│   └── limine-10.8.2-binary/   # Limine binaries
├── development report/          # Development documentation
├── .cargo/
│   └── config.toml             # Cargo alias configuration
├── Cargo.toml                  # Workspace configuration
├── Makefile                    # Traditional build script
├── build_and_run_limine.ps1    # Windows quick start script
├── build_and_run_limine.sh     # Linux/macOS quick start script
└── LIMINE_BUILD_GUIDE.md       # Limine build guide
```

## 🔧 Tech Stack

- **Language**: Rust (nightly)
- **Target Platform**: `x86_64-unknown-none`
- **Boot Methods**: 
  - BIOS/UEFI dual boot support (bootloader_api)
  - Limine bootloader (Multiboot2 protocol)
- **Virtualization**: QEMU
- **Debugging**: Serial output (COM1)

## 📋 Available Commands

| Command | Function | Equivalent Command |
|---------|----------|-------------------|
| `cargo build` | Build entire project | `make build` |
| `cargo kernel` | Build kernel only | - |
| `cargo run -p utopia_bootloader --bin utopia_bootloader` | Run BIOS mode | `make run` |
| `cargo run -p utopia_bootloader --bin utopia_bootloader_uefi --features uefi_mode` | Run UEFI mode | `make run-uefi` |
| `cargo run -p utopia_limine` | Run Limine bootloader | `make run-limine` |
| `.\build_and_run_limine.ps1` | Quick build & run (Windows) | - |
| `./build_and_run_limine.sh` | Quick build & run (Linux/macOS) | - |
| `cargo qemu` | Run kernel (alias) | `make qemu` |
| `cargo debug` | Run in debug mode | `make debug` |
| `cargo test` | Run tests | `make test` |
| `cargo clean` | Clean build artifacts | `make clean` |
| `cargo install-tools` | Install dev tools | `make install-tools` |

## 🎯 Development Workflow

### Method 1: Using Limine (Recommended for Debugging)

1. **Install toolchain**: `cargo install-tools`
2. **Build and run**: `.\build_and_run_limine.ps1` (Windows) or `./build_and_run_limine.sh` (Linux/macOS)
3. **View output**: Serial output will be displayed in QEMU console

### Method 2: Using bootloader_api

1. **Install toolchain**: `cargo install-tools`
2. **Build kernel**: `cargo kernel`
3. **Run tests**: `cargo test`
4. **Launch kernel**: `cargo run`

## 🐛 Troubleshooting

### Limine Boot Failure

If you encounter `multiboot2: Failed to open executable` error:

1. Ensure kernel is compiled with multiboot2 feature:
   ```bash
   cargo build --target x86_64-unknown-none -p utopia_kernel --no-default-features --features multiboot2
   ```

2. Clean and rebuild:
   ```bash
   cargo clean
   .\build_and_run_limine.ps1
   ```

3. See detailed guide: `LIMINE_BUILD_GUIDE.md`

### WSL Timeout Issues

If the build script times out in WSL, this is normal. The build script will automatically fall back to the original disk image creation method.

## 📝 Notes

- Requires Rust nightly toolchain
- QEMU must be pre-installed
- Project supports three boot methods:
  1. **bootloader_api** - Default method, supports BIOS/UEFI
  2. **Limine** - Uses Multiboot2 protocol, recommended for debugging
  3. **Custom bootloader** - Fully custom boot process
- Limine binaries are included in `limine/limine-10.8.2-binary/` directory
- Serial output uses COM1 port, baud rate 115200

## 🤝 Contributing

Issues and Pull Requests are welcome!
