/// 便利构建定点数的通用宏。
///
/// 支持双参数直接构建，或单参数智能识别字符串、浮点数以及各种原生整型。
#[macro_export]
macro_rules! float {
    ( $value:expr, $scale:expr ) => {
        $crate::Float::new($value as i64, $scale as i8)
    };

    ( $val:expr ) => {{
        trait FloatMacroResolver<Marker> {
            fn __resolve_float(self) -> $crate::Float;
        }

        struct StrMarker;
        impl<T: AsRef<str>> FloatMacroResolver<StrMarker> for T {
            #[inline]
            fn __resolve_float(self) -> $crate::Float {
                $crate::Float::from_str(self.as_ref())
                    .expect("float! macro failed: Invalid decimal string")
            }
        }

        struct NumMarker<N>(N);

        impl FloatMacroResolver<NumMarker<f64>> for f64 { #[inline] fn __resolve_float(self) -> $crate::Float { $crate::Float::from_f64(self).expect("f64 out of range") } }
        impl FloatMacroResolver<NumMarker<f32>> for f32 { #[inline] fn __resolve_float(self) -> $crate::Float { $crate::Float::from_f32(self).expect("f32 out of range") } }
        impl FloatMacroResolver<NumMarker<i64>> for i64 { #[inline] fn __resolve_float(self) -> $crate::Float { $crate::Float::new(self, 0) } }
        impl FloatMacroResolver<NumMarker<i32>> for i32 { #[inline] fn __resolve_float(self) -> $crate::Float { $crate::Float::new(self as i64, 0) } }
        impl FloatMacroResolver<NumMarker<i16>> for i16 { #[inline] fn __resolve_float(self) -> $crate::Float { $crate::Float::new(self as i64, 0) } }
        impl FloatMacroResolver<NumMarker<i8>>  for i8  { #[inline] fn __resolve_float(self) -> $crate::Float { $crate::Float::new(self as i64, 0) } }
        impl FloatMacroResolver<NumMarker<u32>> for u32 { #[inline] fn __resolve_float(self) -> $crate::Float { $crate::Float::new(self as i64, 0) } }
        impl FloatMacroResolver<NumMarker<u16>> for u16 { #[inline] fn __resolve_float(self) -> $crate::Float { $crate::Float::new(self as i64, 0) } }
        impl FloatMacroResolver<NumMarker<u8>>  for u8  { #[inline] fn __resolve_float(self) -> $crate::Float { $crate::Float::new(self as i64, 0) } }
        impl FloatMacroResolver<NumMarker<u64>> for u64 { #[inline] fn __resolve_float(self) -> $crate::Float { $crate::Float::from_u64(self).expect("u64 overflows i64") } }
        impl FloatMacroResolver<NumMarker<usize>> for usize { #[inline] fn __resolve_float(self) -> $crate::Float { $crate::Float::from_usize(self).expect("usize overflows i64") } }

        #[inline(always)]
        fn __execute_resolve<T, M>(val: T) -> $crate::Float
        where
            T: FloatMacroResolver<M>
        {
            val.__resolve_float()
        }

        __execute_resolve($val)
    }};
}


#[cfg(test)]
mod tests {
    use crate::Float;

    #[test]
    pub fn test_float_macro() {
        // 1. 匹配字符串字面量
        let from_str = float!("45.678");
        assert_eq!(from_str, Float::new(45678, 3));

        // 2. 匹配动态 String 变量
        let s_var = String::from("10.00");
        assert_eq!(float!(s_var), Float::new(1000, 2));

        // 3. 匹配失真浮点数 (f64)
        let from_f64 = float!(0.1 + 0.2); // 0.30000000000000004
        assert_eq!(from_f64, Float::new(3, 1)); // 自动去噪还原为 0.3

        // 4. 匹配明确指定类型的浮点数 (f32)
        let from_f32 = float!(1.25f32);
        assert_eq!(from_f32, Float::new(125, 2));

        // 5. 匹配普通整数字面量（Rust 默认推导为 i32）
        let from_int = float!(500);
        assert_eq!(from_int, Float::new(500, 0));

        // 6. 匹配不同长度的各种整型变量
        let v_u8: u8 = 8;
        let v_i128: i32 = -100;
        assert_eq!(float!(v_u8), Float::new(8, 0));
        assert_eq!(float!(v_i128), Float::new(-100, 0));

        // 7. 匹配显式传统双参数
        assert_eq!(float!(123, 4), Float::new(123, 4));
    }
}