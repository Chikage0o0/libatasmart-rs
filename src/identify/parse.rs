//! IDENTIFY 数据解析

use crate::error::Result;
use crate::types::IdentifyParsedData;
use crate::utils::read_ata_string;

/// 解析 IDENTIFY 数据
///
/// 从 512 字节的 IDENTIFY 数据中提取设备信息
pub(crate) fn parse_identify_data(raw: &[u8; 512]) -> Result<IdentifyParsedData> {
    // 序列号：字节 20-39 (20 字节)
    let serial = read_ata_string(&raw[20..40]);

    // 固件版本：字节 46-53 (8 字节)
    let firmware = read_ata_string(&raw[46..54]);

    // 型号：字节 54-93 (40 字节)
    let model = read_ata_string(&raw[54..94]);

    Ok(IdentifyParsedData {
        serial,
        firmware,
        model,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_identify_data() {
        // 创建测试数据
        let mut data = [0u8; 512];

        // 模拟序列号 "TEST1234" (需要字节交换)
        data[20] = b'E';
        data[21] = b'T';
        data[22] = b'S';
        data[23] = b'T';

        let result = parse_identify_data(&data);
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert!(!parsed.serial.is_empty());
    }
}
