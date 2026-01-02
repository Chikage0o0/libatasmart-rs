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
- ✅ 实时设备数据读取 (IDENTIFY、SMART数据、SMART阈值、健康状态)
- ✅ 设备睡眠模式检查
- ✅ 执行硬盘自检 (短时/扩展/传输/中止)

## 平台支持

目前仅支持 Linux 平台。

## 使用示例

### 从实际设备读取数据

```rust
use libatasmart::Disk;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 打开磁盘设备 (需要root权限)
    let disk = Disk::open("/dev/sda")?;

    // 2. 读取并解析 IDENTIFY 数据
    let identify = disk.read_identify()?.parse()?;
    println!("型号: {}", identify.model);
    println!("序列号: {}", identify.serial);
    
    // 3. 读取 SMART 信息
    let smart = disk.read_smart()?;
    
    // 4. 获取统计信息
    let stats = smart.statistics();
    if let Some(temp) = stats.temperature {
        println!("温度: {}", temp);  // 自动格式化为 "35.0°C"
    }
    if let Some(bad) = stats.bad_sectors {
        println!("坏扇区: {}", bad);
    }
    
    // 5. 检查健康状态
    if disk.is_healthy()? {
        println!("磁盘健康");
    } else {
        println!("警告: 磁盘可能即将故障!");
    }

    // 6. 解析 SMART 属性
    let attributes = smart.parse_attributes()?;
    for attr in attributes {
        if attr.warn {
            println!("警告: 属性 {} (ID:{}) 异常!", attr.name, attr.id);
        }
    }

    Ok(())
}
```

### 从 Blob 文件读取数据

```rust
use libatasmart::{smart_info_from_blob, identify_from_blob};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取 SMART 信息
    let smart = smart_info_from_blob("tests/blobs/example.blob")?;
    let stats = smart.statistics();
    
    // 读取 IDENTIFY 信息
    let identify = identify_from_blob("tests/blobs/example.blob")?.parse()?;
    println!("型号: {}", identify.model);
    
    Ok(())
}
```
### 从Blob文件读取数据

```rust
use libatasmart::disk_from_blob;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut disk = disk_from_blob("tests/blobs/st3500320as.blob")?;
    
    // Blob文件已包含数据,可直接解析
    let smart_data = disk.parse_smart()?;
    println!("自检状态: {:?}", smart_data.self_test_execution_status);

    Ok(())
}
```

## 命令行工具

```bash
# 编译所有示例程序
cargo build --examples

# 运行 skdump (显示完整SMART信息)
sudo ./target/debug/examples/skdump /dev/sda

# 运行 read_smart (演示实时数据读取)
sudo ./target/debug/examples/read_smart /dev/sda

# 运行 test_blob (测试Blob文件解析)
cargo run --example test_blob assets/blob-examples/FUJITSU_MHY2120BH--0084000D

# 运行 self_test (执行硬盘自检)
sudo ./target/debug/examples/self_test /dev/sda short
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
- [x] 实时设备数据读取 (read_identify、read_smart_data、read_smart_thresholds)
- [x] SMART健康状态查询 (smart_status)
- [x] 设备睡眠模式检查 (check_sleep_mode)
- [x] 完善 `skdump` 和 `read_smart` 示例工具
- [x] 硬盘自检触发功能 (smart_self_test)
- [ ] 完整的测试覆盖和 CI 文档

## 许可证

本项目采用 **MIT** 或 **Apache-2.0** 双协议授权。

## 致谢与参考

本项目基于 Lennart Poettering 的 [libatasmart](http://0pointer.de/blog/projects/being-smart.html) C 库重构而来，核心逻辑和 SMART 数据处理参考了原始实现。

原始 C 代码保存在 `c-original/` 目录中供参考。

