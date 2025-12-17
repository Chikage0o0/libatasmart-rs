//! 数据结构定义

use super::*;

/// IDENTIFY 解析数据
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentifyParsedData {
    /// 序列号
    pub serial: String,
    /// 固件版本
    pub firmware: String,
    /// 型号
    pub model: String,
}

/// SMART 解析数据
#[derive(Debug, Clone)]
pub struct SmartParsedData {
    // 易失性数据
    /// 离线数据收集状态
    pub offline_data_collection_status: OfflineDataCollectionStatus,
    /// 离线数据收集总秒数
    pub total_offline_data_collection_seconds: u32,
    /// 自检执行状态
    pub self_test_execution_status: SelfTestExecutionStatus,
    /// 自检执行剩余百分比
    pub self_test_execution_percent_remaining: u32,

    // 固定数据
    /// 短时和扩展自检可用
    pub short_and_extended_test_available: bool,
    /// 传输自检可用
    pub conveyance_test_available: bool,
    /// 启动自检可用
    pub start_test_available: bool,
    /// 中止自检可用
    pub abort_test_available: bool,

    /// 短时自检轮询分钟数
    pub short_test_polling_minutes: u16,
    /// 扩展自检轮询分钟数
    pub extended_test_polling_minutes: u16,
    /// 传输自检轮询分钟数
    pub conveyance_test_polling_minutes: u16,
}

/// SMART 属性解析数据
#[derive(Debug, Clone)]
pub struct SmartAttributeParsedData {
    // 固定数据
    /// 属性 ID
    pub id: u8,
    /// 属性名称
    pub name: &'static str,
    /// 格式化值的单位
    pub pretty_unit: AttributeUnit,
    /// 标志位
    pub flags: u16,
    /// 阈值
    pub threshold: u8,
    /// 阈值是否有效
    pub threshold_valid: bool,

    /// 是否在线属性
    pub online: bool,
    /// 是否预失败属性
    pub prefailure: bool,

    // 易失性数据
    /// 当前状态良好
    pub good_now: bool,
    /// 当前状态有效
    pub good_now_valid: bool,
    /// 过去状态良好
    pub good_in_the_past: bool,
    /// 过去状态有效
    pub good_in_the_past_valid: bool,
    /// 当前值有效
    pub current_value_valid: bool,
    /// 最差值有效
    pub worst_value_valid: bool,
    /// 是否警告
    pub warn: bool,
    /// 当前值
    pub current_value: u8,
    /// 最差值
    pub worst_value: u8,
    /// 格式化的值
    pub pretty_value: u64,
    /// 原始值 (6 字节)
    pub raw: [u8; 6],
}

impl SmartParsedData {
    /// 检查指定自检是否可用
    pub fn self_test_available(&self, test: SmartSelfTest) -> bool {
        if !self.start_test_available {
            return false;
        }

        match test {
            SmartSelfTest::Short | SmartSelfTest::Extended => {
                self.short_and_extended_test_available
            }
            SmartSelfTest::Conveyance => self.conveyance_test_available,
            SmartSelfTest::Abort => self.abort_test_available,
        }
    }

    /// 获取指定自检的轮询分钟数
    pub fn self_test_polling_minutes(&self, test: SmartSelfTest) -> u16 {
        if !self.self_test_available(test) {
            return 0;
        }

        match test {
            SmartSelfTest::Short => self.short_test_polling_minutes,
            SmartSelfTest::Extended => self.extended_test_polling_minutes,
            SmartSelfTest::Conveyance => self.conveyance_test_polling_minutes,
            SmartSelfTest::Abort => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_parsed_data_self_test() {
        let data = SmartParsedData {
            offline_data_collection_status: OfflineDataCollectionStatus::Never,
            total_offline_data_collection_seconds: 0,
            self_test_execution_status: SelfTestExecutionStatus::SuccessOrNever,
            self_test_execution_percent_remaining: 0,
            short_and_extended_test_available: true,
            conveyance_test_available: false,
            start_test_available: true,
            abort_test_available: true,
            short_test_polling_minutes: 2,
            extended_test_polling_minutes: 60,
            conveyance_test_polling_minutes: 0,
        };

        assert!(data.self_test_available(SmartSelfTest::Short));
        assert!(!data.self_test_available(SmartSelfTest::Conveyance));
        assert_eq!(data.self_test_polling_minutes(SmartSelfTest::Short), 2);
    }
}
