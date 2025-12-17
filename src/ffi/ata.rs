//! ATA 命令定义

/// ATA 命令枚举
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AtaCommand {
    /// IDENTIFY DEVICE 命令
    IdentifyDevice = 0xEC,
    /// IDENTIFY PACKET DEVICE 命令
    IdentifyPacketDevice = 0xA1,
    /// SMART 命令
    Smart = 0xB0,
    /// CHECK POWER MODE 命令
    CheckPowerMode = 0xE5,
}

/// SMART 子命令
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SmartCommand {
    /// 读取 SMART 数据
    ReadData = 0xD0,
    /// 读取 SMART 阈值
    ReadThresholds = 0xD1,
    /// 立即执行离线测试
    ExecuteOfflineImmediate = 0xD4,
    /// 启用 SMART 操作
    EnableOperations = 0xD8,
    /// 禁用 SMART 操作
    DisableOperations = 0xD9,
    /// 返回 SMART 状态
    ReturnStatus = 0xDA,
}

/// ATA 命令方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Direction {
    /// 无数据传输
    None,
    /// 从设备读取数据
    In,
    /// 向设备写入数据
    Out,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ata_command_values() {
        assert_eq!(AtaCommand::IdentifyDevice as u8, 0xEC);
        assert_eq!(AtaCommand::Smart as u8, 0xB0);
    }

    #[test]
    fn test_smart_command_values() {
        assert_eq!(SmartCommand::ReadData as u8, 0xD0);
        assert_eq!(SmartCommand::ReturnStatus as u8, 0xDA);
    }
}
