# Utopia OS - Rust 操作系统内核

Utopia 是一个用 Rust 编写的现代操作系统内核项目，支持 VGA 显示、串口通信和日志系统。

## 🚀 快速开始

### 使用 Cargo 命令（推荐）

项目已配置了 Cargo 别名，您可以使用以下命令：

#### 构建命令
```bash
# 构建整个项目
cargo build

# 仅构建内核
cargo kernel

# 仅构建启动器
cargo bootloader
```

#### 运行命令
```bash
# 运行内核（推荐方式）
cargo run

# 或者使用别名
cargo qemu
cargo debug
cargo dev
cargo start
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

# 运行内核
make run

# 清理
make clean
```

## 📁 项目结构

```
utopia/
├── kernel/           # 内核源代码
├── bootloader/       # 引导启动器
├── .cargo/
│   └── config.toml   # Cargo 别名配置
├── Cargo.toml        # Workspace 配置
└── Makefile          # 传统构建脚本
```

## 🔧 技术栈

- **语言**: Rust (nightly)
- **目标平台**: `x86_64-unknown-none`
- **启动方式**: BIOS/UEFI 双启动支持
- **虚拟化**: QEMU

## 📋 可用命令

| 命令 | 功能 | 等价命令 |
|------|------|----------|
| `cargo build` | 构建整个项目 | `make build` |
| `cargo kernel` | 仅构建内核 | - |
| `cargo bootloader` | 仅构建启动器 | - |
| `cargo run` | 运行内核 | `make run` |
| `cargo qemu` | 运行内核（别名） | `make qemu` |
| `cargo debug` | 调试模式运行 | `make debug` |
| `cargo test` | 运行测试 | `make test` |
| `cargo clean` | 清理构建产物 | `make clean` |
| `cargo install-tools` | 安装开发工具 | `make install-tools` |

## 🎯 开发工作流

1. **安装工具链**: `cargo install-tools`
2. **构建内核**: `cargo kernel`
3. **运行测试**: `cargo test`
4. **启动内核**: `cargo run`

## � 注意事项

- 需要 Rust nightly 工具链
- QEMU 需要预先安装
- 项目使用自定义启动器而非 bootimage 工具

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

---

*项目状态: 可构建和运行*