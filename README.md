# UTOPIA 操作系统内核

一个用 Rust 编写的简单操作系统内核项目，专注于学习和实验操作系统开发。

## 🚀 项目特性

### 核心功能
- **现代启动支持**: 支持 UEFI 和 BIOS 启动
- **帧缓冲区图形**: 自定义字体渲染和文本显示
- **串口调试**: 完整的串口通信和日志输出
- **结构化日志**: 基于 `log` crate 的分级日志系统
- **测试框架**: 集成的单元测试和集成测试

### 代码质量改进
- **模块化设计**: 清晰的模块分离和职责划分
- **错误处理**: 完善的错误类型定义和处理机制
- **常量管理**: 集中的常量定义，避免魔数
- **安全编程**: 边界检查和内存安全保证
- **文档完善**: 详细的代码注释和文档

## 📁 项目结构

```
czxt/
├── kernel/                 # 内核代码
│   └── src/
│       ├── main.rs        # 内核入口点
│       ├── vga_buffer.rs  # VGA 显示驱动
│       ├── serial.rs      # 串口通信
│       ├── logging.rs     # 日志系统
│       ├── logo.rs        # 启动 Logo
│       ├── constants.rs   # 常量定义
│       ├── font.rs        # 字体数据
│       └── error.rs       # 错误处理
├── os/                     # 启动器
│   ├── src/main.rs        # 启动器入口
│   └── build.rs           # 构建脚本
├── font_converter/         # 字体转换工具
└── 开发报告/               # 开发文档
```

## 🛠️ 构建和运行

### 前置要求
- Rust 工具链 (nightly)
- QEMU 模拟器
- bootloader 依赖

### 构建命令
```bash
# 构建内核
cargo build

# 运行内核（在 QEMU 中）
cargo run --bin utopia

# 运行测试
cargo test
```

### 开发环境设置
```bash
# 安装 Rust nightly
rustup install nightly
rustup default nightly

# 添加目标平台
rustup target add x86_64-unknown-none

# 安装 QEMU
# Windows: 下载并安装 QEMU
# Linux: sudo apt install qemu-system-x86
# macOS: brew install qemu
```

## 🔧 最新改进

### 代码重构
1. **模块化改进**
   - 创建 `constants.rs` 模块管理所有常量
   - 分离 `font.rs` 模块处理字体数据
   - 新增 `error.rs` 模块统一错误处理

2. **错误处理增强**
   - 定义 `KernelError` 枚举类型
   - 实现 `KernelResult<T>` 类型别名
   - 添加 `SafeWrite` trait 提供安全写入
   - 创建错误处理宏简化代码

3. **安全性提升**
   - 添加边界检查防止缓冲区溢出
   - 实现安全的像素操作函数
   - 提供不会 panic 的安全打印宏

4. **性能优化**
   - 减少重复的字体数据定义
   - 优化像素渲染逻辑
   - 改进内存访问模式

### 新增功能
- `print_safe!` 和 `println_safe!` 宏
- `serial_print_safe!` 和 `serial_println_safe!` 宏
- 字符可打印性检查
- 改进的日志初始化错误处理

## 📚 开发文档

- [串口调试与内核启动问题排查报告](./开发报告/串口调试与内核启动问题排查报告.md)
- [开发流程](./开发流程.md)

## 🎯 未来计划

### 短期目标
- [ ] 实现真正的屏幕滚动功能
- [ ] 添加键盘输入支持
- [ ] 实现基本的内存管理
- [ ] 添加中断处理

### 长期目标
- [ ] 文件系统支持
- [ ] 多任务调度
- [ ] 网络协议栈
- [ ] 用户空间程序

## 🤝 贡献指南

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [Writing an OS in Rust](https://os.phil-opp.com/) - 优秀的 Rust 操作系统开发教程
- [bootloader](https://github.com/rust-osdev/bootloader) - Rust 启动加载器
- Rust 社区的支持和贡献

---

**注意**: 这是一个学习项目，不适用于生产环境。