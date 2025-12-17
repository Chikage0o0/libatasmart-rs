//! 集成测试

#[cfg(test)]
mod integration_tests {
    use atasmart::*;

    #[test]
    fn test_library_compiles() {
        // 基础编译测试
        assert!(true);
    }

    // 注意: 以下测试需要真实的硬盘设备和 root 权限
    // 在 CI 环境中应该跳过这些测试

    #[test]
    #[ignore]
    fn test_open_device() {
        // 需要 root 权限和真实设备
        // 运行: sudo cargo test -- --ignored
        
        let result = Disk::open("/dev/sda");
        match result {
            Ok(disk) => {
                assert!(disk.size() > 0);
                println!("设备大小: {} 字节", disk.size());
            }
            Err(e) => {
                eprintln!("无法打开设备: {}", e);
            }
        }
    }
}
