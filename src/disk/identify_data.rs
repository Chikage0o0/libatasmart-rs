//! IDENTIFY 数据封装

use crate::error::Result;
use crate::types::IdentifyParsedData;

/// IDENTIFY 数据
#[derive(Debug, Clone)]
pub struct IdentifyData {
    raw: [u8; 512],
}

impl IdentifyData {
    /// 从原始数据创建
    pub(crate) fn new(raw: [u8; 512]) -> Self {
        Self { raw }
    }

    /// 获取原始数据
    pub fn raw(&self) -> &[u8; 512] {
        &self.raw
    }

    /// 解析 IDENTIFY 数据
    pub fn parse(&self) -> Result<IdentifyParsedData> {
        crate::identify::parse::parse_identify_data(&self.raw)
    }
}
