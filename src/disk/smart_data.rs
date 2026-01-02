//! SMART 数据封装

use crate::error::Result;
use crate::types::*;

/// SMART 数据
#[derive(Debug, Clone)]
pub struct SmartData {
    raw: [u8; 512],
    disk_size: u64,
}

impl SmartData {
    /// 从原始数据创建
    pub(crate) fn new(raw: [u8; 512], disk_size: u64) -> Self {
        Self { raw, disk_size }
    }

    /// 获取原始数据
    pub fn raw(&self) -> &[u8; 512] {
        &self.raw
    }

    /// 解析 SMART 数据
    pub fn parse(&self) -> Result<SmartParsedData> {
        crate::smart::parse::parse_smart_data(&self.raw)
    }

    /// 解析 SMART 属性 (需要阈值数据)
    pub fn parse_attributes(
        &self,
        thresholds: Option<&SmartThresholds>,
    ) -> Result<Vec<SmartAttributeParsedData>> {
        let thresholds_raw = thresholds.map(|t| t.raw());

        let mut attributes = Vec::new();

        // SMART 数据从字节 2 开始,每个属性 12 字节,共 30 个槽位
        for i in 0..30 {
            let offset = 2 + i * 12;
            let attr_data = &self.raw[offset..offset + 12];

            // 查找对应的阈值数据
            let threshold_data = thresholds_raw.and_then(|t| {
                for j in 0..30 {
                    let t_offset = 2 + j * 12;
                    if t[t_offset] == attr_data[0] && attr_data[0] != 0 {
                        return Some(&t[t_offset..t_offset + 12]);
                    }
                }
                None
            });

            if let Some(attr) =
                crate::smart::attributes::parse_attribute(attr_data, threshold_data, self.disk_size)
            {
                attributes.push(attr);
            }
        }

        Ok(attributes)
    }
}

/// SMART 阈值数据
#[derive(Debug, Clone)]
pub struct SmartThresholds {
    raw: [u8; 512],
}

impl SmartThresholds {
    /// 从原始数据创建
    pub(crate) fn new(raw: [u8; 512]) -> Self {
        Self { raw }
    }

    /// 获取原始数据
    pub fn raw(&self) -> &[u8; 512] {
        &self.raw
    }
}

/// 完整的 SMART 信息 (数据 + 阈值)
#[derive(Debug, Clone)]
pub struct SmartInfo {
    /// SMART 数据
    pub data: SmartData,
    /// SMART 阈值 (可选)
    pub thresholds: Option<SmartThresholds>,
}

impl SmartInfo {
    /// 从数据和阈值创建
    pub(crate) fn new(data: SmartData, thresholds: Option<SmartThresholds>) -> Self {
        Self { data, thresholds }
    }

    /// 解析 SMART 属性
    pub fn parse_attributes(&self) -> Result<Vec<SmartAttributeParsedData>> {
        self.data.parse_attributes(self.thresholds.as_ref())
    }
}
