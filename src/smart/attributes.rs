//! SMART 属性解析

use crate::types::{AttributeUnit, SmartAttributeParsedData};

/// 属性信息
#[derive(Debug, Clone, Copy)]
pub(crate) struct AttributeInfo {
    pub name: &'static str,
    pub unit: AttributeUnit,
}

/// 属性信息表（256 个条目）
/// 基于 smartmontools 的数据
pub(crate) static ATTRIBUTE_INFO: [Option<AttributeInfo>; 256] = {
    let mut arr: [Option<AttributeInfo>; 256] = [None; 256];

    // 1-13: 基本属性
    arr[1] = Some(AttributeInfo {
        name: "raw-read-error-rate",
        unit: AttributeUnit::None,
    });
    arr[2] = Some(AttributeInfo {
        name: "throughput-performance",
        unit: AttributeUnit::Unknown,
    });
    arr[3] = Some(AttributeInfo {
        name: "spin-up-time",
        unit: AttributeUnit::Milliseconds,
    });
    arr[4] = Some(AttributeInfo {
        name: "start-stop-count",
        unit: AttributeUnit::None,
    });
    arr[5] = Some(AttributeInfo {
        name: "reallocated-sector-count",
        unit: AttributeUnit::Sectors,
    });
    arr[6] = Some(AttributeInfo {
        name: "read-channel-margin",
        unit: AttributeUnit::Unknown,
    });
    arr[7] = Some(AttributeInfo {
        name: "seek-error-rate",
        unit: AttributeUnit::None,
    });
    arr[8] = Some(AttributeInfo {
        name: "seek-time-performance",
        unit: AttributeUnit::Unknown,
    });
    arr[9] = Some(AttributeInfo {
        name: "power-on-hours",
        unit: AttributeUnit::Milliseconds,
    });
    arr[10] = Some(AttributeInfo {
        name: "spin-retry-count",
        unit: AttributeUnit::None,
    });
    arr[11] = Some(AttributeInfo {
        name: "calibration-retry-count",
        unit: AttributeUnit::None,
    });
    arr[12] = Some(AttributeInfo {
        name: "power-cycle-count",
        unit: AttributeUnit::None,
    });
    arr[13] = Some(AttributeInfo {
        name: "read-soft-error-rate",
        unit: AttributeUnit::None,
    });

    // 170-183: SSD 属性
    arr[170] = Some(AttributeInfo {
        name: "available-reserved-space",
        unit: AttributeUnit::Percent,
    });
    arr[171] = Some(AttributeInfo {
        name: "program-fail-count",
        unit: AttributeUnit::None,
    });
    arr[172] = Some(AttributeInfo {
        name: "erase-fail-count",
        unit: AttributeUnit::None,
    });
    arr[175] = Some(AttributeInfo {
        name: "program-fail-count-chip",
        unit: AttributeUnit::None,
    });
    arr[176] = Some(AttributeInfo {
        name: "erase-fail-count-chip",
        unit: AttributeUnit::None,
    });
    arr[177] = Some(AttributeInfo {
        name: "wear-leveling-count",
        unit: AttributeUnit::None,
    });
    arr[178] = Some(AttributeInfo {
        name: "used-reserved-blocks-chip",
        unit: AttributeUnit::None,
    });
    arr[179] = Some(AttributeInfo {
        name: "used-reserved-blocks-total",
        unit: AttributeUnit::None,
    });
    arr[180] = Some(AttributeInfo {
        name: "unused-reserved-blocks",
        unit: AttributeUnit::None,
    });
    arr[181] = Some(AttributeInfo {
        name: "program-fail-count-total",
        unit: AttributeUnit::None,
    });
    arr[182] = Some(AttributeInfo {
        name: "erase-fail-count-total",
        unit: AttributeUnit::None,
    });
    arr[183] = Some(AttributeInfo {
        name: "runtime-bad-block-total",
        unit: AttributeUnit::None,
    });

    // 184-209: 其他属性
    arr[184] = Some(AttributeInfo {
        name: "end-to-end-error",
        unit: AttributeUnit::None,
    });
    arr[187] = Some(AttributeInfo {
        name: "reported-uncorrect",
        unit: AttributeUnit::Sectors,
    });
    arr[188] = Some(AttributeInfo {
        name: "command-timeout",
        unit: AttributeUnit::None,
    });
    arr[189] = Some(AttributeInfo {
        name: "high-fly-writes",
        unit: AttributeUnit::None,
    });
    arr[190] = Some(AttributeInfo {
        name: "airflow-temperature-celsius",
        unit: AttributeUnit::MilliKelvin,
    });
    arr[191] = Some(AttributeInfo {
        name: "g-sense-error-rate",
        unit: AttributeUnit::None,
    });
    arr[192] = Some(AttributeInfo {
        name: "power-off-retract-count",
        unit: AttributeUnit::None,
    });
    arr[193] = Some(AttributeInfo {
        name: "load-cycle-count",
        unit: AttributeUnit::None,
    });
    arr[194] = Some(AttributeInfo {
        name: "temperature-celsius-2",
        unit: AttributeUnit::MilliKelvin,
    });
    arr[195] = Some(AttributeInfo {
        name: "hardware-ecc-recovered",
        unit: AttributeUnit::None,
    });
    arr[196] = Some(AttributeInfo {
        name: "reallocated-event-count",
        unit: AttributeUnit::None,
    });
    arr[197] = Some(AttributeInfo {
        name: "current-pending-sector",
        unit: AttributeUnit::Sectors,
    });
    arr[198] = Some(AttributeInfo {
        name: "offline-uncorrectable",
        unit: AttributeUnit::Sectors,
    });
    arr[199] = Some(AttributeInfo {
        name: "udma-crc-error-count",
        unit: AttributeUnit::None,
    });
    arr[200] = Some(AttributeInfo {
        name: "multi-zone-error-rate",
        unit: AttributeUnit::None,
    });
    arr[201] = Some(AttributeInfo {
        name: "soft-read-error-rate",
        unit: AttributeUnit::None,
    });
    arr[202] = Some(AttributeInfo {
        name: "ta-increase-count",
        unit: AttributeUnit::None,
    });
    arr[203] = Some(AttributeInfo {
        name: "run-out-cancel",
        unit: AttributeUnit::Unknown,
    });
    arr[204] = Some(AttributeInfo {
        name: "shock-count-write-open",
        unit: AttributeUnit::None,
    });
    arr[205] = Some(AttributeInfo {
        name: "shock-rate-write-open",
        unit: AttributeUnit::None,
    });
    arr[206] = Some(AttributeInfo {
        name: "flying-height",
        unit: AttributeUnit::Unknown,
    });
    arr[207] = Some(AttributeInfo {
        name: "spin-high-current",
        unit: AttributeUnit::Unknown,
    });
    arr[208] = Some(AttributeInfo {
        name: "spin-buzz",
        unit: AttributeUnit::Unknown,
    });
    arr[209] = Some(AttributeInfo {
        name: "offline-seek-performance",
        unit: AttributeUnit::Unknown,
    });

    // 220-242: 更多属性
    arr[220] = Some(AttributeInfo {
        name: "disk-shift",
        unit: AttributeUnit::Unknown,
    });
    arr[221] = Some(AttributeInfo {
        name: "g-sense-error-rate-2",
        unit: AttributeUnit::None,
    });
    arr[222] = Some(AttributeInfo {
        name: "loaded-hours",
        unit: AttributeUnit::Milliseconds,
    });
    arr[223] = Some(AttributeInfo {
        name: "load-retry-count",
        unit: AttributeUnit::None,
    });
    arr[224] = Some(AttributeInfo {
        name: "load-friction",
        unit: AttributeUnit::Unknown,
    });
    arr[225] = Some(AttributeInfo {
        name: "load-cycle-count-2",
        unit: AttributeUnit::None,
    });
    arr[226] = Some(AttributeInfo {
        name: "load-in-time",
        unit: AttributeUnit::Milliseconds,
    });
    arr[227] = Some(AttributeInfo {
        name: "torq-amp-count",
        unit: AttributeUnit::None,
    });
    arr[228] = Some(AttributeInfo {
        name: "power-off-retract-count-2",
        unit: AttributeUnit::None,
    });
    arr[230] = Some(AttributeInfo {
        name: "head-amplitude",
        unit: AttributeUnit::Unknown,
    });
    arr[231] = Some(AttributeInfo {
        name: "temperature-celsius",
        unit: AttributeUnit::MilliKelvin,
    });
    arr[232] = Some(AttributeInfo {
        name: "endurance-remaining",
        unit: AttributeUnit::Percent,
    });
    arr[233] = Some(AttributeInfo {
        name: "power-on-seconds-2",
        unit: AttributeUnit::Unknown,
    });
    arr[234] = Some(AttributeInfo {
        name: "uncorrectable-ecc-count",
        unit: AttributeUnit::Sectors,
    });
    arr[235] = Some(AttributeInfo {
        name: "good-block-rate",
        unit: AttributeUnit::Unknown,
    });
    arr[240] = Some(AttributeInfo {
        name: "head-flying-hours",
        unit: AttributeUnit::Milliseconds,
    });
    arr[241] = Some(AttributeInfo {
        name: "total-lbas-written",
        unit: AttributeUnit::Megabytes,
    });
    arr[242] = Some(AttributeInfo {
        name: "total-lbas-read",
        unit: AttributeUnit::Megabytes,
    });
    arr[250] = Some(AttributeInfo {
        name: "read-error-retry-rate",
        unit: AttributeUnit::None,
    });

    arr
};

/// 计算 pretty value
///
/// 根据属性名称和原始值计算格式化后的值
fn make_pretty(attr: &mut SmartAttributeParsedData) {
    if attr.pretty_unit == AttributeUnit::Unknown {
        return;
    }

    // 提取 48 位原始值
    let fourtyeight = u64::from_le_bytes([
        attr.raw[0],
        attr.raw[1],
        attr.raw[2],
        attr.raw[3],
        attr.raw[4],
        attr.raw[5],
        0,
        0,
    ]);

    attr.pretty_value = match attr.name {
        "spin-up-time" => fourtyeight & 0xFFFF,

        "airflow-temperature-celsius" | "temperature-celsius" | "temperature-celsius-2" => {
            (fourtyeight & 0xFFFF) * 1000 + 273150
        }

        "temperature-centi-celsius" => (fourtyeight & 0xFFFF) * 100 + 273150,

        "power-on-minutes" => fourtyeight * 60 * 1000,

        "power-on-seconds" | "power-on-seconds-2" => fourtyeight * 1000,

        "power-on-half-minutes" => fourtyeight * 30 * 1000,

        "power-on-hours" | "loaded-hours" | "head-flying-hours" => {
            (fourtyeight & 0xFFFFFFFF) * 60 * 60 * 1000
        }

        "reallocated-sector-count" | "current-pending-sector" => fourtyeight & 0xFFFFFFFF,

        "endurance-remaining" | "available-reserved-space" => attr.current_value as u64,

        "total-lbas-written" | "total-lbas-read" => {
            // 转换为 MB: LBAs * 65536 * 512 / 1000000
            fourtyeight * 65536 * 512 / 1000000
        }

        "timed-workload-media-wear" | "timed-workload-host-reads" => fourtyeight / 1024,

        "workload-timer" => fourtyeight * 60 * 1000,

        _ => fourtyeight,
    };
}

/// 解析单个属性
///
/// 从 12 字节的属性数据中解析出结构化信息
pub(crate) fn parse_attribute(
    raw_data: &[u8],
    threshold_data: Option<&[u8]>,
    disk_size: u64,
) -> Option<SmartAttributeParsedData> {
    if raw_data.len() < 12 {
        return None;
    }

    let id = raw_data[0];
    if id == 0 {
        return None;
    }

    // 查找属性信息，如果未定义则使用默认值
    let (name, unit) = if let Some(info) = ATTRIBUTE_INFO[id as usize] {
        (info.name, info.unit)
    } else {
        // 未定义的属性，使用通用名称
        let name = Box::leak(format!("attribute-{}", id).into_boxed_str());
        (name as &'static str, AttributeUnit::Unknown)
    };

    // 解析标志位
    let flags = u16::from_le_bytes([raw_data[1], raw_data[2]]);
    let prefailure = (raw_data[1] & 1) != 0;
    let online = (raw_data[1] & 2) != 0;

    // 解析当前值和最差值
    let current_value = raw_data[3];
    let current_value_valid = current_value >= 1 && current_value <= 0xFD;

    let worst_value = raw_data[4];
    let worst_value_valid = worst_value >= 1 && worst_value <= 0xFD;

    // 提取原始值（6 字节）
    let mut raw = [0u8; 6];
    raw.copy_from_slice(&raw_data[5..11]);

    let mut attr = SmartAttributeParsedData {
        id,
        name,
        pretty_unit: unit,
        flags,
        threshold: 0,
        threshold_valid: false,
        online,
        prefailure,
        good_now: true,
        good_now_valid: false,
        good_in_the_past: true,
        good_in_the_past_valid: false,
        current_value_valid,
        worst_value_valid,
        warn: false,
        current_value,
        worst_value,
        pretty_value: 0,
        raw,
    };

    // 计算 pretty value
    make_pretty(&mut attr);

    // 查找并应用阈值
    if let Some(threshold_raw) = threshold_data {
        if threshold_raw.len() >= 2 && threshold_raw[0] == id {
            let threshold = threshold_raw[1];
            attr.threshold = threshold;
            attr.threshold_valid = threshold != 0xFE;

            // 计算健康状态
            if threshold >= 1 && threshold <= 0xFD {
                if worst_value_valid {
                    attr.good_in_the_past = worst_value > threshold;
                    attr.good_in_the_past_valid = true;
                }

                if current_value_valid {
                    attr.good_now = current_value > threshold;
                    attr.good_now_valid = true;
                }
            }

            attr.warn = (attr.good_now_valid && !attr.good_now)
                || (attr.good_in_the_past_valid && !attr.good_in_the_past);
        }
    }

    // 验证属性值
    verify_attribute(&mut attr, disk_size);

    Some(attr)
}

/// 验证属性值的合理性
fn verify_attribute(attr: &mut SmartAttributeParsedData, disk_size: u64) {
    match attr.pretty_unit {
        AttributeUnit::MilliKelvin => {
            // 温度范围：-15°C 到 100°C
            const MIN: u64 = 258150; // -15°C in mK
            const MAX: u64 = 373150; // 100°C in mK
            if attr.pretty_value < MIN || attr.pretty_value > MAX {
                attr.pretty_unit = AttributeUnit::Unknown;
            }
        }

        AttributeUnit::Milliseconds => {
            // 时间范围验证
            const MIN: u64 = 1;
            const SHORT_MAX: u64 = 60 * 60 * 1000; // 1 小时
            const LONG_MAX: u64 = 30 * 365 * 24 * 60 * 60 * 1000; // 30 年

            let max = if attr.name.contains("spin-up") || attr.name.contains("load-in") {
                SHORT_MAX
            } else {
                LONG_MAX
            };

            if attr.pretty_value < MIN || attr.pretty_value > max {
                attr.pretty_unit = AttributeUnit::Unknown;
            }
        }

        AttributeUnit::Sectors => {
            // 扇区数验证
            if disk_size > 0 {
                let max_sectors = disk_size / 512;
                if attr.pretty_value == 0xFFFFFFFF
                    || attr.pretty_value == 0xFFFFFFFFFFFF
                    || attr.pretty_value > max_sectors
                {
                    attr.pretty_unit = AttributeUnit::Unknown;
                } else if (attr.name == "reallocated-sector-count"
                    || attr.name == "current-pending-sector")
                    && attr.pretty_value > 0
                {
                    attr.warn = true;
                }
            }
        }

        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attribute_info_table() {
        assert!(ATTRIBUTE_INFO[1].is_some());
        assert_eq!(ATTRIBUTE_INFO[1].unwrap().name, "raw-read-error-rate");

        assert!(ATTRIBUTE_INFO[9].is_some());
        assert_eq!(ATTRIBUTE_INFO[9].unwrap().name, "power-on-hours");
        assert_eq!(ATTRIBUTE_INFO[9].unwrap().unit, AttributeUnit::Milliseconds);
    }

    #[test]
    fn test_parse_attribute() {
        // 模拟一个属性数据：ID=9 (power-on-hours)
        let mut raw_data = [0u8; 12];
        raw_data[0] = 9; // ID
        raw_data[1] = 0x02; // flags: online
        raw_data[2] = 0x00;
        raw_data[3] = 100; // current value
        raw_data[4] = 100; // worst value
                           // raw[5..11] = 1000 小时（小端序）
        raw_data[5] = 0xE8;
        raw_data[6] = 0x03;

        let attr = parse_attribute(&raw_data, None, 0).unwrap();

        assert_eq!(attr.id, 9);
        assert_eq!(attr.name, "power-on-hours");
        assert_eq!(attr.current_value, 100);
        assert!(attr.online);
        assert!(!attr.prefailure);

        // 1000 小时 = 1000 * 60 * 60 * 1000 毫秒
        assert_eq!(attr.pretty_value, 1000 * 60 * 60 * 1000);
    }
}
