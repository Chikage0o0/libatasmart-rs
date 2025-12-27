//! self_test - 硬盘自检示例
//!
//! 演示如何启动硬盘SMART自检

use atasmart::{Disk, Error, SmartSelfTest};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        print_usage(&args[0]);
        process::exit(1);
    }

    let device_path = &args[1];
    let test_type_str = &args[2];

    // 解析测试类型
    let test_type = match test_type_str.to_lowercase().as_str() {
        "short" => SmartSelfTest::Short,
        "extended" => SmartSelfTest::Extended,
        "conveyance" => SmartSelfTest::Conveyance,
        "abort" => SmartSelfTest::Abort,
        _ => {
            eprintln!("错误: 未知的测试类型 '{}'", test_type_str);
            eprintln!();
            print_usage(&args[0]);
            process::exit(1);
        }
    };

    match run(device_path, test_type) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("错误: {}", e);
            process::exit(1);
        }
    }
}

fn print_usage(program: &str) {
    eprintln!("用法: {} <设备路径> <测试类型>", program);
    eprintln!();
    eprintln!("测试类型:");
    eprintln!("  short       - 短时自检 (通常2分钟)");
    eprintln!("  extended    - 扩展自检 (可能需要数小时)");
    eprintln!("  conveyance  - 传输自检 (快速检测传输损坏)");
    eprintln!("  abort       - 中止正在进行的自检");
    eprintln!();
    eprintln!("示例:");
    eprintln!("  {} /dev/sda short", program);
    eprintln!("  {} /dev/sda extended", program);
    eprintln!();
    eprintln!("注意: 需要root权限才能访问设备");
}

fn run(device_path: &str, test_type: SmartSelfTest) -> Result<(), Error> {
    println!("正在打开设备: {}", device_path);
    let mut disk = Disk::open(device_path)?;

    println!("\n=== 基本信息 ===");
    println!("设备类型: {:?}", disk.disk_type());

    // 读取IDENTIFY数据
    println!("\n=== 读取设备信息 ===");
    disk.read_identify()?;
    println!("✓ IDENTIFY数据读取成功");

    // 解析设备信息
    match disk.parse_identify() {
        Ok(identify) => {
            println!("  型号: {}", identify.model);
            println!("  序列号: {}", identify.serial);
            println!("  固件版本: {}", identify.firmware);
        }
        Err(e) => {
            println!("警告: 解析IDENTIFY数据失败: {}", e);
        }
    }

    // 读取SMART数据
    println!("\n=== 读取SMART数据 ===");
    disk.read_smart_data()?;
    println!("✓ SMART数据读取成功");

    // 检查自检功能可用性
    println!("\n=== 检查自检功能 ===");
    match disk.parse_smart() {
        Ok(smart) => {
            println!(
                "短时/扩展自检可用: {}",
                smart.short_and_extended_test_available
            );
            println!("传输自检可用: {}", smart.conveyance_test_available);
            println!("启动自检可用: {}", smart.start_test_available);
            println!("中止自检可用: {}", smart.abort_test_available);

            // 显示预计时间
            if smart.self_test_available(test_type) {
                let minutes = smart.self_test_polling_minutes(test_type);
                if minutes > 0 {
                    println!("\n{} 自检预计时间: {} 分钟", test_type.as_str(), minutes);
                }
            }
        }
        Err(e) => {
            println!("警告: 解析SMART数据失败: {}", e);
        }
    }

    // 启动自检
    println!("\n=== 启动自检 ===");
    println!("正在启动 {} 自检...", test_type.as_str());

    disk.smart_self_test(test_type)?;

    println!("✓ {} 自检已成功启动!", test_type.as_str());

    // 提示信息
    match test_type {
        SmartSelfTest::Short => {
            println!("\n提示:");
            println!("- 短时自检通常需要2分钟左右");
            println!("- 可以使用 skdump 查看自检进度和结果");
        }
        SmartSelfTest::Extended => {
            println!("\n提示:");
            println!("- 扩展自检可能需要数小时才能完成");
            println!("- 自检会在后台运行,不影响正常使用");
            println!("- 可以使用 skdump 查看自检进度和结果");
        }
        SmartSelfTest::Conveyance => {
            println!("\n提示:");
            println!("- 传输自检用于快速检测传输过程中的损坏");
            println!("- 可以使用 skdump 查看自检进度和结果");
        }
        SmartSelfTest::Abort => {
            println!("\n提示:");
            println!("- 正在进行的自检已被中止");
        }
    }

    Ok(())
}
