//! FFI 和 unsafe 封装模块
//!
//! 此模块包含所有与系统调用和 unsafe 操作相关的代码。
//! 所有 unsafe 代码都被封装在此模块中,不对外导出。

pub(crate) mod ata;
pub(crate) mod ioctl;
pub(crate) mod scsi;
