//! IDENTIFY 数据解析

use crate::types::IdentifyParsedData;
use crate::utils::read_ata_string;
use crate::error::Result;

/// 解析 IDENTIFY 数据
pub(crate) fn parse_identify_data(raw: &[u8; 512]) -> Result<IdentifyParsedData> {
    // IDENTIFY 数据结构:
    // 字节 20-39: 序列号 (20 字节)
    // 字节 46-53: 固件版本 (8 字节)
    // 字节 54-93: 型号 (40 字节)

    let serial = read_ata_string(&raw[20..40]);
    let firmware = read_ata_string(&raw[46..54]);
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
