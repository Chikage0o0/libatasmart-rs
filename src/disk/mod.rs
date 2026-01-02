//! 磁盘操作模块

mod detect;
mod device;
mod identify_data;
mod smart_data;

pub(crate) use detect::detect_disk_type;
pub use device::Disk;
pub use identify_data::IdentifyData;
pub use smart_data::{SmartData, SmartInfo, SmartThresholds};
