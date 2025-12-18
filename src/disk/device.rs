//! 磁盘设备操作

use crate::error::{Error, Result};
use crate::ffi;
use crate::types::*;
use std::fs::{File, OpenOptions};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;

/// 磁盘设备句柄
pub struct Disk {
    file: File,
    disk_type: DiskType,
    size: u64,
    identify_data: Option<[u8; 512]>,
    smart_data: Option<[u8; 512]>,
    smart_thresholds: Option<[u8; 512]>,
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
            file,
            disk_type,
            size,
            identify_data: None,
            smart_data: None,
            smart_thresholds: None,
        })
    }

    /// 获取文件描述符
    pub(crate) fn fd(&self) -> RawFd {
        self.file.as_raw_fd()
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
    pub(crate) fn identify_data(&self) -> Option<&[u8; 512]> {
        self.identify_data.as_ref()
    }

    /// 获取 SMART 数据
    pub(crate) fn smart_data(&self) -> Option<&[u8; 512]> {
        self.smart_data.as_ref()
    }

    /// 获取 SMART 阈值数据
    pub(crate) fn smart_thresholds(&self) -> Option<&[u8; 512]> {
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
