# UTOPIA Operating System Kernel (DEPRECATED)

> ⚠️ **DEPRECATED**: This version is no longer maintained and has been abandoned. The new kernel has been migrated to [xxxx](https://github.com/your-new-repo-link).

[![1752970302233.png](https://i.postimg.cc/Vv79JnMS/1752970302233.png)](https://postimg.cc/8f6f3FZD)
[![License: GPL v2](https://img.shields.io/badge/License-GPL%20v2-blue.svg)](https://www.gnu.org/licenses/old-licenses/gpl-2.0.en.html)
[![Rust](https://img.shields.io/badge/rust-nightly-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-x86__64-lightgrey.svg)](https://en.wikipedia.org/wiki/X86-64)
[![Boot](https://img.shields.io/badge/boot-UEFI%2FBIOS-green.svg)](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface)
[![Status](https://img.shields.io/badge/status-deprecated-red.svg)](https://github.com/)
## 🚀 快速开始

### 使用 Cargo 命令（推荐）

项目已配置了 Cargo 别名，您可以使用以下命令：

#### 构建命令
```bash
# 构建整个项目
cargo build

# 仅构建内核
cargo kernel

# 构建 BIOS 启动器
cargo build -p utopia_bootloader --bin utopia_bootloader

# 构建 UEFI 启动器
cargo build -p utopia_bootloader --bin utopia_bootloader_uefi --features uefi_mode

# 构建内核（Multiboot2 支持，用于 Limine）
cargo build --target x86_64-unknown-none -p utopia_kernel --no-default-features --features multiboot2
```

#### 运行命令
```bash
# 运行内核 - BIOS 模式（默认）
cargo run -p utopia_bootloader --bin utopia_bootloader

# 运行内核 - UEFI 模式
cargo run -p utopia_bootloader --bin utopia_bootloader_uefi --features uefi_mode

# 运行内核 - Limine 启动器（推荐用于调试）
cargo run -p utopia_limine

# 或者使用别名
cargo qemu
cargo debug
cargo dev
cargo start
```

#### Limine 快速启动（推荐）
```bash
# Windows PowerShell
.\build_and_run_limine.ps1

# Linux/macOS/WSL
./build_and_run_limine.sh
```

#### 开发工具
```bash
# 运行测试
cargo test

# 清理构建产物
cargo clean

# 安装开发工具
cargo install-tools
```

### 传统方式（Makefile）

项目仍然支持传统的 Makefile 命令：

```bash
# 构建项目
make build

# 运行内核 - BIOS 模式（默认）
make run

# 运行内核 - UEFI 模式
make run-uefi

# 运行内核 - Limine 启动器
make run-limine

# 清理
make clean
```

## 📁 项目结构

```
utopia/
├── kernel/                      # 内核源代码
│   ├── src/
│   │   ├── main.rs             # 内核主入口
│   │   ├── multiboot2.rs       # Multiboot2 协议支持
│   │   ├── limine_entry.rs     # Limine 启动支持
│   │   ├── serial.rs           # 串口通信
│   │   ├── logging.rs          # 日志系统
│   │   └── ...
│   └── linker.ld               # 链接脚本
├── bootloader/                  # 自定义引导启动器
├── limine/                      # Limine 启动器支持
│   ├── limine.conf             # Limine 配置
│   └── limine-10.8.2-binary/   # Limine 二进制文件
├── development report/          # 开发文档
├── .cargo/
│   └── config.toml             # Cargo 别名配置
├── Cargo.toml                  # Workspace 配置
├── Makefile                    # 传统构建脚本
├── build_and_run_limine.ps1    # Windows 快速启动脚本
├── build_and_run_limine.sh     # Linux/macOS 快速启动脚本
└── LIMINE_BUILD_GUIDE.md       # Limine 构建指南
```

## 🔧 技术栈

- **语言**: Rust (nightly)
- **目标平台**: `x86_64-unknown-none`
- **启动方式**: 
  - BIOS/UEFI 双启动支持（bootloader_api）
  - Limine 启动器（Multiboot2 协议）
- **虚拟化**: QEMU
- **调试**: 串口输出（COM1）

## 📋 可用命令

| 命令 | 功能 | 等价命令 |
|------|------|----------|
| `cargo build` | 构建整个项目 | `make build` |
| `cargo kernel` | 仅构建内核 | - |
| `cargo run -p utopia_bootloader --bin utopia_bootloader` | BIOS 模式运行 | `make run` |
| `cargo run -p utopia_bootloader --bin utopia_bootloader_uefi --features uefi_mode` | UEFI 模式运行 | `make run-uefi` |
| `cargo run -p utopia_limine` | Limine 启动器运行 | `make run-limine` |
| `.\build_and_run_limine.ps1` | 快速构建和运行（Windows） | - |
| `./build_and_run_limine.sh` | 快速构建和运行（Linux/macOS） | - |
| `cargo qemu` | 运行内核（别名） | `make qemu` |
| `cargo debug` | 调试模式运行 | `make debug` |
| `cargo test` | 运行测试 | `make test` |
| `cargo clean` | 清理构建产物 | `make clean` |
| `cargo install-tools` | 安装开发工具 | `make install-tools` |

## 🎯 开发工作流

### 方式 1: 使用 Limine（推荐用于调试）

1. **安装工具链**: `cargo install-tools`
2. **构建和运行**: `.\build_and_run_limine.ps1` (Windows) 或 `./build_and_run_limine.sh` (Linux/macOS)
3. **查看输出**: 串口输出会显示在 QEMU 控制台

### 方式 2: 使用 bootloader_api

1. **安装工具链**: `cargo install-tools`
2. **构建内核**: `cargo kernel`
3. **运行测试**: `cargo test`
4. **启动内核**: `cargo run`

## 🐛 故障排除

### Limine 启动失败

如果遇到 `multiboot2: Failed to open executable` 错误：

1. 确保使用 multiboot2 feature 编译内核：
   ```bash
   cargo build --target x86_64-unknown-none -p utopia_kernel --no-default-features --features multiboot2
   ```

2. 清理并重新构建：
   ```bash
   cargo clean
   .\build_and_run_limine.ps1
   ```

3. 查看详细指南：`LIMINE_BUILD_GUIDE.md`

### WSL 超时问题

如果构建时 WSL 脚本超时，这是正常的。构建脚本会自动回退到原始磁盘镜像创建方式。

## 📝 注意事项

- 需要 Rust nightly 工具链
- QEMU 需要预先安装
- 项目支持三种启动方式：
  1. **bootloader_api** - 默认方式，支持 BIOS/UEFI
  2. **Limine** - 使用 Multiboot2 协议，推荐用于调试
  3. **自定义启动器** - 完全自定义的启动流程
- Limine 二进制文件已包含在 `limine/limine-10.8.2-binary/` 目录
- 串口输出使用 COM1 端口，波特率 115200

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

---

*项目状态: 可构建和运行*
