//! SCSI 命令和数据结构

use nix::libc;

/// SCSI 数据传输方向
pub(crate) const SG_DXFER_NONE: i32 = -1;
pub(crate) const SG_DXFER_TO_DEV: i32 = -2;
pub(crate) const SG_DXFER_FROM_DEV: i32 = -3;

/// SG_IO 头结构 (对应 Linux sg_io_hdr_t)
#[repr(C)]
#[derive(Debug)]
pub(crate) struct SgIoHdr {
    pub interface_id: i32,
    pub dxfer_direction: i32,
    pub cmd_len: u8,
    pub mx_sb_len: u8,
    pub iovec_count: u16,
    pub dxfer_len: u32,
    pub dxferp: *mut u8,
    pub cmdp: *mut u8,
    pub sbp: *mut u8,
    pub timeout: u32,
    pub flags: u32,
    pub pack_id: i32,
    pub usr_ptr: *mut libc::c_void,
    pub status: u8,
    pub masked_status: u8,
    pub msg_status: u8,
    pub sb_len_wr: u8,
    pub host_status: u16,
    pub driver_status: u16,
    pub resid: i32,
    pub duration: u32,
    pub info: u32,
}

impl SgIoHdr {
    /// 创建新的 SG_IO 头
    pub(crate) fn new() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// SCSI 命令描述符块 (12 字节)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct ScsiCdb12 {
    pub data: [u8; 12],
}

impl ScsiCdb12 {
    pub(crate) fn new() -> Self {
        Self { data: [0; 12] }
    }
}

/// SCSI 命令描述符块 (16 字节)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct ScsiCdb16 {
    pub data: [u8; 16],
}

impl ScsiCdb16 {
    pub(crate) fn new() -> Self {
        Self { data: [0; 16] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sg_io_hdr_size() {
        // 确保结构体大小正确
        assert!(std::mem::size_of::<SgIoHdr>() > 0);
    }

    #[test]
    fn test_cdb_creation() {
        let cdb12 = ScsiCdb12::new();
        assert_eq!(cdb12.data.len(), 12);

        let cdb16 = ScsiCdb16::new();
        assert_eq!(cdb16.data.len(), 16);
    }
}
