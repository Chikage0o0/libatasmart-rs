//! 磁盘设备类型检测

use crate::error::Result;
use crate::ffi::ata::{AtaCommand, Direction};
use crate::ffi::commands::{send_ata_command, AtaRegisters};
use crate::types::DiskType;
use std::os::unix::io::RawFd;

/// 尝试发送 IDENTIFY DEVICE 命令
///
/// 如果成功读取到有效数据,返回识别数据
fn try_identify_device(fd: RawFd, disk_type: DiskType) -> Result<[u8; 512]> {
    let mut registers = AtaRegisters::new();
    let mut identify_data = [0u8; 512];

    // 准备 IDENTIFY DEVICE 命令
    // SECTOR COUNT = 1
    registers.set_sector_count(1);

    // 发送命令
    send_ata_command(
        fd,
        disk_type,
        AtaCommand::IdentifyDevice,
        Direction::In,
        &mut registers,
        Some(&mut identify_data),
    )?;

    // 验证数据不全为 0
    if identify_data.iter().all(|&b| b == 0) {
        return Err(
            std::io::Error::new(std::io::ErrorKind::InvalidData, "IDENTIFY 数据全为 0").into(),
        );
    }

    Ok(identify_data)
}

/// 自动检测磁盘类型
///
/// 依次尝试不同的命令接口,找到第一个能成功执行 IDENTIFY DEVICE 的类型
///
/// # 检测顺序
/// 1. ATA Passthrough 16 (最常用,现代 SATA 硬盘)
/// 2. ATA Passthrough 12 (USB 外置硬盘)
///
/// # 返回值
/// - 成功: 返回检测到的磁盘类型
/// - 失败: 如果所有类型都失败,返回 `DiskType::None`
pub(crate) fn detect_disk_type(fd: RawFd) -> Result<DiskType> {
    // 要测试的磁盘类型列表 (按优先级排序)
    let types_to_test = [DiskType::AtaPassthrough16, DiskType::AtaPassthrough12];

    for disk_type in types_to_test {
        // 尝试发送 IDENTIFY DEVICE 命令
        if try_identify_device(fd, disk_type).is_ok() {
            return Ok(disk_type);
        }
        // 如果失败,继续尝试下一个类型
    }

    // 所有类型都失败,返回 None
    Ok(DiskType::None)
}

/// 发送 IDENTIFY DEVICE 命令并返回识别数据
///
/// # 参数
/// - `fd`: 文件描述符
/// - `disk_type`: 磁盘类型
///
/// # 返回值
/// 成功返回 512 字节的 IDENTIFY 数据
pub(crate) fn identify_device(fd: RawFd, disk_type: DiskType) -> Result<[u8; 512]> {
    if disk_type == DiskType::Blob {
        // Blob 类型不支持发送命令
        return Ok([0u8; 512]);
    }

    try_identify_device(fd, disk_type)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identify_device_blob() {
        // Blob 类型应该返回空数据
        let result = identify_device(-1, DiskType::Blob);
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.len(), 512);
    }
}
