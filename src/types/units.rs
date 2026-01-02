//! 单位类型定义
//!
//! 提供类型安全的单位包装器

use std::fmt;

/// 温度 (摄氏度)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Temperature {
    celsius: f64,
}

impl Temperature {
    /// 从摄氏度创建
    pub fn from_celsius(celsius: f64) -> Self {
        Self { celsius }
    }

    /// 从毫开尔文创建
    pub fn from_millikelvin(mk: u64) -> Self {
        let celsius = (mk as f64 - 273150.0) / 1000.0;
        Self { celsius }
    }

    /// 获取摄氏度值
    pub fn celsius(&self) -> f64 {
        self.celsius
    }

    /// 获取华氏度值
    pub fn fahrenheit(&self) -> f64 {
        self.celsius * 9.0 / 5.0 + 32.0
    }

    /// 获取开尔文值
    pub fn kelvin(&self) -> f64 {
        self.celsius + 273.15
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}°C", self.celsius)
    }
}

/// 时长
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Duration {
    milliseconds: u64,
}

impl Duration {
    /// 从毫秒创建
    pub fn from_millis(ms: u64) -> Self {
        Self { milliseconds: ms }
    }

    /// 从小时创建
    pub fn from_hours(hours: u64) -> Self {
        Self {
            milliseconds: hours * 3600 * 1000,
        }
    }

    /// 从分钟创建
    pub fn from_minutes(minutes: u64) -> Self {
        Self {
            milliseconds: minutes * 60 * 1000,
        }
    }

    /// 获取毫秒值
    pub fn as_millis(&self) -> u64 {
        self.milliseconds
    }

    /// 获取秒值
    pub fn as_secs(&self) -> u64 {
        self.milliseconds / 1000
    }

    /// 获取分钟值
    pub fn as_minutes(&self) -> u64 {
        self.milliseconds / (60 * 1000)
    }

    /// 获取小时值
    pub fn as_hours(&self) -> u64 {
        self.milliseconds / (3600 * 1000)
    }

    /// 获取天数 (浮点数)
    pub fn as_days(&self) -> f64 {
        self.milliseconds as f64 / (24.0 * 3600.0 * 1000.0)
    }

    /// 获取年数 (浮点数,按 365 天计算)
    pub fn as_years(&self) -> f64 {
        self.as_days() / 365.0
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours = self.as_hours();
        if hours < 24 {
            write!(f, "{} 小时", hours)
        } else {
            let days = self.as_days();
            if days < 365.0 {
                write!(f, "{:.1} 天", days)
            } else {
                write!(f, "{:.1} 年", self.as_years())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temperature() {
        let temp = Temperature::from_celsius(25.0);
        assert_eq!(temp.celsius(), 25.0);
        assert_eq!(temp.fahrenheit(), 77.0);
        assert_eq!(temp.kelvin(), 298.15);

        let temp2 = Temperature::from_millikelvin(298150);
        assert!((temp2.celsius() - 25.0).abs() < 0.01);
    }

    #[test]
    fn test_duration() {
        let dur = Duration::from_hours(24);
        assert_eq!(dur.as_hours(), 24);
        assert_eq!(dur.as_days(), 1.0);

        let dur2 = Duration::from_millis(3600000);
        assert_eq!(dur2.as_hours(), 1);
    }
}
