use serde::{Serialize, Deserialize, Serializer, Deserializer, de::Error};
use std::io::Write;
use crate::Float;

impl Serialize for Float {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut buf = [0u8; 32];
        let mut cursor = std::io::Cursor::new(&mut buf[..]);

        if self.scale <= 0 {
            write!(cursor, "{}", self.value).map_err(serde::ser::Error::custom)?;
        } else {
            let factor = 10i64.pow(self.scale as u32);
            let int_part = self.value / factor;
            let frac_part = (self.value % factor).abs();

            write!(
                cursor,
                "{}.{:0width$}",
                int_part,
                frac_part,
                width = self.scale as usize
            )
                .map_err(serde::ser::Error::custom)?;
        }

        let len = cursor.position() as usize;
        let s = std::str::from_utf8(&buf[..len]).map_err(serde::ser::Error::custom)?;

        serializer.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for Float {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum RawValue {
            Str(String),
            F64(f64),
            I64(i64),
        }

        match RawValue::deserialize(deserializer)? {
            RawValue::Str(s) => Self::from_str(&s)
                .ok_or_else(|| D::Error::custom(format!("Invalid Float string: {}", s))),
            RawValue::F64(f) => Self::from_f64(f)
                .ok_or_else(|| D::Error::custom(format!("Invalid Float f64: {}", f))),
            RawValue::I64(i) => Ok(Self::from_i64(i)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // 引入我们在 lib.rs 里声明的通用 float! 宏进行快捷构建
    use crate::float;

    // 声明一个模拟量化交易的结构体
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct MockOrder {
        id: u64,
        price: Float,
        quantity: Float,
    }

    #[test]
    fn test_serialize_and_deserialize() {
        // ---- 测试 1：正向序列化（Float -> JSON 字符串） ----
        let order = MockOrder {
            id: 8888,
            price: float!("1500.50"),   // 带小数，必须严格保留尾随 0
            quantity: float!("5000"),   // 纯整数，scale 为 0 且保留尾随 0
        };

        // 临时将结构体转换为 JSON 文本
        let json_output = serde_json::to_string(&order).unwrap();

        // 严格断言其输出样式完全契合你定制的精度规则
        assert_eq!(json_output, r#"{"id":8888,"price":"1500.50","quantity":"5000"}"#);

        // ---- 测试 2：反序列化（各种混合类型 JSON -> Float） ----
        // 外部发来的 JSON 可能是字符串，也可能是标准的数字类型
        let incoming_json = r#"{"id":8888,"price":"1500.50","quantity":5000}"#;
        let decoded_order: MockOrder = serde_json::from_str(incoming_json).unwrap();

        assert_eq!(decoded_order.price, Float::new(150050, 2));
        assert_eq!(decoded_order.quantity, Float::new(5000, 0));

        // ---- 测试 3：反序列化中的“防二进制浮点杂音失真”判定 ----
        // 模拟外部传输了由于累加计算溢出的失真浮点数 0.30000000000000004
        let noise_json = r#"{"id":9999,"price":0.30000000000000004,"quantity":"1"}"#;
        let decoded_noise: MockOrder = serde_json::from_str(noise_json).unwrap();

        // 完美复用你之前打磨出的 from_f64 去噪机制，精准收拢
        assert_eq!(decoded_noise.price, Float::new(3, 1)); // 0.3
    }
}