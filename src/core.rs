/// 核心定点数结构体。
///
/// 通过将十进制数据放大到整数存储在 `value` 中，结合 `scale` 指明小数点位置。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Float {
    /// 放大后的整数值
    pub value: i64,
    /// 十进制缩放因子，代表小数点后的位数
    pub scale: i8,
}

impl Float {
    /// 创建一个新的 `Float` 实例。
    #[inline]
    pub fn new(value: i64, scale: i8) -> Self {
        Self { value, scale }
    }

    /// 从 `i64` 整数创建一个 `Float`（`scale` 为 0）。
    #[inline]
    pub fn from_i64(v: i64) -> Self {
        Self::new(v, 0)
    }

    /// 从 `i32` 整数创建一个 `Float`（`scale` 为 0）。
    #[inline]
    pub fn from_i32(v: i32) -> Self {
        Self::new(v as i64, 0)
    }

    /// 从 `i8` 整数创建一个 `Float`（`scale` 为 0）。
    #[inline]
    pub fn from_i8(v: i8) -> Self {
        Self::new(v as i64, 0)
    }

    /// 从 `u32` 整数创建一个 `Float`（`scale` 为 0）。
    #[inline]
    pub fn from_u32(v: u32) -> Self {
        Self::new(v as i64, 0)
    }

    /// 从 `u8` 整数创建一个 `Float`（`scale` 为 0）。
    #[inline]
    pub fn from_u8(v: u8) -> Self {
        Self::new(v as i64, 0)
    }

    /// 尝试从 `u64` 整数创建一个 `Float`。若超出 `i64::MAX` 则返回 `None`。
    #[inline]
    pub fn from_u64(v: u64) -> Option<Self> {
        if v <= i64::MAX as u64 {
            Some(Self::new(v as i64, 0))
        } else {
            None
        }
    }

    /// 尝试从 `usize` 整数创建一个 `Float`。若超出 `i64::MAX` 则返回 `None`。
    #[inline]
    pub fn from_usize(v: usize) -> Option<Self> {
        if v <= i64::MAX as usize {
            Some(Self::new(v as i64, 0))
        } else {
            None
        }
    }

    /// 内部辅助：提升 scale 位数
    #[inline]
    pub(crate) fn scale_up(value: i64, current_scale: i8, target_scale: i8) -> i64 {
        if current_scale >= target_scale {
            return value;
        }
        let exp = (target_scale - current_scale) as u32;
        value * 10i64.pow(exp)
    }

    /// 内部辅助：获取 scale 对应的 10 进制系数
    #[inline]
    pub(crate) fn get_scale_factor(scale: i8) -> i64 {
        if scale <= 0 {
            return 1;
        }
        10i64.pow(scale as u32)
    }

    /// 转换回原生的 `i64` 类型，小数部分将被直接截断丢弃。
    #[inline]
    pub fn to_i64(&self) -> i64 {
        if self.scale <= 0 {
            self.value
        } else {
            self.value / Self::get_scale_factor(self.scale)
        }
    }

    /// 尝试转换回原生的 `u64` 类型。如果当前值为负数则返回 `None`。
    #[inline]
    pub fn to_u64(&self) -> Option<u64> {
        let val = self.to_i64();
        if val >= 0 {
            Some(val as u64)
        } else {
            None
        }
    }

    /// 尝试转换回原生的 `i32` 类型与安全的溢出检查。
    #[inline]
    pub fn to_i32(&self) -> Option<i32> {
        let val = self.to_i64();
        if val >= i32::MIN as i64 && val <= i32::MAX as i64 {
            Some(val as i32)
        } else {
            None
        }
    }

    /// 尝试转换回原生的 `u32` 类型与安全的溢出检查。
    #[inline]
    pub fn to_u32(&self) -> Option<u32> {
        let val = self.to_i64();
        if val >= 0 && val <= u32::MAX as i64 {
            Some(val as u32)
        } else {
            None
        }
    }

    /// 尝试转换回原生的 `usize` 类型与安全的溢出检查。
    #[inline]
    pub fn to_usize(&self) -> Option<usize> {
        let val = self.to_i64();
        if val >= 0 && val <= usize::MAX as i64 {
            Some(val as usize)
        } else {
            None
        }
    }

    /// 转换回原生的 `f64` 类型。
    #[inline]
    pub fn to_f64(&self) -> f64 {
        if self.scale <= 0 {
            self.value as f64
        } else {
            self.value as f64 / Self::get_scale_factor(self.scale) as f64
        }
    }

    /// 转换回原生的 `f32` 类型。
    #[inline]
    pub fn to_f32(&self) -> f32 {
        self.to_f64() as f32
    }
}

// 标注标准 From / TryFrom 转换实现
impl From<i64> for Float { #[inline] fn from(v: i64) -> Self { Self::new(v, 0) } }
impl From<i32> for Float { #[inline] fn from(v: i32) -> Self { Self::new(v as i64, 0) } }
impl From<i8>  for Float { #[inline] fn from(v: i8)  -> Self { Self::new(v as i64, 0) } }
impl From<u32> for Float { #[inline] fn from(v: u32) -> Self { Self::new(v as i64, 0) } }
impl From<u8>  for Float { #[inline] fn from(v: u8)  -> Self { Self::new(v as i64, 0) } }

impl TryFrom<u64> for Float {
    type Error = &'static str;
    #[inline]
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Self::from_u64(v).ok_or("u64 value overflows i64")
    }
}

impl TryFrom<usize> for Float {
    type Error = &'static str;
    #[inline]
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Self::from_usize(v).ok_or("usize value overflows i64")
    }
}
