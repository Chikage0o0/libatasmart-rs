// test_blob - 测试 Blob 文件解析

use libatasmart::{identify_from_blob, smart_info_from_blob, Error};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("用法: {} <blob文件路径>", args[0]);
        eprintln!("示例: {} tests/blobs/example.blob", args[0]);
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
    println!("正在读取 Blob 文件: {}", blob_path);

    // 读取 IDENTIFY 数据
    println!("\n=== IDENTIFY 数据 ===");
    match identify_from_blob(blob_path) {
        Ok(identify_data) => {
            println!("✓ IDENTIFY 数据读取成功");
            match identify_data.parse() {
                Ok(identify) => {
                    println!("型号: {}", identify.model);
                    println!("序列号: {}", identify.serial);
                    println!("固件版本: {}", identify.firmware);
                }
                Err(e) => {
                    println!("解析 IDENTIFY 数据失败: {}", e);
                }
            }
        }
        Err(e) => {
            println!("读取 IDENTIFY 数据失败: {}", e);
        }
    }

    // 读取 SMART 信息
    println!("\n=== SMART 信息 ===");
    let smart = match smart_info_from_blob(blob_path) {
        Ok(smart) => {
            println!("✓ SMART 数据读取成功");
            smart
        }
        Err(e) => {
            println!("读取 SMART 数据失败: {}", e);
            return Ok(());
        }
    };

    // 解析 SMART 数据
    match smart.data.parse() {
        Ok(smart_data) => {
            println!("\nSMART 总体数据:");
            println!(
                "  离线数据收集状态: {:?}",
                smart_data.offline_data_collection_status
            );
            println!(
                "  自检执行状态: {:?}",
                smart_data.self_test_execution_status
            );
            println!(
                "  自检剩余百分比: {}%",
                smart_data.self_test_execution_percent_remaining
            );

            println!("\n自检功能:");
            println!(
                "  短时/扩展自检: {}",
                if smart_data.short_and_extended_test_available {
                    "可用"
                } else {
                    "不可用"
                }
            );
            println!(
                "  传输自检: {}",
                if smart_data.conveyance_test_available {
                    "可用"
                } else {
                    "不可用"
                }
            );

            if smart_data.short_and_extended_test_available {
                println!(
                    "  短时自检预计时间: {} 分钟",
                    smart_data.short_test_polling_minutes
                );
                println!(
                    "  扩展自检预计时间: {} 分钟",
                    smart_data.extended_test_polling_minutes
                );
            }
        }
        Err(e) => {
            println!("解析 SMART 数据失败: {}", e);
        }
    }

    // 显示统计信息
    println!("\n=== 统计信息 ===");
    let stats = smart.statistics();

    if let Some(bad) = stats.bad_sectors {
        let marker = if bad > 0 { " ⚠" } else { "" };
        println!("坏扇区: {} 扇区{}", bad, marker);
    } else {
        println!("坏扇区: 不可用");
    }

    if let Some(duration) = stats.power_on_duration {
        println!("累计开机时间: {}", duration);
    }

    if let Some(cycles) = stats.power_cycle_count {
        println!("电源循环次数: {}", cycles);

        if let Some(duration) = stats.power_on_duration {
            if cycles > 0 {
                let avg_hours = duration.as_hours() / cycles;
                println!("平均每次开机时间: {} 小时", avg_hours);
            }
        }
    }

    if let Some(temp) = stats.temperature {
        println!("当前温度: {}", temp);
    }

    // 显示 SMART 属性
    println!("\n=== SMART 属性 ===");
    match smart.parse_attributes() {
        Ok(attributes) => {
            println!("找到 {} 个 SMART 属性:\n", attributes.len());
            println!(
                "{:<4} {:<40} {:<10} {:<10} {:<10} {:<10}",
                "ID", "名称", "当前值", "最差值", "阈值", "格式化值"
            );
            println!("{}", "-".repeat(90));

            for attr in attributes.iter() {
                let warn_marker = if attr.warn { " ⚠" } else { "" };
                println!(
                    "{:<4} {:<40} {:<10} {:<10} {:<10} {:<10}{}",
                    attr.id,
                    attr.name,
                    attr.current_value,
                    attr.worst_value,
                    attr.threshold,
                    attr.pretty_value,
                    warn_marker
                );
            }
        }
        Err(e) => {
            println!("解析 SMART 属性失败: {}", e);
        }
    }

    println!("\n✓ 完成");
    Ok(())
}
