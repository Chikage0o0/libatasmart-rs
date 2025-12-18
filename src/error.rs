//! 错误类型定义

use std::io;

/// libatasmart 错误类型
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// I/O 错误
    #[error("I/O 错误: {0}")]
    Io(#[from] io::Error),

    /// Nix 系统调用错误
    #[error("系统调用错误: {0}")]
    Nix(#[from] nix::Error),

    /// 设备不支持
    #[error("设备不支持此操作: {0}")]
    NotSupported(String),

    /// SMART 不可用
    #[error("SMART 功能不可用")]
    SmartNotAvailable,

    /// 数据无效
    #[error("数据无效或损坏: {0}")]
    InvalidData(String),

    /// 设备处于睡眠状态
    #[error("设备处于睡眠状态")]
    DeviceSleeping,

    /// 数据不存在
    #[error("请求的数据不存在")]
    NoData,
}

/// Result 类型别名
pub type Result<T> = std::result::Result<T, Error>;
