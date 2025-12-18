//! SMART 统计信息提取
//!
//! 从 SMART 属性中提取高级统计信息

use crate::disk::Disk;
use crate::error::{Error, Result};

impl Disk {
    /// 获取坏扇区总数
    ///
    /// 包括已重新分配的扇区和待处理的扇区
    pub fn smart_get_bad_sectors(&self) -> Result<u64> {
        let attributes = self.parse_smart_attributes()?;

        let mut reallocated = None;
        let mut pending = None;

        for attr in attributes {
            match attr.id {
                5 => reallocated = Some(attr.pretty_value), // reallocated-sector-count
                197 => pending = Some(attr.pretty_value),   // current-pending-sector
                _ => {}
            }
        }

        match (reallocated, pending) {
            (Some(r), Some(p)) => Ok(r + p),
            (Some(r), None) => Ok(r),
            (None, Some(p)) => Ok(p),
            (None, None) => Err(Error::NoData),
        }
    }

    /// 获取累计开机时间（毫秒）
    pub fn smart_get_power_on(&self) -> Result<u64> {
        let attributes = self.parse_smart_attributes()?;

        for attr in attributes {
            if attr.id == 9 && attr.name == "power-on-hours" {
                return Ok(attr.pretty_value);
            }
        }

        Err(Error::NoData)
    }

    /// 获取电源循环次数
    pub fn smart_get_power_cycle(&self) -> Result<u64> {
        let attributes = self.parse_smart_attributes()?;

        for attr in attributes {
            if attr.id == 12 && attr.name == "power-cycle-count" {
                return Ok(attr.pretty_value);
            }
        }

        Err(Error::NoData)
    }

    /// 获取温度（毫开尔文）
    pub fn smart_get_temperature(&self) -> Result<u64> {
        let attributes = self.parse_smart_attributes()?;

        // 优先查找常见的温度属性
        for attr in attributes {
            match attr.id {
                194 | 190 | 231 => {
                    // temperature-celsius-2, airflow-temperature-celsius, temperature-celsius
                    if attr.name.contains("temperature") {
                        return Ok(attr.pretty_value);
                    }
                }
                _ => {}
            }
        }

        Err(Error::NoData)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statistics_methods_exist() {
        // 这些方法应该存在并可以编译
        // 实际测试需要真实的 SMART 数据
    }
}
