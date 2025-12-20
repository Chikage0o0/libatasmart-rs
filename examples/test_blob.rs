//! Blob 文件测试程序
//!
//! 读取并解析 blob 文件，显示 SMART 信息

use atasmart::{disk_from_blob, AttributeUnit, Error};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("用法: {} <blob文件路径>", args[0]);
        eprintln!(
            "示例: {} assets/blob-examples/FUJITSU_MHY2120BH--0084000D",
            args[0]
        );
        process::exit(1);
    }

    let blob_path = &args[1];

    match run(blob_path) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("错误: {}", e);
            process::exit(1);
        }
    }
}

fn run(blob_path: &str) -> Result<(), Error> {
    println!("正在读取 blob 文件: {}", blob_path);
    println!();

    // 从 blob 文件创建 Disk
    let mut disk = disk_from_blob(blob_path)?;

    println!("=== 设备信息 ===");
    println!("设备类型: {:?}", disk.disk_type());
    println!();

    // 解析并显示 IDENTIFY 数据
    if let Ok(identify) = disk.parse_identify() {
        println!("型号: {}", identify.model);
        println!("序列号: {}", identify.serial);
        println!("固件版本: {}", identify.firmware);
        println!();
    }

    // 检查 SMART 状态
    match disk.smart_status() {
        Ok(status) => {
            println!("SMART 状态: {}", if status { "良好" } else { "警告" });
        }
        Err(e) => {
            println!("SMART 状态不可用: {}", e);
        }
    }
    println!();

    // 解析并显示 SMART 数据
    if let Ok(smart) = disk.parse_smart() {
        println!("=== SMART 数据 ===");
        println!(
            "离线数据收集状态: {}",
            smart.offline_data_collection_status.as_str()
        );
        println!(
            "离线数据收集时间: {} 秒",
            smart.total_offline_data_collection_seconds
        );
        println!(
            "自检执行状态: {}",
            smart.self_test_execution_status.as_str()
        );
        println!("自检剩余: {}%", smart.self_test_execution_percent_remaining);
        println!();

        // 显示自检可用性
        println!(
            "传输自检可用: {}",
            if smart.conveyance_test_available {
                "yes"
            } else {
                "no"
            }
        );
        println!(
            "短时/扩展自检可用: {}",
            if smart.short_and_extended_test_available {
                "yes"
            } else {
                "no"
            }
        );
        println!(
            "启动自检可用: {}",
            if smart.start_test_available {
                "yes"
            } else {
                "no"
            }
        );
        println!(
            "中止自检可用: {}",
            if smart.abort_test_available {
                "yes"
            } else {
                "no"
            }
        );
        println!();

        // 显示自检轮询时间
        if smart.short_and_extended_test_available {
            println!("短时自检时间: {} 分钟", smart.short_test_polling_minutes);
            println!("扩展自检时间: {} 分钟", smart.extended_test_polling_minutes);
        }

        if smart.conveyance_test_available {
            println!(
                "传输自检时间: {} 分钟",
                smart.conveyance_test_polling_minutes
            );
        }
        println!();
    }

    // 显示统计信息
    println!("=== 统计信息 ===");

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
            format!("{:.1} years", years)
        } else if months >= 1.0 {
            format!("{:.1} months", months)
        } else if days >= 1.0 {
            format!("{:.1} days", days)
        } else {
            format!("{:.1} hours", hours)
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
                println!("平均每次开机时间: {:.1} h", avg_hours);
            }
        }
    }

    // 温度
    if let Ok(mkelvin) = disk.smart_get_temperature() {
        let celsius = (mkelvin as f64 - 273150.0) / 1000.0;
        println!("温度: {:.1} C", celsius);
    }

    println!();

    // 解析并显示 SMART 属性
    if let Ok(attributes) = disk.parse_smart_attributes() {
        println!("=== SMART 属性 ===");
        println!(
            "{:<4} {:<30} {:>6} {:>6} {:>6} {:>15} {:>10}",
            "ID", "名称", "当前", "最差", "阈值", "格式化值", "单位"
        );
        println!("{}", "-".repeat(90));

        for attr in &attributes {
            let pretty_str = format_pretty_value(attr.pretty_value, attr.pretty_unit);
            let unit_str = format_unit(attr.pretty_unit);

            let warn_marker = if attr.warn { " ⚠" } else { "" };

            println!(
                "{:<4} {:<30} {:>6} {:>6} {:>6} {:>15} {:>10}{}",
                attr.id,
                attr.name,
                attr.current_value,
                attr.worst_value,
                if attr.threshold_valid {
                    attr.threshold.to_string()
                } else {
                    "N/A".to_string()
                },
                pretty_str,
                unit_str,
                warn_marker
            );
        }
        println!();
        println!("总计 {} 个属性", attributes.len());
    }

    Ok(())
}

/// 格式化 pretty value
fn format_pretty_value(value: u64, unit: AttributeUnit) -> String {
    match unit {
        AttributeUnit::MilliKelvin => {
            // 转换为摄氏度
            let celsius = (value as f64 - 273150.0) / 1000.0;
            format!("{:.1}°C", celsius)
        }
        AttributeUnit::Milliseconds => {
            // 转换为小时
            let hours = value as f64 / (1000.0 * 60.0 * 60.0);
            if hours < 24.0 {
                format!("{:.1}h", hours)
            } else {
                let days = hours / 24.0;
                if days < 365.0 {
                    format!("{:.1}d", days)
                } else {
                    format!("{:.1}y", days / 365.0)
                }
            }
        }
        AttributeUnit::Sectors => format!("{}", value),
        AttributeUnit::Percent | AttributeUnit::SmallPercent => format!("{}%", value),
        AttributeUnit::Megabytes => format!("{} MB", value),
        _ => format!("{}", value),
    }
}

/// 格式化单位
fn format_unit(unit: AttributeUnit) -> &'static str {
    match unit {
        AttributeUnit::MilliKelvin => "温度",
        AttributeUnit::Milliseconds => "时间",
        AttributeUnit::Sectors => "扇区",
        AttributeUnit::Percent | AttributeUnit::SmallPercent => "百分比",
        AttributeUnit::Megabytes => "数据量",
        AttributeUnit::None => "",
        AttributeUnit::Unknown => "未知",
    }
}
