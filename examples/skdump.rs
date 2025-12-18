//! skdump - SMART 数据转储工具
//!
//! 这是 libatasmart 的示例程序,用于显示磁盘的 SMART 信息

use atasmart::{Disk, Error};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("用法: {} <设备路径>", args[0]);
        eprintln!("示例: {} /dev/sda", args[0]);
        process::exit(1);
    }

    let device_path = &args[1];

    match run(device_path) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("错误: {}", e);
            process::exit(1);
        }
    }
}

fn run(device_path: &str) -> Result<(), Error> {
    println!("正在打开设备: {}", device_path);

    let disk = Disk::open(device_path)?;

    println!("\n=== 设备信息 ===");
    println!("设备类型: {:?}", disk.disk_type());
    println!(
        "设备大小: {} 字节 ({:.2} GB)",
        disk.size(),
        disk.size() as f64 / 2.0f64.powi(30)
    );

    // TODO: 添加更多功能
    // - 读取 IDENTIFY 数据
    // - 读取 SMART 数据
    // - 显示 SMART 属性
    // - 显示健康状态

    println!("\n注意: 完整功能正在开发中");
    println!("当前版本仅显示基本设备信息");

    Ok(())
}
