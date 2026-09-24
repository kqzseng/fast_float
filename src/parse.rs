use std::io::Write;
use crate::Float;

impl Float {
    /// 将标准的十进制数字字符串解析为 `Float` 类型。
    pub fn from_str(s: &str) -> Option<Self> {
        let s = s.trim();
        if s.is_empty() {
            return None;
        }

        let (is_negative, num_part) = if let Some(stripped) = s.strip_prefix('-') {
            (true, stripped)
        } else if let Some(stripped) = s.strip_prefix('+') {
            (false, stripped)
        } else {
            (false, s)
        };

        if let Some(dot_idx) = num_part.find('.') {
            let int_part = &num_part[..dot_idx];
            let frac_part = &num_part[dot_idx + 1..];

            if frac_part.contains('.') {
                return None;
            }

            let scale = frac_part.len() as i8;
            let mut value = if int_part == "0" {
                if frac_part.is_empty() { 0 } else { frac_part.parse::<i64>().ok()? }
            } else {
                let mut buf = [0u8; 25];
                let int_bytes = int_part.as_bytes();
                let frac_bytes = frac_part.as_bytes();
                let total_len = int_bytes.len() + frac_bytes.len();

                if total_len > buf.len() {
                    return None;
                }

                buf[..int_bytes.len()].copy_from_slice(int_bytes);
                buf[int_bytes.len()..total_len].copy_from_slice(frac_bytes);

                let combined_str = std::str::from_utf8(&buf[..total_len]).ok()?;
                combined_str.parse::<i64>().ok()?
            };

            if is_negative {
                value = -value;
            }
            Some(Self::new(value, scale))

        } else {
            if num_part.is_empty() {
                return None;
            }
            let mut value = num_part.parse::<i64>().ok()?;
            if is_negative {
                value = -value;
            }
            Some(Self::new(value, 0))
        }
    }

    /// 从 `f64` 浮点数安全转换，附带防失真算法。
    pub fn from_f64(f: f64) -> Option<Self> {
        if f.is_nan() || f.is_infinite() {
            return None;
        }

        let mut buf = [0u8; 32];
        let mut cursor = std::io::Cursor::new(&mut buf[..]);

        write!(cursor, "{:.16}", f).ok()?;

        let len = cursor.position() as usize;
        let s = std::str::from_utf8(&buf[..len]).ok()?;

        if s.contains('e') || s.contains('E') {
            return Self::from_f64_scientific(f);
        }

        if let Some(dot_idx) = s.find('.') {
            let frac_part = &s[dot_idx + 1..];

            if let Some(zeros_idx) = frac_part.find("0000000") {
                let clean_frac = &frac_part[..zeros_idx];
                let mut buf_clean = [0u8; 32];
                let int_part = &s[..dot_idx];

                let int_bytes = int_part.as_bytes();
                let frac_bytes = clean_frac.as_bytes();
                let total_len = int_bytes.len() + frac_bytes.len() + 1;

                buf_clean[..int_bytes.len()].copy_from_slice(int_bytes);
                buf_clean[int_bytes.len()] = b'.';
                buf_clean[int_bytes.len() + 1..total_len].copy_from_slice(frac_bytes);

                let clean_str = std::str::from_utf8(&buf_clean[..total_len]).ok()?;
                return Self::from_str(clean_str);
            }

            if let Some(nines_idx) = frac_part.find("9999999") {
                let precision = nines_idx;
                let mut buf_round = [0u8; 32];
                let mut cursor_round = std::io::Cursor::new(&mut buf_round[..]);
                write!(cursor_round, "{:.*}", precision, f).ok()?;
                let len_round = cursor_round.position() as usize;
                let round_str = std::str::from_utf8(&buf_round[..len_round]).ok()?;
                return Self::from_str(round_str);
            }
        }

        Self::from_str(s)
    }

    /// 从 `f32` 浮点数转换。
    #[inline]
    pub fn from_f32(f: f32) -> Option<Self> {
        Self::from_f64(f as f64)
    }

    fn from_f64_scientific(f: f64) -> Option<Self> {
        let mut buf = [0u8; 64];
        let mut cursor = std::io::Cursor::new(&mut buf[..]);
        write!(cursor, "{:.14}", f).ok()?;
        let len = cursor.position() as usize;
        let s = std::str::from_utf8(&buf[..len]).ok()?;
        let trimmed = s.trim_end_matches('0').trim_end_matches('.');
        Self::from_str(trimmed)
    }
}
