//! 磁盘操作模块

mod detect;
mod device;

pub(crate) use detect::detect_disk_type;
pub use device::Disk;
