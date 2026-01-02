#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_doc_comments)]
//! libatasmart - ATA S.M.A.R.T. 硬盘健康监控库
//!
//! 这是 libatasmart C 库的 Rust 重构版本,提供类型安全的 API 用于:
//! - 读取硬盘 SMART 数据
//! - 解析硬盘健康状态
//! - 执行硬盘自检
//!
//! # 示例
//!
//! ```no_run
//! use libatasmart::Disk;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // 打开磁盘设备
//! let disk = Disk::open("/dev/sda")?;
//!
//! // 读取并解析 IDENTIFY 数据
//! let identify = disk.read_identify()?.parse()?;
//! println!("型号: {}", identify.model);
//! println!("序列号: {}", identify.serial);
//!
//! // 读取 SMART 信息
//! let smart = disk.read_smart()?;
//!
//! // 获取统计信息
//! let stats = smart.statistics();
//! if let Some(temp) = stats.temperature {
//!     println!("温度: {}", temp); // 自动格式化为 "25.0°C"
//! }
//! if let Some(bad) = stats.bad_sectors {
//!     println!("坏扇区: {}", bad);
//! }
//!
//! // 检查健康状态
//! if disk.is_healthy()? {
//!     println!("磁盘健康");
//! }
//! # Ok(())
//! # }
//! ```

// 模块声明
mod disk;
mod error;
mod ffi;
mod identify;
mod smart;
mod types;
mod utils;

// 公共导出
pub use disk::{Disk, IdentifyData, SmartData, SmartInfo, SmartThresholds};
pub use error::{Error, Result};
pub use smart::{identify_from_blob, read_blob_from_file, smart_info_from_blob, BlobData};
pub use types::{
    AttributeUnit, DiskStatistics, DiskType, Duration, IdentifyParsedData,
    OfflineDataCollectionStatus, SelfTestExecutionStatus, SmartAttributeParsedData, SmartOverall,
    SmartParsedData, SmartSelfTest, Temperature,
};
