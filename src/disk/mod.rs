//! 磁盘操作模块

mod device;
mod detect;

pub use device::Disk;
pub(crate) use detect::detect_disk_type;
