//! ioctl 系统调用封装
//!
//! 使用 libc 提供的 ioctl 系统调用

use super::scsi::SgIoHdr;
use std::os::unix::io::RawFd;

// ioctl 请求码定义
// 参考: https://docs.kernel.org/userspace-api/ioctl/ioctl-number.html

// 在不同平台上，ioctl 的 request 参数类型不同
// - glibc: c_ulong (32位系统: u32, 64位系统: u64)
// - musl: c_int (i32)
#[cfg(target_env = "musl")]
type IoctlRequest = libc::c_int;

#[cfg(not(target_env = "musl"))]
type IoctlRequest = libc::c_ulong;

/// HDIO_DRIVE_CMD - IDE 驱动器命令
/// 请求码: 0x031f
const HDIO_DRIVE_CMD: IoctlRequest = 0x031f;

/// HDIO_DRIVE_TASK - IDE 驱动器任务
/// 请求码: 0x031e  
const HDIO_DRIVE_TASK: IoctlRequest = 0x031e;

/// SG_IO - SCSI 通用 I/O
/// 请求码: 0x2285
const SG_IO: IoctlRequest = 0x2285;

/// BLKGETSIZE64 - 获取块设备大小
/// 请求码: 0x80081272
#[cfg(target_env = "musl")]
const BLKGETSIZE64: IoctlRequest = 0x80081272u32 as i32;

#[cfg(not(target_env = "musl"))]
const BLKGETSIZE64: IoctlRequest = 0x80081272;

/// 底层 ioctl 调用封装
unsafe fn raw_ioctl<T>(fd: RawFd, request: IoctlRequest, arg: *mut T) -> std::io::Result<()> {
    let ret = libc::ioctl(fd, request, arg);
    if ret == -1 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(())
}

/// 安全的 HDIO_DRIVE_CMD 封装
pub(crate) fn drive_cmd(fd: RawFd, data: &mut [u8]) -> std::io::Result<()> {
    assert!(data.len() >= 4, "数据缓冲区至少需要 4 字节");

    unsafe { raw_ioctl(fd, HDIO_DRIVE_CMD, data.as_mut_ptr() as *mut [u8; 4]) }
}

/// 安全的 HDIO_DRIVE_TASK 封装
pub(crate) fn drive_task(fd: RawFd, data: &mut [u8; 7]) -> std::io::Result<()> {
    unsafe { raw_ioctl(fd, HDIO_DRIVE_TASK, data as *mut [u8; 7]) }
}

/// 安全的 SG_IO 封装
pub(crate) fn sg_io_cmd(fd: RawFd, hdr: &mut SgIoHdr) -> std::io::Result<()> {
    unsafe { raw_ioctl(fd, SG_IO, hdr as *mut SgIoHdr) }
}

/// 安全的 BLKGETSIZE64 封装
pub(crate) fn get_block_size(fd: RawFd) -> std::io::Result<u64> {
    let mut size: u64 = 0;
    unsafe {
        raw_ioctl(fd, BLKGETSIZE64, &mut size as *mut u64)?;
    }
    Ok(size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ioctl_definitions() {
        // 这些测试只是确保 ioctl 定义能够正确编译
        // 实际的功能测试需要真实的设备
    }

    #[test]
    #[should_panic(expected = "数据缓冲区至少需要 4 字节")]
    fn test_drive_cmd_buffer_size() {
        let mut data = [0u8; 2];
        let _ = drive_cmd(-1, &mut data);
    }
}
