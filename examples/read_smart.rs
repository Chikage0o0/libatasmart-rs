// ! read_smart - 实时读取SMART数据示例
//!
//! 演示如何从物理设备实时读取SMART数据

use atasmart::{Disk, Error};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("用法: {} <设备路径>", args[0]);
        eprintln!("示例: {} /dev/sda", args[0]);
        eprintln!();
        eprintln!("注意: 需要root权限才能访问设备");
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
    let mut disk = Disk::open(device_path)?;

    println!("\n=== 基本信息 ===");
    println!("设备类型: {:?}", disk.disk_type());
    println!(
        "设备大小: {} 字节 ({:.2} GB)",
        disk.size(),
        disk.size() as f64 / 1_000_000_000.0
    );

    // 检查睡眠状态
    println!("\n=== 电源状态 ===");
    match disk.check_sleep_mode() {
        Ok(awake) => {
            println!("设备状态: {}", if awake { "活动/空闲" } else { "睡眠" });
        }
        Err(e) => {
            println!("无法检查睡眠状态: {}", e);
        }
    }

    // 读取IDENTIFY数据
    println!("\n=== 读取IDENTIFY数据 ===");
    disk.read_identify()?;
    println!("✓ IDENTIFY数据读取成功");

    // 解析IDENTIFY数据
    match disk.parse_identify() {
        Ok(identify) => {
            println!("\n设备信息:");
            println!("  型号: {}", identify.model);
            println!("  序列号: {}", identify.serial);
            println!("  固件版本: {}", identify.firmware);
        }
        Err(e) => {
            println!("解析IDENTIFY数据失败: {}", e);
        }
    }

    // 读取SMART数据
    println!("\n=== 读取SMART数据 ===");
    match disk.read_smart_data() {
        Ok(()) => {
            println!("✓ SMART数据读取成功");
        }
        Err(e) => {
            println!("读取SMART数据失败: {}", e);
            return Ok(());
        }
    }

    // 读取SMART阈值
    println!("\n=== 读取SMART阈值 ===");
    match disk.read_smart_thresholds() {
        Ok(()) => {
            println!("✓ SMART阈值读取成功");
        }
        Err(e) => {
            println!("读取SMART阈值失败: {}", e);
        }
    }

    // 获取SMART状态
    println!("\n=== SMART健康状态 ===");
    match disk.smart_status() {
        Ok(good) => {
            if good {
                println!("✓ SMART状态: 良好");
            } else {
                println!("✗ SMART状态: 异常 - 磁盘可能即将故障!");
            }
        }
        Err(e) => {
            println!("获取SMART状态失败: {}", e);
        }
    }

    // 解析SMART属性
    println!("\n=== SMART属性 ===");
    match disk.parse_smart_attributes() {
        Ok(attributes) => {
            println!("找到 {} 个SMART属性:\n", attributes.len());
            println!(
                "{:<4} {:<40} {:<10} {:<10} {:<10}",
                "ID", "名称", "当前值", "最差值", "阈值"
            );
            println!("{}", "-".repeat(80));

            for attr in attributes.iter().take(10) {
                println!(
                    "{:<4} {:<40} {:<10} {:<10} {:<10}",
                    attr.id, attr.name, attr.current_value, attr.worst_value, attr.threshold
                );
            }

            if attributes.len() > 10 {
                println!("\n... 还有 {} 个属性", attributes.len() - 10);
            }
        }
        Err(e) => {
            println!("解析SMART属性失败: {}", e);
        }
    }

    // 解析SMART总体数据
    println!("\n=== SMART总体数据 ===");
    match disk.parse_smart() {
        Ok(smart) => {
            println!(
                "离线数据收集状态: {:?}",
                smart.offline_data_collection_status
            );
            println!("自检执行状态: {:?}", smart.self_test_execution_status);
        }
        Err(e) => {
            println!("解析SMART数据失败: {}", e);
        }
    }

    // 显示统计信息
    println!("\n=== 统计信息 ===");

    // 坏扇区
    match disk.smart_get_bad_sectors() {
        Ok(sectors) => {
            let marker = if sectors > 0 { " ⚠" } else { "" };
            println!("坏扇区: {} 扇区{}", sectors, marker);
        }
        Err(_) => println!("坏扇区: 不可用"),
    }

    // 开机时间
    if let Ok(msec) = disk.smart_get_power_on() {
        let hours = msec as f64 / (1000.0 * 60.0 * 60.0);
        let days = hours / 24.0;
        let months = days / 30.0;
        let years = days / 365.0;

        let time_str = if years >= 1.0 {
            format!("{:.1} 年", years)
        } else if months >= 1.0 {
            format!("{:.1} 月", months)
        } else if days >= 1.0 {
            format!("{:.1} 天", days)
        } else {
            format!("{:.1} 小时", hours)
        };

        println!("累计开机时间: {}", time_str);
    }

    // 电源循环
    if let Ok(cycles) = disk.smart_get_power_cycle() {
        println!("电源循环次数: {}", cycles);

        // 平均每次开机时间
        if let Ok(msec) = disk.smart_get_power_on() {
            if cycles > 0 {
                let avg_hours = (msec as f64 / cycles as f64) / (1000.0 * 60.0 * 60.0);
                println!("平均每次开机时间: {:.1} 小时", avg_hours);
            }
        }
    }

    // 温度
    if let Ok(mkelvin) = disk.smart_get_temperature() {
        let celsius = (mkelvin as f64 - 273150.0) / 1000.0;
        println!("当前温度: {:.1} °C", celsius);
    }

    println!("\n✓ 完成");
    Ok(())
}
