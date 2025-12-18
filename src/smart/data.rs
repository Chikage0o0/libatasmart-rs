//! SMART 数据读取

use crate::disk::Disk;
use crate::error::{Error, Result};
use crate::types::DiskType;

impl Disk {
    /// 读取 SMART 数据
    ///
    /// 从设备读取 SMART 数据并缓存
    pub fn smart_read_data(&mut self) -> Result<()> {
        // TODO: 实现 SMART 数据读取
        // 1. 检查 SMART 是否可用
        // 2. 发送 SMART READ DATA 命令
        // 3. 缓存数据
        Ok(())
    }

    /// 检查 SMART 是否可用
    pub fn smart_is_available(&self) -> Result<bool> {
        // TODO: 检查 IDENTIFY 数据中的 SMART 支持位
        Ok(false)
    }

    /// 获取 SMART 状态
    ///
    /// # 返回
    ///
    /// * `Ok(true)` - SMART 状态良好
    /// * `Ok(false)` - SMART 检测到问题
    pub fn smart_status(&self) -> Result<bool> {
        // 对于 blob 类型，直接返回存储的状态
        if self.disk_type() == DiskType::Blob {
            return self
                .get_smart_status_internal()
                .ok_or_else(|| Error::NotSupported("Blob 数据中没有 SMART 状态".to_string()));
        }

        // TODO: 实现 SMART 状态检查
        // 发送 SMART RETURN STATUS 命令
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_operations() {
        // 需要真实设备才能测试
    }
}
