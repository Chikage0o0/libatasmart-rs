//! ioctl 系统调用封装
//!
//! 使用 nix crate 提供的 ioctl 宏来定义类型安全的 ioctl 调用

use super::scsi::SgIoHdr;
use nix::libc;
use std::os::unix::io::RawFd;

// 使用 nix::ioctl_* 宏定义 ioctl 调用
// 参考: https://docs.rs/nix/latest/nix/macro.ioctl_readwrite.html

/// HDIO_DRIVE_CMD - IDE 驱动器命令 (读写)
/// 请求码: 0x031f
nix::ioctl_readwrite!(hdio_drive_cmd, 0x03, 0x1f, [u8; 4]);

/// HDIO_DRIVE_TASK - IDE 驱动器任务 (写)
/// 请求码: 0x031e  
nix::ioctl_write_ptr!(hdio_drive_task, 0x03, 0x1e, [u8; 7]);

/// SG_IO - SCSI 通用 I/O (读写)
/// 请求码: 0x2285
nix::ioctl_readwrite!(sg_io, 'S', 0x85, SgIoHdr);

/// BLKGETSIZE64 - 获取块设备大小 (读)
/// 请求码: 0x80081272
nix::ioctl_read!(blkgetsize64, 0x12, 114, u64);

/// 安全的 HDIO_DRIVE_CMD 封装
pub(crate) fn drive_cmd(fd: RawFd, data: &mut [u8]) -> nix::Result<()> {
    assert!(data.len() >= 4, "数据缓冲区至少需要 4 字节");

    unsafe {
        hdio_drive_cmd(fd, data.as_mut_ptr() as *mut [u8; 4])?;
    }
    Ok(())
}

/// 安全的 HDIO_DRIVE_TASK 封装
pub(crate) fn drive_task(fd: RawFd, data: &mut [u8; 7]) -> nix::Result<()> {
    unsafe {
        hdio_drive_task(fd, data as *mut [u8; 7])?;
    }
    Ok(())
}

/// 安全的 SG_IO 封装
pub(crate) fn sg_io_cmd(fd: RawFd, hdr: &mut SgIoHdr) -> nix::Result<()> {
    unsafe {
        sg_io(fd, hdr)?;
    }
    Ok(())
}

/// 安全的 BLKGETSIZE64 封装
pub(crate) fn get_block_size(fd: RawFd) -> nix::Result<u64> {
    let mut size: u64 = 0;
    unsafe {
        blkgetsize64(fd, &mut size)?;
    }
    Ok(size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ioctl_definitions() {
        // 这些测试只是确保 ioctl 宏能够正确编译
        // 实际的功能测试需要真实的设备
    }

    #[test]
    #[should_panic(expected = "数据缓冲区至少需要 4 字节")]
    fn test_drive_cmd_buffer_size() {
        let mut data = [0u8; 2];
        let _ = drive_cmd(-1, &mut data);
    }
}
