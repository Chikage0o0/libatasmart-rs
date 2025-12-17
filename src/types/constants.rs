//! 常量定义

/// 超时时间 (毫秒)
pub const TIMEOUT_MS: u32 = 2000;

/// 温度有效范围 (毫开尔文)
pub const MKELVIN_VALID_MIN: u64 = ((-15i64 * 1000) + 273150) as u64;
pub const MKELVIN_VALID_MAX: u64 = ((100i64 * 1000) + 273150) as u64;

/// 时间有效范围 (毫秒)
pub const MSECOND_VALID_MIN: u64 = 1;
pub const MSECOND_VALID_SHORT_MAX: u64 = 60 * 60 * 1000; // 1 小时
pub const MSECOND_VALID_LONG_MAX: u64 = 30 * 365 * 24 * 60 * 60 * 1000; // 30 年
