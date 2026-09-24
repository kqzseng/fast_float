use qtx_float::{float, Float};

fn main() {
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