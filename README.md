# qtx_float



一个面向金融、量化和高频交易（HFT）盘口数据设计的 **极致性能、零堆分配（Zero Heap Allocation）** 固定精度定点数库。

---

##  核心设计优势

- ** 零堆内存分配 (Zero Allocation)**：全库所有解析、转换和数学运算均在栈（Stack）上完成，彻底规避高并发场景下频繁申请操作系统堆内存带来的性能损耗。
- ** 终结浮点数失真**：内置自适应防失真过滤算法，能够精准识别并切除由二进制浮点数（IEEE 754）累加产生的末尾杂音（如将 `0.30000000000000004` 精准还原为 `0.3`）。
- ** 业务级的精度规则**：
    - **纯整数路径**：解析不带小数点的数字（如 `"123000"`）时，尾部的零原样保留，且缩放因子 `scale` 始终为 `0`。
    - **小数路径**：解析带小数点的数字（如 `"123.000"`）时，尾部的零作为精确度严格保留。
- ** 智能四舍五入除法**：除法运算默认保留两个操作数中的最大 `scale`，并对超出部分在整数域内进行精准的四舍五入，规避传统中间层转换的精度偏差。

## 安装使用

在你的项目 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
qtx_float = "0.1.0"
```

## ️ 快速上手示例

### 1. 极致性能解析与防失真

```rust
use qtx_float::Float;

fn main() {
    // 从十进制字符串解析
    let f1 = Float::from_str("123.456").unwrap();
    assert_eq!(f1.value, 123456);
    assert_eq!(f1.scale, 3);

    // 纯整数路径：严格保持 scale 为 0，不删尾部 0
    let f2 = Float::from_str("1230000").unwrap();
    assert_eq!(f2, Float::new(1230000, 0));

    // 经典浮点数二进制失真拯救
    let f_noise = Float::from_f64(0.1 + 0.2).unwrap(); // 实际为 0.30000000000000004
    assert_eq!(f_noise, Float::new(3, 1)); // 完美投影恢复为 0.3！
}
```

### 2. 数学运算与精度对齐

```rust
use qtx_float::Float;

fn main() {
    let mut price = Float::from_str("1500.50").unwrap(); // 1500.50 (scale 2)
    let tick_size = Float::from_str("0.25").unwrap();   // 0.25 (scale 2)

    // 复合赋值运算
    price += tick_size;
    assert_eq!(price, Float::new(150075, 2)); // 1500.75

    // 除法运算：自适应保留最大 scale 精度并四舍五入
    let a = Float::new(200, 2); // 2.00 (scale 2)
    let b = Float::new(3, 0);   // 3 (scale 0)
    // 2.00 / 3 = 0.6666... 结果保留 max_scale(2) 并进位 -> 0.67
    assert_eq!(a / b, Float::new(67, 2));
}
```

### 3. 原生类型安全转换

```rust
use qtx_float::Float;

fn main() {
    let f = Float::new(12345, 2); // 123.45

    assert_eq!(f.to_i64(), 123);        // 整数直接截断小数部分
    assert_eq!(f.to_f64(), 123.45);     // 完美恢复为浮点数
    assert_eq!(f.to_i32(), Some(123));  // 伴随安全的范围溢出检查
}
```

## 性能配置推荐 (生产环境)

为了在生产环境中让编译器最大化地展开内联（Inline）函数，强烈建议在主项目的 `Cargo.toml` 中配置单单元链接时优化（LTO）：

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

##  开源许可证

本项目采用以下双重许可证（Dual-Licensed）任选其一：

- MIT 许可证 ([LICENSE-MIT](LICENSE-MIT) 或 <http://opensource.org>)
- Apache 2.0 许可证 ([LICENSE-APACHE](LICENSE-APACHE) 或 <http://apache.org>)
