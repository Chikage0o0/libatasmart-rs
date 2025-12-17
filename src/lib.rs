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
//! use atasmart::Disk;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // 打开磁盘设备
//! let disk = Disk::open("/dev/sda")?;
//!
//! // 读取 SMART 数据
//! let smart_data = disk.smart_parse()?;
//! println!("磁盘健康状态: {:?}", smart_data);
//!
//! // 获取整体健康评估
//! let overall = disk.smart_get_overall()?;
//! println!("整体状态: {:?}", overall);
//! # Ok(())
//! # }
//! ```

// 模块声明
mod error;
mod ffi;
mod types;
mod disk;
mod smart;
mod identify;
mod utils;

// 公共导出
pub use error::{Error, Result};
pub use types::{
    DiskType, SmartSelfTest, SmartOverall,
    OfflineDataCollectionStatus, SelfTestExecutionStatus,
    AttributeUnit, IdentifyParsedData, SmartParsedData,
    SmartAttributeParsedData,
};
pub use disk::Disk;
