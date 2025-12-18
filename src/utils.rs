//! 工具函数模块

/// 交换字符串中的字节对 (用于处理 ATA IDENTIFY 数据)
pub(crate) fn swap_string_bytes(s: &mut [u8]) {
    assert!(s.len().is_multiple_of(2), "字符串长度必须是偶数");

    for chunk in s.chunks_exact_mut(2) {
        chunk.swap(0, 1);
    }
}

/// 清理字符串中的非打印字符
pub(crate) fn clean_string(s: &mut [u8]) {
    for byte in s.iter_mut() {
        if *byte < b' ' || *byte >= 127 {
            *byte = b' ';
        }
    }
}

/// 移除多余的空格
pub(crate) fn trim_spaces(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// 从原始字节读取并处理字符串
pub(crate) fn read_ata_string(raw: &[u8]) -> String {
    let mut buf = raw.to_vec();
    swap_string_bytes(&mut buf);
    clean_string(&mut buf);

    let s = String::from_utf8_lossy(&buf);
    trim_spaces(&s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_string_bytes() {
        let mut data = b"ABCD".to_vec();
        swap_string_bytes(&mut data);
        assert_eq!(&data, b"BADC");
    }

    #[test]
    fn test_clean_string() {
        let mut data = vec![0x01, b'A', 0x7F, b'B'];
        clean_string(&mut data);
        assert_eq!(&data, b" A B");
    }

    #[test]
    fn test_trim_spaces() {
        assert_eq!(trim_spaces("  hello   world  "), "hello world");
    }
}
