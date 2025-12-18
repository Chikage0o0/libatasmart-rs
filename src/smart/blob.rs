//! Blob 文件读取和解析
//!
//! Blob 文件格式用于存储 SMART 数据的快照，主要用于测试和离线分析

use crate::disk::Disk;
use crate::error::{Error, Result};
use crate::types::DiskType;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Blob 标签类型
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BlobTag {
    /// IDENTIFY 数据
    Identify = 0x49444659, // 'IDFY'
    /// SMART 状态
    SmartStatus = 0x534D5354, // 'SMST'
    /// SMART 数据
    SmartData = 0x534D4454, // 'SMDT'
    /// SMART 阈值
    SmartThresholds = 0x534D5448, // 'SMTH'
}

impl BlobTag {
    /// 从 u32 值创建标签
    fn from_u32(value: u32) -> Option<Self> {
        match value {
            0x49444659 => Some(BlobTag::Identify),
            0x534D5354 => Some(BlobTag::SmartStatus),
            0x534D4454 => Some(BlobTag::SmartData),
            0x534D5448 => Some(BlobTag::SmartThresholds),
            _ => None,
        }
    }
}

/// Blob 数据结构
pub struct BlobData {
    /// IDENTIFY 数据
    pub identify: Option<[u8; 512]>,
    /// SMART 状态
    pub smart_status: Option<bool>,
    /// SMART 数据
    pub smart_data: Option<[u8; 512]>,
    /// SMART 阈值
    pub smart_thresholds: Option<[u8; 512]>,
}

impl BlobData {
    /// 创建空的 blob 数据
    fn new() -> Self {
        Self {
            identify: None,
            smart_status: None,
            smart_data: None,
            smart_thresholds: None,
        }
    }
}

/// 从文件读取 blob 数据
pub fn read_blob_from_file<P: AsRef<Path>>(path: P) -> Result<BlobData> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    parse_blob(&buffer)
}

/// 解析 blob 数据
fn parse_blob(data: &[u8]) -> Result<BlobData> {
    let mut blob_data = BlobData::new();
    let mut pos = 0;

    // 第一遍：验证格式
    let mut has_identify = false;
    let mut has_smart_status = false;
    let mut has_smart_data = false;
    let mut has_smart_thresholds = false;

    let mut temp_pos = 0;
    while temp_pos + 8 <= data.len() {
        // 读取标签（4 字节）
        let tag_bytes = [
            data[temp_pos],
            data[temp_pos + 1],
            data[temp_pos + 2],
            data[temp_pos + 3],
        ];
        let tag_value = u32::from_be_bytes(tag_bytes);

        // 读取大小（4 字节，网络字节序）
        let size_bytes = [
            data[temp_pos + 4],
            data[temp_pos + 5],
            data[temp_pos + 6],
            data[temp_pos + 7],
        ];
        let size = u32::from_be_bytes(size_bytes) as usize;

        temp_pos += 8;

        if temp_pos + size > data.len() {
            return Err(Error::InvalidData("Blob 数据不完整".to_string()));
        }

        // 验证标签和大小
        match BlobTag::from_u32(tag_value) {
            Some(BlobTag::Identify) => {
                if size != 512 || has_identify {
                    return Err(Error::InvalidData("无效的 IDENTIFY 块".to_string()));
                }
                has_identify = true;
            }
            Some(BlobTag::SmartStatus) => {
                if size != 4 || has_smart_status {
                    return Err(Error::InvalidData("无效的 SMART STATUS 块".to_string()));
                }
                has_smart_status = true;
            }
            Some(BlobTag::SmartData) => {
                if size != 512 || has_smart_data {
                    return Err(Error::InvalidData("无效的 SMART DATA 块".to_string()));
                }
                has_smart_data = true;
            }
            Some(BlobTag::SmartThresholds) => {
                if size != 512 || has_smart_thresholds {
                    return Err(Error::InvalidData("无效的 SMART THRESHOLDS 块".to_string()));
                }
                has_smart_thresholds = true;
            }
            None => {
                return Err(Error::InvalidData(format!(
                    "未知的 blob 标签: 0x{:08X}",
                    tag_value
                )));
            }
        }

        temp_pos += size;
    }

    if !has_identify {
        return Err(Error::InvalidData("Blob 数据缺少 IDENTIFY 块".to_string()));
    }

    // 第二遍：实际读取数据
    while pos + 8 <= data.len() {
        // 读取标签
        let tag_bytes = [data[pos], data[pos + 1], data[pos + 2], data[pos + 3]];
        let tag_value = u32::from_be_bytes(tag_bytes);

        // 读取大小
        let size_bytes = [data[pos + 4], data[pos + 5], data[pos + 6], data[pos + 7]];
        let size = u32::from_be_bytes(size_bytes) as usize;

        pos += 8;

        if let Some(tag) = BlobTag::from_u32(tag_value) {
            match tag {
                BlobTag::Identify => {
                    let mut identify = [0u8; 512];
                    identify.copy_from_slice(&data[pos..pos + 512]);
                    blob_data.identify = Some(identify);
                }
                BlobTag::SmartStatus => {
                    let status_bytes = [data[pos], data[pos + 1], data[pos + 2], data[pos + 3]];
                    let status = u32::from_be_bytes(status_bytes);
                    blob_data.smart_status = Some(status != 0);
                }
                BlobTag::SmartData => {
                    let mut smart_data = [0u8; 512];
                    smart_data.copy_from_slice(&data[pos..pos + 512]);
                    blob_data.smart_data = Some(smart_data);
                }
                BlobTag::SmartThresholds => {
                    let mut thresholds = [0u8; 512];
                    thresholds.copy_from_slice(&data[pos..pos + 512]);
                    blob_data.smart_thresholds = Some(thresholds);
                }
            }
        }

        pos += size;
    }

    Ok(blob_data)
}

/// 从 blob 文件创建 Disk 实例
pub fn disk_from_blob<P: AsRef<Path>>(path: P) -> Result<Disk> {
    let blob_data = read_blob_from_file(path)?;

    // 创建一个 blob 类型的 Disk
    let mut disk = Disk::from_blob()?;

    // 设置数据
    if let Some(identify) = blob_data.identify {
        disk.set_identify_data(identify);
    }

    if let Some(smart_data) = blob_data.smart_data {
        disk.set_smart_data(smart_data);
    }

    if let Some(thresholds) = blob_data.smart_thresholds {
        disk.set_smart_thresholds(thresholds);
    }

    if let Some(status) = blob_data.smart_status {
        disk.set_smart_status(status);
    }

    Ok(disk)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blob_tag_conversion() {
        assert_eq!(BlobTag::from_u32(0x49444659), Some(BlobTag::Identify));
        assert_eq!(BlobTag::from_u32(0x534D5354), Some(BlobTag::SmartStatus));
        assert_eq!(BlobTag::from_u32(0x534D4454), Some(BlobTag::SmartData));
        assert_eq!(
            BlobTag::from_u32(0x534D5448),
            Some(BlobTag::SmartThresholds)
        );
        assert_eq!(BlobTag::from_u32(0x12345678), None);
    }

    #[test]
    fn test_blob_data_creation() {
        let blob_data = BlobData::new();
        assert!(blob_data.identify.is_none());
        assert!(blob_data.smart_status.is_none());
        assert!(blob_data.smart_data.is_none());
        assert!(blob_data.smart_thresholds.is_none());
    }
}
