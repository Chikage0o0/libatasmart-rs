//! 枚举类型定义

/// 磁盘类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiskType {
    /// ATA Passthrough 16 字节 SCSI 命令
    AtaPassthrough16,
    /// ATA Passthrough 12 字节 SCSI 命令
    AtaPassthrough12,
    /// Linux IDE 原生接口
    LinuxIde,
    /// Sunplus USB/ATA 桥接
    Sunplus,
    /// JMicron USB/ATA 桥接
    Jmicron,
    /// 从文件读取的数据
    Blob,
    /// 自动检测
    Auto,
    /// 无访问方法
    None,
}

/// SMART 自检类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmartSelfTest {
    /// 短时自检
    Short = 1,
    /// 扩展自检
    Extended = 2,
    /// 传输自检
    Conveyance = 3,
    /// 中止自检
    Abort = 127,
}

/// 离线数据收集状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineDataCollectionStatus {
    /// 从未启动
    Never,
    /// 成功完成
    Success,
    /// 进行中
    InProgress,
    /// 已暂停
    Suspended,
    /// 已中止
    Aborted,
    /// 致命错误
    Fatal,
    /// 未知状态
    Unknown,
}

/// 自检执行状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelfTestExecutionStatus {
    /// 成功或从未运行
    SuccessOrNever = 0,
    /// 被主机中止
    Aborted = 1,
    /// 被中断
    Interrupted = 2,
    /// 致命错误
    Fatal = 3,
    /// 未知错误
    ErrorUnknown = 4,
    /// 电气元件错误
    ErrorElectrical = 5,
    /// 伺服/寻道错误
    ErrorServo = 6,
    /// 读取错误
    ErrorRead = 7,
    /// 处理损坏
    ErrorHandling = 8,
    /// 进行中
    InProgress = 15,
}

/// SMART 属性单位
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttributeUnit {
    /// 未知
    Unknown,
    /// 无单位
    None,
    /// 毫秒
    Milliseconds,
    /// 扇区数
    Sectors,
    /// 毫开尔文 (温度)
    MilliKelvin,
    /// 小百分比 (3 位小数)
    SmallPercent,
    /// 百分比
    Percent,
    /// 兆字节
    Megabytes,
}

/// SMART 整体健康状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmartOverall {
    /// 良好
    Good,
    /// 过去有属性超过阈值
    BadAttributeInThePast,
    /// 存在坏扇区
    BadSector,
    /// 当前有属性超过阈值
    BadAttributeNow,
    /// 存在大量坏扇区
    BadSectorMany,
    /// SMART 自评估为负面
    BadStatus,
}

impl OfflineDataCollectionStatus {
    /// 转换为字符串描述
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Never => "离线数据收集从未启动",
            Self::Success => "离线数据收集成功完成",
            Self::InProgress => "离线活动进行中",
            Self::Suspended => "离线数据收集被主机命令暂停",
            Self::Aborted => "离线数据收集被主机命令中止",
            Self::Fatal => "离线数据收集因致命错误中止",
            Self::Unknown => "未知状态",
        }
    }
}

impl SelfTestExecutionStatus {
    /// 转换为字符串描述
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SuccessOrNever => "上次自检成功完成或从未运行",
            Self::Aborted => "自检被主机中止",
            Self::Interrupted => "自检被硬件或软件重置中断",
            Self::Fatal => "自检期间发生致命错误",
            Self::ErrorUnknown => "上次自检有测试元件失败",
            Self::ErrorElectrical => "上次自检电气元件失败",
            Self::ErrorServo => "上次自检伺服/寻道元件失败",
            Self::ErrorRead => "上次自检读取元件失败",
            Self::ErrorHandling => "上次自检失败,疑似处理损坏",
            Self::InProgress => "自检进行中",
        }
    }
}

impl AttributeUnit {
    /// 转换为字符串描述
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unknown => "未知",
            Self::None => "无",
            Self::Milliseconds => "毫秒",
            Self::Sectors => "扇区",
            Self::MilliKelvin => "毫开尔文",
            Self::SmallPercent => "小百分比",
            Self::Percent => "百分比",
            Self::Megabytes => "MB",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_self_test_values() {
        assert_eq!(SmartSelfTest::Short as u8, 1);
        assert_eq!(SmartSelfTest::Extended as u8, 2);
        assert_eq!(SmartSelfTest::Abort as u8, 127);
    }

    #[test]
    fn test_status_strings() {
        assert!(!OfflineDataCollectionStatus::Success.as_str().is_empty());
        assert!(!SelfTestExecutionStatus::InProgress.as_str().is_empty());
    }
}
