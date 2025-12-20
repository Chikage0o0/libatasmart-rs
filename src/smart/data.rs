//! SMART 数据读取

use crate::disk::Disk;
use crate::error::Result;

impl Disk {
    // 注意: SMART 数据读取方法已在 src/disk/device.rs 中实现
    // - read_identify()
    // - read_smart_data()
    // - read_smart_thresholds()
    // - smart_status()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_operations() {
        // 需要真实设备才能测试
    }
}
