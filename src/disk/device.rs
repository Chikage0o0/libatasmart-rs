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
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use atasmart::Disk;
    ///
    /// let disk = Disk::open("/dev/sda")?;
    /// let awake = disk.check_sleep_mode()?;
    /// println!("设备状态: {}", if awake { "活动" } else { "睡眠" });
    /// # Ok::<(), atasmart::Error>(())
    /// ```
    pub fn check_sleep_mode(&self) -> Result<bool> {
        // Blob类型不支持
        if self.disk_type == DiskType::Blob {
            return Err(Error::NotSupported(
                "Blob类型不支持睡眠模式检查".to_string(),
            ));
        }

        // 需要先有IDENTIFY数据
        if self.identify_data.is_none() {
            return Err(Error::NotSupported("需要先读取IDENTIFY数据".to_string()));
        }

        let fd = self.fd();
        let mut registers = ffi::commands::AtaRegisters::new();

        // 发送 CHECK_POWER_MODE 命令
        ffi::commands::send_ata_command(
            fd,
            self.disk_type,
            ffi::ata::AtaCommand::CheckPowerMode,
            ffi::ata::Direction::None,
            &mut registers,
            None,
        )?;

        // 检查返回状态
        // cmd[0] 应该是 0, cmd[5] 的最低位应该是 0
        if registers.data[0] != 0 || (registers.data[5] & 1) != 0 {
            return Err(
                std::io::Error::new(std::io::ErrorKind::InvalidData, "无效的电源模式响应").into(),
            );
        }

        // 获取状态值 (SECTOR COUNT 寄存器)
        let status = registers.data[3];

        // 0xFF = active/idle, 0x80 = idle
        // 其他值表示睡眠或待机状态
        Ok(status == 0xFF || status == 0x80)
    }

    /// 从设备读取 IDENTIFY 数据
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use atasmart::Disk;
    ///
    /// let mut disk = Disk::open("/dev/sda")?;
    /// disk.read_identify()?;
    /// # Ok::<(), atasmart::Error>(())
    /// ```
    pub fn read_identify(&mut self) -> Result<()> {
        // Blob类型不支持
        if self.disk_type == DiskType::Blob {
            return Ok(());
        }

        let fd = self.fd();
        let mut data = [0u8; 512];
        let mut registers = ffi::commands::AtaRegisters::new();
        registers.set_sector_count(1);

        // 发送 IDENTIFY DEVICE 命令
        ffi::commands::send_ata_command(
            fd,
            self.disk_type,
            ffi::ata::AtaCommand::IdentifyDevice,
            ffi::ata::Direction::In,
            &mut registers,
            Some(&mut data),
        )?;

        // 检查数据是否全为0 (无效)
        if data.iter().all(|&b| b == 0) {
            return Err(
                std::io::Error::new(std::io::ErrorKind::InvalidData, "IDENTIFY数据全为0").into(),
            );
        }

        self.identify_data = Some(data);
        Ok(())
    }

    /// 从设备读取 SMART 数据
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use atasmart::Disk;
    ///
    /// let mut disk = Disk::open("/dev/sda")?;
    /// disk.read_identify()?;
    /// disk.read_smart_data()?;
    /// # Ok::<(), atasmart::Error>(())
    /// ```
    pub fn read_smart_data(&mut self) -> Result<()> {
        // 检查SMART是否可用
        if !self.is_smart_available()? {
            return Err(Error::NotSupported("SMART功能不可用".to_string()));
        }

        // Blob类型不支持
        if self.disk_type == DiskType::Blob {
            return Ok(());
        }

        let fd = self.fd();
        let mut data = [0u8; 512];
        let mut registers = ffi::commands::AtaRegisters::new();

        // 设置SMART READ DATA命令参数
        registers.set_features(ffi::ata::SmartCommand::ReadData as u8);
        registers.set_sector_count(1);
        registers.set_lba_low(0x00);
        registers.set_lba_mid(0x4F);
        registers.set_lba_high(0xC2);

        // 发送 SMART 命令
        ffi::commands::send_ata_command(
            fd,
            self.disk_type,
            ffi::ata::AtaCommand::Smart,
            ffi::ata::Direction::In,
            &mut registers,
            Some(&mut data),
        )?;

        self.smart_data = Some(data);
        Ok(())
    }

    /// 从设备读取 SMART 阈值数据
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use atasmart::Disk;
    ///
    /// let mut disk = Disk::open("/dev/sda")?;
    /// disk.read_identify()?;
    /// disk.read_smart_thresholds()?;
    /// # Ok::<(), atasmart::Error>(())
    /// ```
    pub fn read_smart_thresholds(&mut self) -> Result<()> {
        // 检查SMART是否可用
        if !self.is_smart_available()? {
            return Err(Error::NotSupported("SMART功能不可用".to_string()));
        }

        // Blob类型不支持
        if self.disk_type == DiskType::Blob {
            return Ok(());
        }

        let fd = self.fd();
        let mut data = [0u8; 512];
        let mut registers = ffi::commands::AtaRegisters::new();

        // 设置SMART READ THRESHOLDS命令参数
        registers.set_features(ffi::ata::SmartCommand::ReadThresholds as u8);
        registers.set_sector_count(1);
        registers.set_lba_low(0x00);
        registers.set_lba_mid(0x4F);
        registers.set_lba_high(0xC2);

        // 发送 SMART 命令
        ffi::commands::send_ata_command(
            fd,
            self.disk_type,
            ffi::ata::AtaCommand::Smart,
            ffi::ata::Direction::In,
            &mut registers,
            Some(&mut data),
        )?;

        self.smart_thresholds = Some(data);
        Ok(())
    }

    /// 获取 SMART 健康状态
    ///
    /// # 返回
    ///
    /// * `Ok(true)` - SMART状态良好
    /// * `Ok(false)` - SMART状态异常,磁盘可能即将故障
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use atasmart::Disk;
    ///
    /// let mut disk = Disk::open("/dev/sda")?;
    /// disk.read_identify()?;
    /// let status = disk.smart_status()?;
    /// println!("SMART状态: {}", if status { "良好" } else { "异常" });
    /// # Ok::<(), atasmart::Error>(())
    /// ```
    pub fn smart_status(&mut self) -> Result<bool> {
        // 检查SMART是否可用
        if !self.is_smart_available()? {
            return Err(Error::NotSupported("SMART功能不可用".to_string()));
        }

        // Blob类型使用缓存的状态
        if self.disk_type == DiskType::Blob {
            return self.smart_status.ok_or(Error::NoData);
        }

        let fd = self.fd();
        let mut registers = ffi::commands::AtaRegisters::new();

        // 设置SMART RETURN STATUS命令参数
        registers.set_features(ffi::ata::SmartCommand::ReturnStatus as u8);
        registers.set_lba_low(0x00);
        registers.set_lba_mid(0x4F);
        registers.set_lba_high(0xC2);

        // 发送 SMART 命令
        ffi::commands::send_ata_command(
            fd,
            self.disk_type,
            ffi::ata::AtaCommand::Smart,
            ffi::ata::Direction::None,
            &mut registers,
            None,
        )?;

        // 检查返回的LBA寄存器值
        // LBA MID = 0x4F, LBA HIGH = 0xC2 表示状态良好
        // LBA MID = 0xF4, LBA HIGH = 0x2C 表示状态异常
        let lba_mid = registers.data[8];
        let lba_high = registers.data[7];

        let good = if (self.disk_type == DiskType::AtaPassthrough12 || lba_high == 0xC2)
            && lba_mid == 0x4F
        {
            true
        } else if (self.disk_type == DiskType::AtaPassthrough12 || lba_high == 0x2C)
            && lba_mid == 0xF4
        {
            false
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "无效的SMART状态响应",
            )
            .into());
        };

        self.smart_status = Some(good);
        Ok(good)
    }

    /// 检查SMART是否可用
    fn is_smart_available(&self) -> Result<bool> {
        let identify = self.identify_data.as_ref().ok_or(Error::NoData)?;
        // IDENTIFY word 82 bit 0 表示SMART是否支持
        Ok((identify[164] & 1) != 0)
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
            .ok_or(crate::error::Error::NoData)?;

        crate::identify::parse::parse_identify_data(identify_data)
    }

    /// 解析 SMART 数据
    pub fn parse_smart(&self) -> crate::error::Result<crate::types::SmartParsedData> {
        let smart_data = self
            .smart_data
            .as_ref()
            .ok_or(crate::error::Error::NoData)?;

        crate::smart::parse::parse_smart_data(smart_data)
    }

    /// 解析 SMART 属性
    pub fn parse_smart_attributes(
        &self,
    ) -> crate::error::Result<Vec<crate::types::SmartAttributeParsedData>> {
        let smart_data = self
            .smart_data
            .as_ref()
            .ok_or(crate::error::Error::NoData)?;

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
