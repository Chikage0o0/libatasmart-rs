//! SMART 数据解析

use crate::error::Result;
use crate::types::*;

/// 解析 SMART 数据
///
/// 从 512 字节的 SMART 数据中解析出结构化信息
pub(crate) fn parse_smart_data(raw: &[u8; 512]) -> Result<SmartParsedData> {
    // 解析离线数据收集状态（字节 362）
    let offline_data_collection_status = match raw[362] {
        0x00 | 0x80 => OfflineDataCollectionStatus::Never,
        0x02 | 0x82 => OfflineDataCollectionStatus::Success,
        0x03 => OfflineDataCollectionStatus::InProgress,
        0x04 | 0x84 => OfflineDataCollectionStatus::Suspended,
        0x05 | 0x85 => OfflineDataCollectionStatus::Aborted,
        0x06 | 0x86 => OfflineDataCollectionStatus::Fatal,
        _ => OfflineDataCollectionStatus::Unknown,
    };

    // 解析自检执行状态和剩余百分比（字节 363）
    let self_test_execution_percent_remaining = (10 * (raw[363] & 0xF)) as u32;
    let self_test_execution_status = match (raw[363] >> 4) & 0xF {
        0 => SelfTestExecutionStatus::SuccessOrNever,
        1 => SelfTestExecutionStatus::Aborted,
        2 => SelfTestExecutionStatus::Interrupted,
        3 => SelfTestExecutionStatus::Fatal,
        4 => SelfTestExecutionStatus::ErrorUnknown,
        5 => SelfTestExecutionStatus::ErrorElectrical,
        6 => SelfTestExecutionStatus::ErrorServo,
        7 => SelfTestExecutionStatus::ErrorRead,
        8 => SelfTestExecutionStatus::ErrorHandling,
        15 => SelfTestExecutionStatus::InProgress,
        _ => SelfTestExecutionStatus::SuccessOrNever,
    };

    // 解析离线数据收集总时间（字节 364-365，小端序）
    let total_offline_data_collection_seconds = u16::from_le_bytes([raw[364], raw[365]]) as u32;

    // 解析自检可用性标志（字节 367）
    let conveyance_test_available = (raw[367] & 32) != 0;
    let short_and_extended_test_available = (raw[367] & 16) != 0;
    let start_test_available = (raw[367] & 1) != 0;
    let abort_test_available = (raw[367] & 41) != 0;

    // 解析自检轮询时间（字节 372-376）
    let short_test_polling_minutes = raw[372] as u16;

    // 扩展自检时间：如果字节 373 不是 0xFF，使用它；否则使用字节 375-376
    let extended_test_polling_minutes = if raw[373] != 0xFF {
        raw[373] as u16
    } else {
        u16::from_le_bytes([raw[375], raw[376]])
    };

    let conveyance_test_polling_minutes = raw[374] as u16;

    Ok(SmartParsedData {
        offline_data_collection_status,
        total_offline_data_collection_seconds,
        self_test_execution_status,
        self_test_execution_percent_remaining,
        short_and_extended_test_available,
        conveyance_test_available,
        start_test_available,
        abort_test_available,
        short_test_polling_minutes,
        extended_test_polling_minutes,
        conveyance_test_polling_minutes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_smart_data() {
        let mut data = [0u8; 512];

        // 设置离线数据收集状态为 Never
        data[362] = 0x00;

        // 设置自检状态：成功，0% 剩余
        data[363] = 0x00;

        // 设置离线数据收集时间：100 秒
        data[364] = 100;
        data[365] = 0;

        // 设置自检可用性
        data[367] = 0x11; // start_test_available = 1

        // 设置轮询时间
        data[372] = 2; // short test: 2 分钟
        data[373] = 60; // extended test: 60 分钟
        data[374] = 2; // conveyance test: 2 分钟

        let parsed = parse_smart_data(&data).unwrap();

        assert_eq!(
            parsed.offline_data_collection_status,
            OfflineDataCollectionStatus::Never
        );
        assert_eq!(
            parsed.self_test_execution_status,
            SelfTestExecutionStatus::SuccessOrNever
        );
        assert_eq!(parsed.self_test_execution_percent_remaining, 0);
        assert_eq!(parsed.total_offline_data_collection_seconds, 100);
        assert_eq!(parsed.short_test_polling_minutes, 2);
        assert_eq!(parsed.extended_test_polling_minutes, 60);
    }

    #[test]
    fn test_parse_extended_test_time_extended_format() {
        let mut data = [0u8; 512];

        // 使用扩展格式（字节 373 = 0xFF）
        data[373] = 0xFF;
        data[375] = 0x2C; // 300 分钟的低字节
        data[376] = 0x01; // 300 分钟的高字节

        let parsed = parse_smart_data(&data).unwrap();
        assert_eq!(parsed.extended_test_polling_minutes, 300);
    }
}
