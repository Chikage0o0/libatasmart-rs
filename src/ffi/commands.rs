//! ATA 命令发送实现
//!
//! 实现多种 ATA 命令传输方式,支持不同的硬件接口

use super::ata::{AtaCommand, Direction};
use super::ioctl::sg_io_cmd;
use super::scsi::{
    ScsiCdb12, ScsiCdb16, SgIoHdr, SG_DXFER_FROM_DEV, SG_DXFER_NONE, SG_DXFER_TO_DEV,
};
use crate::error::Result;
use crate::types::DiskType;
use std::os::unix::io::RawFd;

/// 超时时间 (毫秒)
const TIMEOUT_MS: u32 = 2000;

/// ATA 命令寄存器缓冲区 (12 字节)
///
/// 用于存储 ATA 命令的寄存器值
/// 索引对应关系参考 C 源码中的 cmd_data
#[derive(Debug, Clone, Copy)]
pub(crate) struct AtaRegisters {
    pub data: [u8; 12],
}

impl AtaRegisters {
    /// 创建新的寄存器缓冲区
    pub fn new() -> Self {
        Self { data: [0u8; 12] }
    }

    /// 设置 FEATURES 寄存器
    pub fn set_features(&mut self, value: u8) {
        self.data[1] = value;
    }

    /// 设置 SECTOR COUNT 寄存器
    pub fn set_sector_count(&mut self, value: u8) {
        self.data[3] = value;
    }

    /// 设置 LBA LOW 寄存器
    pub fn set_lba_low(&mut self, value: u8) {
        self.data[9] = value;
    }

    /// 设置 LBA MID 寄存器
    pub fn set_lba_mid(&mut self, value: u8) {
        self.data[8] = value;
    }

    /// 设置 LBA HIGH 寄存器
    pub fn set_lba_high(&mut self, value: u8) {
        self.data[7] = value;
    }

    /// 设置 DEVICE/SELECT 寄存器
    pub fn set_device(&mut self, value: u8) {
        self.data[10] = value;
    }

    /// 获取 STATUS 寄存器
    pub fn status(&self) -> u8 {
        self.data[11]
    }

    /// 获取 ERROR 寄存器
    pub fn error(&self) -> u8 {
        self.data[2]
    }
}

/// ATA Passthrough 16 命令发送
///
/// 使用 16 字节 SCSI CDB 发送 ATA 命令
pub(crate) fn passthrough_16(
    fd: RawFd,
    command: AtaCommand,
    direction: Direction,
    registers: &mut AtaRegisters,
    data: Option<&mut [u8]>,
) -> Result<()> {
    let mut cdb = ScsiCdb16::new();
    let mut sense = [0u8; 32];

    // 构建 ATA Pass-Through 16 字节命令
    // 参考: T10 04-262r8 ATA Command Pass-Through
    cdb.data[0] = 0x85; // OPERATION CODE: 16 byte pass through

    // 设置协议和传输方向
    match direction {
        Direction::None => {
            cdb.data[1] = 3 << 1; // PROTOCOL: Non-Data
            cdb.data[2] = 0x20; // OFF_LINE=0, CK_COND=1, T_DIR=0, BYT_BLOK=0, T_LENGTH=0
        }
        Direction::In => {
            cdb.data[1] = 4 << 1; // PROTOCOL: PIO Data-in
            cdb.data[2] = 0x2e; // OFF_LINE=0, CK_COND=1, T_DIR=1, BYT_BLOK=1, T_LENGTH=2
        }
        Direction::Out => {
            cdb.data[1] = 5 << 1; // PROTOCOL: PIO Data-Out
            cdb.data[2] = 0x26; // OFF_LINE=0, CK_COND=1, T_DIR=0, BYT_BLOK=1, T_LENGTH=2
        }
    }

    // 填充 ATA 寄存器值到 CDB
    cdb.data[3] = registers.data[0]; // FEATURES (15:8)
    cdb.data[4] = registers.data[1]; // FEATURES (7:0)
    cdb.data[5] = registers.data[2]; // SECTOR COUNT (15:8)
    cdb.data[6] = registers.data[3]; // SECTOR COUNT (7:0)
    cdb.data[8] = registers.data[9]; // LBA LOW
    cdb.data[10] = registers.data[8]; // LBA MID
    cdb.data[12] = registers.data[7]; // LBA HIGH
    cdb.data[13] = registers.data[10] & 0x4F; // DEVICE/SELECT
    cdb.data[14] = command as u8; // COMMAND

    // 准备 SG_IO 头
    let sg_direction = match direction {
        Direction::None => SG_DXFER_NONE,
        Direction::In => SG_DXFER_FROM_DEV,
        Direction::Out => SG_DXFER_TO_DEV,
    };

    let (data_ptr, data_len) = match data {
        Some(buf) => (buf.as_mut_ptr(), buf.len() as u32),
        None => (std::ptr::null_mut(), 0),
    };

    let mut hdr = SgIoHdr::new();
    hdr.interface_id = b'S' as i32;
    hdr.dxfer_direction = sg_direction;
    hdr.cmd_len = 16;
    hdr.mx_sb_len = sense.len() as u8;
    hdr.dxfer_len = data_len;
    hdr.dxferp = data_ptr;
    hdr.cmdp = cdb.data.as_mut_ptr();
    hdr.sbp = sense.as_mut_ptr();
    hdr.timeout = TIMEOUT_MS;

    // 发送命令
    sg_io_cmd(fd, &mut hdr)?;

    // 解析 sense 数据获取 ATA 返回寄存器
    // sense[0] 应该是 0x72 (descriptor format)
    // sense[8..] 是 ATA Status Return descriptor
    if sense[0] != 0x72 || sense[8] != 0x09 || sense[9] != 0x0c {
        return Err(
            std::io::Error::new(std::io::ErrorKind::InvalidData, "无效的 SCSI sense 数据").into(),
        );
    }

    // 提取 ATA 返回寄存器
    let desc = &sense[8..];
    registers.data[0] = 0;
    registers.data[1] = desc[3]; // FEATURES
    registers.data[2] = desc[4]; // STATUS
    registers.data[3] = desc[5]; // SECTOR COUNT
    registers.data[7] = desc[11]; // LBA HIGH
    registers.data[8] = desc[9]; // LBA MID
    registers.data[9] = desc[7]; // LBA LOW
    registers.data[10] = desc[12]; // DEVICE
    registers.data[11] = desc[13]; // ERROR

    Ok(())
}

/// ATA Passthrough 12 命令发送
///
/// 使用 12 字节 SCSI CDB 发送 ATA 命令
pub(crate) fn passthrough_12(
    fd: RawFd,
    command: AtaCommand,
    direction: Direction,
    registers: &mut AtaRegisters,
    data: Option<&mut [u8]>,
) -> Result<()> {
    let mut cdb = ScsiCdb12::new();
    let mut sense = [0u8; 32];

    // 构建 ATA Pass-Through 12 字节命令
    cdb.data[0] = 0xa1; // OPERATION CODE: 12 byte pass through

    // 设置协议和传输方向
    match direction {
        Direction::None => {
            cdb.data[1] = 3 << 1; // PROTOCOL: Non-Data
            cdb.data[2] = 0x20;
        }
        Direction::In => {
            cdb.data[1] = 4 << 1; // PROTOCOL: PIO Data-in
            cdb.data[2] = 0x2e;
        }
        Direction::Out => {
            cdb.data[1] = 5 << 1; // PROTOCOL: PIO Data-Out
            cdb.data[2] = 0x26;
        }
    }

    // 填充 ATA 寄存器值到 CDB
    cdb.data[3] = registers.data[1]; // FEATURES
    cdb.data[4] = registers.data[3]; // SECTOR COUNT
    cdb.data[5] = registers.data[9]; // LBA LOW
    cdb.data[6] = registers.data[8]; // LBA MID
    cdb.data[7] = registers.data[7]; // LBA HIGH
    cdb.data[8] = registers.data[10] & 0x4F; // DEVICE/SELECT
    cdb.data[9] = command as u8; // COMMAND

    // 准备 SG_IO 头
    let sg_direction = match direction {
        Direction::None => SG_DXFER_NONE,
        Direction::In => SG_DXFER_FROM_DEV,
        Direction::Out => SG_DXFER_TO_DEV,
    };

    let (data_ptr, data_len) = match data {
        Some(buf) => (buf.as_mut_ptr(), buf.len() as u32),
        None => (std::ptr::null_mut(), 0),
    };

    let mut hdr = SgIoHdr::new();
    hdr.interface_id = b'S' as i32;
    hdr.dxfer_direction = sg_direction;
    hdr.cmd_len = 12;
    hdr.mx_sb_len = sense.len() as u8;
    hdr.dxfer_len = data_len;
    hdr.dxferp = data_ptr;
    hdr.cmdp = cdb.data.as_mut_ptr();
    hdr.sbp = sense.as_mut_ptr();
    hdr.timeout = TIMEOUT_MS;

    // 发送命令
    sg_io_cmd(fd, &mut hdr)?;

    // 解析 sense 数据
    if sense[0] != 0x72 || sense[8] != 0x09 || sense[9] != 0x0c {
        return Err(
            std::io::Error::new(std::io::ErrorKind::InvalidData, "无效的 SCSI sense 数据").into(),
        );
    }

    // 提取 ATA 返回寄存器
    let desc = &sense[8..];
    registers.data[0] = 0;
    registers.data[1] = desc[3]; // FEATURES
    registers.data[2] = desc[4]; // STATUS
    registers.data[3] = desc[5]; // SECTOR COUNT
    registers.data[7] = desc[11]; // LBA HIGH
    registers.data[8] = desc[9]; // LBA MID
    registers.data[9] = desc[7]; // LBA LOW
    registers.data[10] = desc[12]; // DEVICE
    registers.data[11] = desc[13]; // ERROR

    Ok(())
}

/// Sunplus USB/ATA 桥接命令发送
///
/// 使用 Sunplus 特定的 SCSI 命令
pub(crate) fn sunplus_command(
    fd: RawFd,
    command: AtaCommand,
    direction: Direction,
    registers: &mut AtaRegisters,
    data: Option<&mut [u8]>,
) -> Result<()> {
    let mut cdb = ScsiCdb12::new();
    let mut sense = [0u8; 32];

    // 构建 Sunplus 特定命令
    cdb.data[0] = 0xF8; // OPERATION CODE: Sunplus specific
    cdb.data[1] = 0x00; // Subcommand: Pass-thru
    cdb.data[2] = 0x22;

    // 设置协议
    cdb.data[3] = match direction {
        Direction::None => 0x00,
        Direction::In => 0x10,
        Direction::Out => 0x11,
    };

    // 填充 ATA 寄存器
    cdb.data[4] = registers.data[3]; // size?
    cdb.data[5] = registers.data[1]; // FEATURES
    cdb.data[6] = registers.data[3]; // SECTOR COUNT
    cdb.data[7] = registers.data[9]; // LBA LOW
    cdb.data[8] = registers.data[8]; // LBA MID
    cdb.data[9] = registers.data[7]; // LBA HIGH
    cdb.data[10] = registers.data[10] | 0xA0; // DEVICE/SELECT
    cdb.data[11] = command as u8; // COMMAND

    // 准备 SG_IO 头
    let sg_direction = match direction {
        Direction::None => SG_DXFER_NONE,
        Direction::In => SG_DXFER_FROM_DEV,
        Direction::Out => SG_DXFER_TO_DEV,
    };

    let (data_ptr, data_len) = match data {
        Some(buf) => (buf.as_mut_ptr(), buf.len() as u32),
        None => (std::ptr::null_mut(), 0),
    };

    let mut hdr = SgIoHdr::new();
    hdr.interface_id = b'S' as i32;
    hdr.dxfer_direction = sg_direction;
    hdr.cmd_len = 12;
    hdr.mx_sb_len = sense.len() as u8;
    hdr.dxfer_len = data_len;
    hdr.dxferp = data_ptr;
    hdr.cmdp = cdb.data.as_mut_ptr();
    hdr.sbp = sense.as_mut_ptr();
    hdr.timeout = TIMEOUT_MS;

    // 发送命令
    sg_io_cmd(fd, &mut hdr)?;

    // 获取响应
    let mut response_cdb = ScsiCdb12::new();
    response_cdb.data[0] = 0xF8;
    response_cdb.data[1] = 0x00;
    response_cdb.data[2] = 0x21;

    let mut buf = [0u8; 8];
    let mut response_hdr = SgIoHdr::new();
    response_hdr.interface_id = b'S' as i32;
    response_hdr.dxfer_direction = SG_DXFER_FROM_DEV;
    response_hdr.cmd_len = 12;
    response_hdr.mx_sb_len = sense.len() as u8;
    response_hdr.dxfer_len = buf.len() as u32;
    response_hdr.dxferp = buf.as_mut_ptr();
    response_hdr.cmdp = response_cdb.data.as_mut_ptr();
    response_hdr.sbp = sense.as_mut_ptr();
    response_hdr.timeout = TIMEOUT_MS;

    sg_io_cmd(fd, &mut response_hdr)?;

    // 提取返回寄存器
    registers.data[0] = 0;
    registers.data[2] = buf[1]; // ERROR
    registers.data[3] = buf[2]; // SECTOR COUNT
    registers.data[7] = buf[5]; // LBA HIGH
    registers.data[8] = buf[4]; // LBA MID
    registers.data[9] = buf[3]; // LBA LOW
    registers.data[10] = buf[6]; // DEVICE
    registers.data[11] = buf[7]; // STATUS

    Ok(())
}

/// JMicron USB/ATA 桥接命令发送
///
/// 使用 JMicron 特定的 SCSI 命令
pub(crate) fn jmicron_command(
    fd: RawFd,
    command: AtaCommand,
    direction: Direction,
    registers: &mut AtaRegisters,
    data: Option<&mut [u8]>,
) -> Result<()> {
    let mut cdb = ScsiCdb12::new();
    let mut sense = [0u8; 32];

    // 首先读取端口信息
    cdb.data[0] = 0xdf; // operation code
    cdb.data[1] = 0x10;
    cdb.data[2] = 0x00;
    cdb.data[3] = 0x00; // size HI
    cdb.data[4] = 1; // size LO (sizeof(port))
    cdb.data[5] = 0x00;
    cdb.data[6] = 0x72; // register address HI
    cdb.data[7] = 0x0f; // register address LO
    cdb.data[8] = 0x00;
    cdb.data[9] = 0x00;
    cdb.data[10] = 0x00;
    cdb.data[11] = 0xfd;

    let mut port = 0u8;
    let mut hdr = SgIoHdr::new();
    hdr.interface_id = b'S' as i32;
    hdr.dxfer_direction = SG_DXFER_FROM_DEV;
    hdr.cmd_len = 12;
    hdr.mx_sb_len = sense.len() as u8;
    hdr.dxfer_len = 1;
    hdr.dxferp = &mut port as *mut u8;
    hdr.cmdp = cdb.data.as_mut_ptr();
    hdr.sbp = sense.as_mut_ptr();
    hdr.timeout = TIMEOUT_MS;

    sg_io_cmd(fd, &mut hdr)?;

    // 检查端口是否有效
    // Port & 0x04 是端口 #0, Port & 0x40 是端口 #1
    if (port & 0x44) == 0 {
        return Err(
            std::io::Error::new(std::io::ErrorKind::NotFound, "无效的 JMicron 端口").into(),
        );
    }

    // 准备发送 ATA 命令
    cdb.data[0] = 0xdf;
    cdb.data[1] = 0x10;
    cdb.data[2] = 0x00;

    let data_len = data.as_ref().map(|d| d.len()).unwrap_or(0);
    cdb.data[3] = (data_len >> 8) as u8;
    cdb.data[4] = (data_len & 0xFF) as u8;

    cdb.data[5] = registers.data[1]; // FEATURES
    cdb.data[6] = registers.data[3]; // SECTOR COUNT
    cdb.data[7] = registers.data[9]; // LBA LOW
    cdb.data[8] = registers.data[8]; // LBA MID
    cdb.data[9] = registers.data[7]; // LBA HIGH
    cdb.data[10] = registers.data[10] | if (port & 0x04) != 0 { 0xA0 } else { 0xB0 }; // DEVICE
    cdb.data[11] = command as u8; // COMMAND

    // 发送命令
    let sg_direction = match direction {
        Direction::None => SG_DXFER_NONE,
        Direction::In => SG_DXFER_FROM_DEV,
        Direction::Out => SG_DXFER_TO_DEV,
    };

    let (data_ptr, data_len) = match data {
        Some(buf) => (buf.as_mut_ptr(), buf.len() as u32),
        None => (std::ptr::null_mut(), 0),
    };

    hdr = SgIoHdr::new();
    hdr.interface_id = b'S' as i32;
    hdr.dxfer_direction = sg_direction;
    hdr.cmd_len = 12;
    hdr.mx_sb_len = sense.len() as u8;
    hdr.dxfer_len = data_len;
    hdr.dxferp = data_ptr;
    hdr.cmdp = cdb.data.as_mut_ptr();
    hdr.sbp = sense.as_mut_ptr();
    hdr.timeout = TIMEOUT_MS;

    sg_io_cmd(fd, &mut hdr)?;

    // 读取寄存器状态
    let mut regbuf = [0u8; 16];
    cdb.data[0] = 0xdf;
    cdb.data[1] = 0x10;
    cdb.data[2] = 0x00;
    cdb.data[3] = 0x00;
    cdb.data[4] = regbuf.len() as u8;
    cdb.data[5] = 0x00;
    cdb.data[6] = if (port & 0x04) != 0 { 0x80 } else { 0x90 };
    cdb.data[7] = 0x00;
    cdb.data[8] = 0x00;
    cdb.data[9] = 0x00;
    cdb.data[10] = 0x00;
    cdb.data[11] = 0xfd;

    hdr = SgIoHdr::new();
    hdr.interface_id = b'S' as i32;
    hdr.dxfer_direction = SG_DXFER_FROM_DEV;
    hdr.cmd_len = 12;
    hdr.mx_sb_len = sense.len() as u8;
    hdr.dxfer_len = regbuf.len() as u32;
    hdr.dxferp = regbuf.as_mut_ptr();
    hdr.cmdp = cdb.data.as_mut_ptr();
    hdr.sbp = sense.as_mut_ptr();
    hdr.timeout = TIMEOUT_MS;

    sg_io_cmd(fd, &mut hdr)?;

    // 提取返回寄存器
    registers.data[0] = 0;
    registers.data[2] = regbuf[14]; // STATUS
    registers.data[3] = regbuf[0]; // SECTOR COUNT
    registers.data[7] = regbuf[10]; // LBA HIGH
    registers.data[8] = regbuf[4]; // LBA MID
    registers.data[9] = regbuf[6]; // LBA LOW
    registers.data[10] = regbuf[9]; // DEVICE
    registers.data[11] = regbuf[13]; // ERROR

    Ok(())
}

/// 发送 ATA 命令 (根据磁盘类型选择合适的方法)
pub(crate) fn send_ata_command(
    fd: RawFd,
    disk_type: DiskType,
    command: AtaCommand,
    direction: Direction,
    registers: &mut AtaRegisters,
    data: Option<&mut [u8]>,
) -> Result<()> {
    match disk_type {
        DiskType::AtaPassthrough16 => passthrough_16(fd, command, direction, registers, data),
        DiskType::AtaPassthrough12 => passthrough_12(fd, command, direction, registers, data),
        DiskType::Sunplus => sunplus_command(fd, command, direction, registers, data),
        DiskType::Jmicron => jmicron_command(fd, command, direction, registers, data),
        DiskType::Blob => Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Blob 类型不支持发送命令",
        )
        .into()),
        DiskType::Auto | DiskType::None | DiskType::LinuxIde => {
            Err(std::io::Error::new(std::io::ErrorKind::Unsupported, "不支持的磁盘类型").into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ata_registers() {
        let mut regs = AtaRegisters::new();
        regs.set_features(0x12);
        regs.set_sector_count(0x01);
        regs.set_lba_low(0xAB);
        regs.set_lba_mid(0xCD);
        regs.set_lba_high(0xEF);

        assert_eq!(regs.data[1], 0x12);
        assert_eq!(regs.data[3], 0x01);
        assert_eq!(regs.data[9], 0xAB);
        assert_eq!(regs.data[8], 0xCD);
        assert_eq!(regs.data[7], 0xEF);
    }
}
