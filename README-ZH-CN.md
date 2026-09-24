# qtx_float

[![Crates.io](https://shields.io)](https://crates.io)
[![Documentation](https://docs.rs)](https://docs.rs)
[![License](https://shields.io)](#开源许可证)

一个面向金融、量化和高频交易（HFT）盘口数据设计的 极致性能、零堆分配（Zero Heap Allocation） 固定精度定点数库。

---

## 核心设计优势

- **零堆内存分配 (Zero Allocation)**：全库所有解析、转换和数学运算均在栈（Stack）上完成，彻底规避高并发场景下频繁申请操作系统堆内存带来的性能损耗。
- **全类型适配 float! 宏**：无需手动标注类型，单参数智能通杀并分流所有原生格式（字符串、浮点数、各种长度的整型变量或字面量），且具备 零成本运行时开销。
- **终结浮点数失真**：内置自适应防失真过滤算法，能够精准识别并切除由二进制浮点数（IEEE 754）累加产生的末尾杂音（如将 `0.30000000000000004` 精准还原为 `0.3`）。
- **业务级的精度规则**：
  - **纯整数路径**：解析不带小数点的数字（如 `"123000"`）时，尾部的零原样保留，且缩放因子 `scale` 始终为 `0`。
  - **小数路径**：解析带小数点的数字（如 `"123.000"`）时，尾部的零作为精确度严格保留。
- **智能四舍五入除法**：除法运算默认保留两个操作数中的最大 `scale`，并对超出部分在整数域内进行精准的四舍五入，规避传统中间层转换的精度偏差。

## 安装使用

在你的项目 `Cargo.toml` 中添加依赖。默认保持最纯净的轻量化无外部依赖状态，您可以根据业务场景自由开启高级扩展特性：

```toml
[dependencies]
# 1. 核心极速版引入（零额外外部依赖开销）
qtx_float = "0.3.0"

# 2. 混合搭配高级特性：
# qtx_float = { version = "0.3.0", features = ["ser", "pgsql", "mysql"] }
```

### 可用特性说明

- **ser**：开启 Serde 序列化与反序列化支持。
- **pgsql**：开启 SQLx PostgreSQL 数据库无损映射支持（对接 NUMERIC/DECIMAL）。
- **mysql**：开启 SQLx MySQL 数据库无损映射支持（对接 DECIMAL）。

## 快速上手示例

### 1. 使用 float! 宏通杀全类型构建

```rust
use qtx_float::float;
use qtx_float::Float;

fn main() {
    // 字符串字面量自适应解析
    let f_str = float!("123.456"); 
    assert_eq!(f_str, Float::new(123456, 3));

    // 原生浮点数字面量（自动激活防失真去噪）
    let f_noise = float!(0.1 + 0.2); // 底层实际为 0.30000000000000004
    assert_eq!(f_noise, Float::new(3, 1)); // 完美投影恢复为 0.3

    // 原生整型变量或字面量（自动设置 scale 为 0 且保留尾随零）
    let val_u8: u8 = 8;
    assert_eq!(float!(val_u8), Float::new(8, 0));
    assert_eq!(float!(5000), Float::new(5000, 0));

    // 显式传统双参数模式构建 (value, scale)
    assert_eq!(float!(12345, 2), Float::new(12345, 2));
}
```

### 2. 数学运算与复合赋值

```rust
use qtx_float::float;

fn main() {
    let mut price = float!("1500.50"); // 1500.50 (scale 2)
    let tick_size = float!("0.25");    // 0.25 (scale 2)

    // 支持 +=, -=, *=, /=, %= 复合赋值运算
    price += tick_size;
    assert_eq!(price, float!("1500.75")); 

    // 除法运算：自适应保留最大 scale 精度并四舍五入
    let a = float!("2.00"); // 2.00 (scale 2)
    let b = float!("3");    // 3 (scale 0)
    // 2.00 / 3 = 0.6666... 结果保留最大 scale（2位）并进位 -> 0.67
    assert_eq!(a / b, float!(67, 2));
}
```

### 3. 可选特性：Serde 序列化应用 (features = ["ser"])

启用 `ser` 特性后，`Float` 在外部会展现为规范的十进制字符串（规避前端解析大数精度截断），且反序列化完美适配各种混合类型：

```rust
use qtx_float::float;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Order {
    id: u64,
    price: qtx_float::Float,
}

fn main() {
    // 序列化测试：自动转换输出为带精确精度的十进制字符串
    let order = Order { id: 1, price: float!("15.50") };
    let json = serde_json::to_string(&order).unwrap();
    assert_eq!(json, r#"{"id":1,"price":"15.50"}"#);

    // 反序列化测试：支持从混合数字类型或带失真的二进制浮点数中安全去噪还原
    let incoming = r#"{"id":2,"price":0.30000000000000004}"#;
    let decoded: Order = serde_json::from_str(incoming).unwrap();
    assert_eq!(decoded.price, float!("0.3"));
}
```

### 4. 数据库联调应用 (features = ["pgsql"] / ["mysql"])

开启对应数据库 Feature 后，`Float` 即可作为数据表中的高精度定点数（NUMERIC/DECIMAL）进行无堆分配的双向无损读写，完美兼容 SQLx 的 `FromRow` 宏和参数绑定：

```rust
// 完美的行模型映射，彻底消除 Decode 报错风险
#[derive(sqlx::FromRow, Debug, PartialEq)]
struct DBStockItem {
    id: i64,
    symbol: String,
    price: qtx_float::Float, // 自动绑定为数据库高精度定点数
}

// 1. PostgreSQL 绑定调用示例 (features = ["pgsql"])
async fn pg_example(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO items (price) VALUES ($1)").bind(float!("68500.2500")).execute(pool).await?;
    let item: DBStockItem = sqlx::query_as("SELECT id, symbol, price FROM items LIMIT 1").fetch_one(pool).await?;
    Ok(())
}

// 2. MySQL 绑定调用示例 (features = ["mysql"])
async fn mysql_example(pool: &sqlx::MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO items (price) VALUES (?)").bind(float!("68500.2500")).execute(pool).await?;
    let item: DBStockItem = sqlx::query_as("SELECT id, symbol, price FROM items LIMIT 1").fetch_one(pool).await?;
    Ok(())
}
```

## 性能配置推荐 (生产环境)

为了让编译器跨模块/跨文件最大化地展开内联（Inline）代码，强烈建议在主项目的 `Cargo.toml` 中配置单单元链接时优化（LTO）：

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

## 开源许可证

本项目采用以下双重许可证（Dual-Licensed）任选其一：

- MIT 许可证 ([LICENSE-MIT](LICENSE-MIT) 或 <http://opensource.org>)
- Apache 2.0 许可证 ([LICENSE-APACHE](LICENSE-APACHE) 或 <http://apache.org>)
