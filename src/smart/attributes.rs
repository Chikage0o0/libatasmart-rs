//! SMART 属性解析

use crate::types::{SmartAttributeParsedData, AttributeUnit};

/// 属性信息
struct AttributeInfo {
    name: &'static str,
    unit: AttributeUnit,
}

/// 属性信息表 (部分实现)
static ATTRIBUTE_INFO: [Option<AttributeInfo>; 256] = {
    let mut arr: [Option<AttributeInfo>; 256] = [None; 256];
    // TODO: 填充属性信息表
    arr
};

/// 解析单个属性
pub(crate) fn parse_attribute(
    _id: u8,
    _raw_data: &[u8],
    _threshold: u8,
) -> Option<SmartAttributeParsedData> {
    // TODO: 实现属性解析
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_attribute() {
        // TODO: 添加测试
    }
}
