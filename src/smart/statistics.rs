//! SMART 统计信息提取
//!
//! 从 SMART 属性中提取高级统计信息

use crate::disk::SmartInfo;
use crate::types::{DiskStatistics, Duration, Temperature};

impl SmartInfo {
    /// 获取坏扇区总数
    ///
    /// 包括已重新分配的扇区和待处理的扇区
    pub fn bad_sectors(&self) -> Option<u64> {
        let attributes = self.parse_attributes().ok()?;

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
            (Some(r), Some(p)) => Some(r + p),
            (Some(r), None) => Some(r),
            (None, Some(p)) => Some(p),
            (None, None) => None,
        }
    }

    /// 获取累计开机时间
    pub fn power_on_duration(&self) -> Option<Duration> {
        let attributes = self.parse_attributes().ok()?;

        for attr in attributes {
            if attr.id == 9 && attr.name == "power-on-hours" {
                return Some(Duration::from_millis(attr.pretty_value));
            }
        }

        None
    }

    /// 获取电源循环次数
    pub fn power_cycle_count(&self) -> Option<u64> {
        let attributes = self.parse_attributes().ok()?;

        for attr in attributes {
            if attr.id == 12 && attr.name == "power-cycle-count" {
                return Some(attr.pretty_value);
            }
        }

        None
    }

    /// 获取温度
    pub fn temperature(&self) -> Option<Temperature> {
        let attributes = self.parse_attributes().ok()?;

        // 优先查找常见的温度属性
        for attr in attributes {
            match attr.id {
                194 | 190 | 231 => {
                    // temperature-celsius-2, airflow-temperature-celsius, temperature-celsius
                    if attr.name.contains("temperature") {
                        return Some(Temperature::from_millikelvin(attr.pretty_value));
                    }
                }
                _ => {}
            }
        }

        None
    }

    /// 获取所有统计信息
    pub fn statistics(&self) -> DiskStatistics {
        DiskStatistics {
            bad_sectors: self.bad_sectors(),
            power_on_duration: self.power_on_duration(),
            power_cycle_count: self.power_cycle_count(),
            temperature: self.temperature(),
        }
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
