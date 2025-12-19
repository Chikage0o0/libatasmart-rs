# libatasmart-rs

ATA S.M.A.R.T. 硬盘健康监控库 (Rust 实现)

这是 [libatasmart](http://0pointer.de/blog/projects/being-smart.html) C 库的 Rust 重构版本。

## 特性

- ✅ 类型安全的 Rust API
- ✅ 所有 unsafe 代码隔离在 FFI 模块中
- ✅ 使用 `libc` 进行底层系统调用，配合标准库错误处理
- ✅ 完整的错误处理
- ✅ SMART 数据结构化解析 (属性、健康状态、离线测试状态)
- ✅ IDENTIFY 数据解析 (型号、序列号、固件版本)
- ✅ 支持从 Blob 文件加载数据进行离线分析
- ✅ 自动磁盘类型检测
- 🚧 实时设备数据读取 (开发中)
- 🚧 执行硬盘自检 (计划中)

## 平台支持

目前仅支持 Linux 平台。

## 使用示例

```rust
use atasmart::Disk;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 打开磁盘设备 (从 Blob 文件读取示例)
    // 实际使用时可以使用 Disk::open("/dev/sda")
    let disk = atasmart::disk_from_blob("tests/blobs/st3500320as.blob")?;

    // 2. 获取基本信息
    println!("设备大小: {} 字节", disk.size());
    
    // 3. 解析 SMART 数据
    let smart_data = disk.parse_smart()?;
    println!("自检状态: {:?}", smart_data.self_test_execution_status);

    // 4. 解析属性
    let attributes = disk.parse_smart_attributes()?;
    for attr in attributes {
        if attr.warn {
            println!("警告: 属性 {} (ID:{}) 异常!", attr.name, attr.id);
        }
    }

    Ok(())
}
```

## 命令行工具

```bash
# 编译示例程序
cargo build --example skdump

# 运行 (需要 root 权限，如果是真实设备)
sudo ./target/debug/examples/skdump /dev/sda

# 运行测试 Blob 解析
cargo run --example test_blob
```

## 开发状态

本项目正在积极开发中。当前已完成:

- [x] 项目结构和模块划分
- [x] 错误处理和类型定义
- [x] FFI 层 unsafe 代码封装 (libc/ioctl)
- [x] SMART 属性全面解析 (包含 256 个已知属性定义)
- [x] 健康状态评估规则 (基于阈值和属性)
- [x] IDENTIFY 数据基本解析
- [x] 支持从 Blob 加载数据用于测试和离线分析
- [x] 自动设备类型检测逻辑 (AtaPassthrough/LinuxIde 等)
- [ ] 实时设备数据抓取 (实现 ioctl 交互逻辑)
- [ ] 完善 `skdump` 示例工具的输出内容
- [ ] 硬盘自检触发功能
- [ ] 完整的测试覆盖和 CI 文档

## 许可证

本项目采用 **MIT** 或 **Apache-2.0** 双协议授权。

## 致谢与参考

本项目基于 Lennart Poettering 的 [libatasmart](http://0pointer.de/blog/projects/being-smart.html) C 库重构而来，核心逻辑和 SMART 数据处理参考了原始实现。

原始 C 代码保存在 `c-original/` 目录中供参考。

