use std::time::{SystemTime, UNIX_EPOCH};

/// 构建标的唯一代码
pub fn format_code(exchange: &str, symbol: &str) -> String {
    format!("{}:{}", exchange, symbol)
}
/// 获取当前毫秒级时间戳
pub fn get_current_timestamp_millis() -> u64{
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}


#[repr(C)]
#[derive(Default, Hash, Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct ID(u8, u8, u8, u8, u8, u8, u8, u8);

impl From<Vec<u8>> for ID {
    fn from(v: Vec<u8>) -> Self {
        Self{
            0: *v.get(0).unwrap_or(&0),
            1: *v.get(1).unwrap_or(&0),
            2: *v.get(2).unwrap_or(&0),
            3: *v.get(3).unwrap_or(&0),
            4: *v.get(4).unwrap_or(&0),
            5: *v.get(5).unwrap_or(&0),
            6: *v.get(6).unwrap_or(&0),
            7: *v.get(7).unwrap_or(&0),
        }
    }
}
