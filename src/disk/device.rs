//! 磁盘设备操作

use crate::error::{Error, Result};
use crate::ffi;
use crate::types::*;
use std::fs::{File, OpenOptions};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;

/// 磁盘设备句柄
pub struct Disk {
    file: Option<File>,
    disk_type: DiskType,
    size: u64,
    identify_data: Option<[u8; 512]>,
    smart_data: Option<[u8; 512]>,
    smart_thresholds: Option<[u8; 512]>,
    smart_status: Option<bool>,
}

impl Disk {
    /// 打开磁盘设备
    ///
    /// # 参数
    ///
    /// * `path` - 设备路径,例如 `/dev/sda`
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use atasmart::Disk;
    ///
    /// let disk = Disk::open("/dev/sda")?;
    /// # Ok::<(), atasmart::Error>(())
    /// ```
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(false)
            .open(path.as_ref())?;

        let fd = file.as_raw_fd();

        // 获取设备大小
        let size = ffi::ioctl::get_block_size(fd)
            .map_err(|_| Error::Io(std::io::Error::last_os_error()))?;

        // 自动检测设备类型
        let disk_type = super::detect::detect_disk_type(fd)?;

        Ok(Self {
            file: Some(file),
            disk_type,
            size,
            identify_data: None,
            smart_data: None,
            smart_thresholds: None,
            smart_status: None,
        })
    }

    /// 获取文件描述符
    pub(crate) fn fd(&self) -> RawFd {
        self.file.as_ref().expect("Disk 没有文件句柄").as_raw_fd()
    }

    /// 获取磁盘大小 (字节)
    pub fn size(&self) -> u64 {
        self.size
    }

    /// 获取磁盘类型
    pub fn disk_type(&self) -> DiskType {
        self.disk_type
    }

    /// 检查设备是否处于睡眠模式
    ///
    /// # 返回
    ///
    /// * `Ok(true)` - 设备处于活动或空闲状态
    /// * `Ok(false)` - 设备处于睡眠状态
    pub fn check_sleep_mode(&self) -> Result<bool> {
        // TODO: 实现睡眠模式检查
        // 使用 CHECK_POWER_MODE 命令
        Ok(true)
    }

    /// 获取 IDENTIFY 数据
    pub fn identify_data(&self) -> Option<&[u8; 512]> {
        self.identify_data.as_ref()
    }

    /// 获取 SMART 数据
    pub fn smart_data(&self) -> Option<&[u8; 512]> {
        self.smart_data.as_ref()
    }

    /// 获取 SMART 阈值数据
    pub fn smart_thresholds(&self) -> Option<&[u8; 512]> {
        self.smart_thresholds.as_ref()
    }

    /// 设置 IDENTIFY 数据
    pub(crate) fn set_identify_data(&mut self, data: [u8; 512]) {
        self.identify_data = Some(data);
    }

    /// 设置 SMART 数据
    pub(crate) fn set_smart_data(&mut self, data: [u8; 512]) {
        self.smart_data = Some(data);
    }

    /// 设置 SMART 阈值数据
    pub(crate) fn set_smart_thresholds(&mut self, data: [u8; 512]) {
        self.smart_thresholds = Some(data);
    }

    /// 从 blob 数据创建 Disk 实例
    pub(crate) fn from_blob() -> Result<Self> {
        Ok(Self {
            file: None,
            disk_type: DiskType::Blob,
            size: 0,
            identify_data: None,
            smart_data: None,
            smart_thresholds: None,
            smart_status: None,
        })
    }

    /// 设置 SMART 状态
    pub(crate) fn set_smart_status(&mut self, status: bool) {
        self.smart_status = Some(status);
    }

    /// 获取 SMART 状态（内部使用）
    pub(crate) fn get_smart_status_internal(&self) -> Option<bool> {
        self.smart_status
    }

    /// 解析 IDENTIFY 数据
    pub fn parse_identify(&self) -> crate::error::Result<crate::types::IdentifyParsedData> {
        let identify_data = self
            .identify_data
            .as_ref()
            .ok_or_else(|| crate::error::Error::NoData)?;

        crate::identify::parse::parse_identify_data(identify_data)
    }

    /// 解析 SMART 数据
    pub fn parse_smart(&self) -> crate::error::Result<crate::types::SmartParsedData> {
        let smart_data = self
            .smart_data
            .as_ref()
            .ok_or_else(|| crate::error::Error::NoData)?;

        crate::smart::parse::parse_smart_data(smart_data)
    }

    /// 解析 SMART 属性
    pub fn parse_smart_attributes(
        &self,
    ) -> crate::error::Result<Vec<crate::types::SmartAttributeParsedData>> {
        let smart_data = self
            .smart_data
            .as_ref()
            .ok_or_else(|| crate::error::Error::NoData)?;

        let thresholds = self.smart_thresholds.as_ref();

        let mut attributes = Vec::new();

        // SMART 数据从字节 2 开始，每个属性 12 字节，共 30 个槽位
        for i in 0..30 {
            let offset = 2 + i * 12;
            let attr_data = &smart_data[offset..offset + 12];

            // 查找对应的阈值数据
            let threshold_data = thresholds.and_then(|t| {
                for j in 0..30 {
                    let t_offset = 2 + j * 12;
                    if t[t_offset] == attr_data[0] && attr_data[0] != 0 {
                        return Some(&t[t_offset..t_offset + 12]);
                    }
                }
                None
            });

            if let Some(attr) =
                crate::smart::attributes::parse_attribute(attr_data, threshold_data, self.size)
            {
                attributes.push(attr);
            }
        }

        Ok(attributes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disk_creation() {
        // 需要真实设备才能测试
        // 这里只测试类型定义
    }
}
