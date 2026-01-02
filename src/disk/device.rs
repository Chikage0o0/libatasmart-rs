//! 磁盘设备操作

use crate::disk::{IdentifyData, SmartData, SmartInfo, SmartThresholds};
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
    /// use libatasmart::Disk;
    ///
    /// let disk = Disk::open("/dev/sda")?;
    /// # Ok::<(), libatasmart::Error>(())
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
    /// use libatasmart::Disk;
    ///
    /// let disk = Disk::open("/dev/sda")?;
    /// let awake = disk.check_sleep_mode()?;
    /// println!("设备状态: {}", if awake { "活动" } else { "睡眠" });
    /// # Ok::<(), libatasmart::Error>(())
    /// ```
    pub fn check_sleep_mode(&self) -> Result<bool> {
        // Blob类型不支持
        if self.disk_type == DiskType::Blob {
            return Err(Error::NotSupported(
                "Blob类型不支持睡眠模式检查".to_string(),
            ));
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
    /// use libatasmart::Disk;
    ///
    /// let disk = Disk::open("/dev/sda")?;
    /// let identify = disk.read_identify()?;
    /// let info = identify.parse()?;
    /// println!("型号: {}", info.model);
    /// # Ok::<(), libatasmart::Error>(())
    /// ```
    pub fn read_identify(&self) -> Result<IdentifyData> {
        // Blob类型不支持
        if self.disk_type == DiskType::Blob {
            return Err(Error::NotSupported(
                "Blob类型不支持读取IDENTIFY".to_string(),
            ));
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

        Ok(IdentifyData::new(data))
    }

    /// 从设备读取 SMART 数据
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use libatasmart::Disk;
    ///
    /// let disk = Disk::open("/dev/sda")?;
    /// let smart_data = disk.read_smart_data()?;
    /// let parsed = smart_data.parse()?;
    /// # Ok::<(), libatasmart::Error>(())
    /// ```
    pub fn read_smart_data(&self) -> Result<SmartData> {
        // 检查SMART是否可用
        let identify = self.read_identify()?;
        if !Self::is_smart_available(&identify)? {
            return Err(Error::NotSupported("SMART功能不可用".to_string()));
        }

        // Blob类型不支持
        if self.disk_type == DiskType::Blob {
            return Err(Error::NotSupported(
                "Blob类型不支持读取SMART数据".to_string(),
            ));
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

        Ok(SmartData::new(data, self.size))
    }

    /// 从设备读取 SMART 阈值数据
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use libatasmart::Disk;
    ///
    /// let disk = Disk::open("/dev/sda")?;
    /// let thresholds = disk.read_smart_thresholds()?;
    /// # Ok::<(), libatasmart::Error>(())
    /// ```
    pub fn read_smart_thresholds(&self) -> Result<SmartThresholds> {
        // 检查SMART是否可用
        let identify = self.read_identify()?;
        if !Self::is_smart_available(&identify)? {
            return Err(Error::NotSupported("SMART功能不可用".to_string()));
        }

        // Blob类型不支持
        if self.disk_type == DiskType::Blob {
            return Err(Error::NotSupported(
                "Blob类型不支持读取SMART阈值".to_string(),
            ));
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

        Ok(SmartThresholds::new(data))
    }

    /// 读取完整的 SMART 信息 (数据 + 阈值)
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use libatasmart::Disk;
    ///
    /// let disk = Disk::open("/dev/sda")?;
    /// let smart = disk.read_smart()?;
    /// let stats = smart.statistics();
    /// # Ok::<(), libatasmart::Error>(())
    /// ```
    pub fn read_smart(&self) -> Result<SmartInfo> {
        let data = self.read_smart_data()?;
        let thresholds = self.read_smart_thresholds().ok();
        Ok(SmartInfo::new(data, thresholds))
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
    /// use libatasmart::Disk;
    ///
    /// let disk = Disk::open("/dev/sda")?;
    /// let status = disk.is_healthy()?;
    /// println!("SMART状态: {}", if status { "良好" } else { "异常" });
    /// # Ok::<(), libatasmart::Error>(())
    /// ```
    pub fn is_healthy(&self) -> Result<bool> {
        // 检查SMART是否可用
        let identify = self.read_identify()?;
        if !Self::is_smart_available(&identify)? {
            return Err(Error::NotSupported("SMART功能不可用".to_string()));
        }

        // Blob类型不支持
        if self.disk_type == DiskType::Blob {
            return Err(Error::NotSupported(
                "Blob类型不支持健康状态查询".to_string(),
            ));
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

        Ok(good)
    }

    /// 执行硬盘自检
    ///
    /// # 参数
    ///
    /// * `test` - 自检类型 (短时/扩展/传输/中止)
    ///
    /// # 返回
    ///
    /// * `Ok(())` - 自检已成功启动
    /// * `Err(Error::NotSupported)` - 自检功能不可用或不支持该类型的自检
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use libatasmart::{Disk, SmartSelfTest};
    ///
    /// let disk = Disk::open("/dev/sda")?;
    ///
    /// // 启动短时自检
    /// disk.start_self_test(SmartSelfTest::Short)?;
    /// println!("短时自检已启动");
    /// # Ok::<(), libatasmart::Error>(())
    /// ```
    pub fn start_self_test(&self, test: SmartSelfTest) -> Result<()> {
        // 检查SMART是否可用
        let identify = self.read_identify()?;
        if !Self::is_smart_available(&identify)? {
            return Err(Error::NotSupported("SMART功能不可用".to_string()));
        }

        // Blob类型不支持
        if self.disk_type == DiskType::Blob {
            return Err(Error::NotSupported("Blob类型不支持自检".to_string()));
        }

        // 读取SMART数据以检查自检功能可用性
        let smart_data = self.read_smart_data()?;
        let parsed = smart_data.parse()?;

        // 检查自检功能是否可用
        if !parsed.self_test_available(test) {
            return Err(Error::NotSupported(format!("{} 自检不可用", test.as_str())));
        }

        let fd = self.fd();
        let mut registers = ffi::commands::AtaRegisters::new();

        // 设置SMART EXECUTE OFFLINE IMMEDIATE命令参数
        registers.set_features(ffi::ata::SmartCommand::ExecuteOfflineImmediate as u8);
        registers.set_lba_low(0x00);
        registers.set_lba_mid(0x4F);
        registers.set_lba_high(0xC2);
        // 测试类型放在LBA LOW寄存器的低字节
        registers.data[9] = test as u8;

        // 发送 SMART 命令
        ffi::commands::send_ata_command(
            fd,
            self.disk_type,
            ffi::ata::AtaCommand::Smart,
            ffi::ata::Direction::None,
            &mut registers,
            None,
        )?;

        Ok(())
    }

    /// 检查SMART是否可用
    fn is_smart_available(identify: &IdentifyData) -> Result<bool> {
        let raw = identify.raw();
        // IDENTIFY word 82 bit 0 表示SMART是否支持
        Ok((raw[164] & 1) != 0)
    }

    /// 从 blob 数据创建 Disk 实例
    pub(crate) fn from_blob() -> Result<Self> {
        Ok(Self {
            file: None,
            disk_type: DiskType::Blob,
            size: 0,
        })
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
