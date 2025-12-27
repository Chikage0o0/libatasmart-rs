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

    println!("\n=== 设备信息 ===");
    println!("设备类型: {:?}", disk.disk_type());
    println!(
        "设备大小: {} 字节 ({:.2} GB)",
        disk.size(),
        disk.size() as f64 / 1_000_000_000.0
    );

    // 读取IDENTIFY数据
    println!("\n正在读取IDENTIFY数据...");
    disk.read_identify()?;

    if let Ok(identify) = disk.parse_identify() {
        println!("\n=== IDENTIFY信息 ===");
        println!("型号: {}", identify.model);
        println!("序列号: {}", identify.serial);
        println!("固件版本: {}", identify.firmware);
    }

    // 读取SMART数据
    println!("\n正在读取SMART数据...");
    match disk.read_smart_data() {
        Ok(()) => {
            println!("✓ SMART数据读取成功");

            // 读取阈值
            let _ = disk.read_smart_thresholds();

            // 显示SMART状态
            if let Ok(status) = disk.smart_status() {
                println!("\n=== SMART状态 ===");
                if status {
                    println!("✓ 磁盘健康状态: 良好");
                } else {
                    println!("✗ 磁盘健康状态: 异常 - 建议备份数据!");
                }
            }

            // 显示SMART总体信息
            if let Ok(smart) = disk.parse_smart() {
                println!("\n=== SMART总体信息 ===");
                println!(
                    "离线数据收集状态: {}",
                    smart.offline_data_collection_status.as_str()
                );
                println!(
                    "离线数据收集总秒数: {}",
                    smart.total_offline_data_collection_seconds
                );

                // 自检状态和进度
                println!("\n--- 自检状态 ---");
                println!("执行状态: {}", smart.self_test_execution_status.as_str());

                // 如果自检正在进行,显示进度
                if smart.self_test_execution_percent_remaining > 0 {
                    let completed = 100 - smart.self_test_execution_percent_remaining;
                    println!(
                        "进度: {}% (剩余 {}%)",
                        completed, smart.self_test_execution_percent_remaining
                    );
                }

                // 显示可用的自检类型和预计时间
                println!("\n--- 可用的自检类型 ---");

                use atasmart::SmartSelfTest;

                if smart.self_test_available(SmartSelfTest::Short) {
                    println!("✓ 短时自检");
                } else {
                    println!("✗ 短时自检 (不可用)");
                }

                if smart.self_test_available(SmartSelfTest::Extended) {
                    println!("✓ 扩展自检");
                } else {
                    println!("✗ 扩展自检 (不可用)");
                }

                if smart.self_test_available(SmartSelfTest::Conveyance) {
                    println!("✓ 传输自检");
                } else {
                    println!("✗ 传输自检 (不可用)");
                }

                if smart.abort_test_available {
                    println!("✓ 中止自检");
                } else {
                    println!("✗ 中止自检 (不可用)");
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

            // 显示SMART属性
            if let Ok(attributes) = disk.parse_smart_attributes() {
                println!("\n=== SMART属性详情 ===");
                println!(
                    "{:<4} {:<40} {:<8} {:<8} {:<8} {:<10}",
                    "ID", "属性名称", "当前值", "最差值", "阈值", "原始值"
                );
                println!("{}", "-".repeat(90));

                for attr in &attributes {
                    println!(
                        "{:<4} {:<40} {:<8} {:<8} {:<8} {:<10}",
                        attr.id,
                        attr.name,
                        attr.current_value,
                        attr.worst_value,
                        attr.threshold,
                        attr.pretty_value
                    );
                }
            }
        }
        Err(e) => {
            println!("读取SMART数据失败: {}", e);
            println!("\n注意: 某些设备可能不支持SMART功能");
        }
    }

    Ok(())
}
