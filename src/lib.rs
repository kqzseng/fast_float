//! # 高性能固定精度定点数库 (Float)
//!
//! 本 Crate 提供了一个面向金融与高频盘口数据设计的**高性能固定精度定点数**类型。
//! 其核心设计通过内部的 `i64` 整数和 `i8` 的十进制缩放因子（`scale`）来表示高精度数字，
//! 并且实现了**零堆分配（Zero Heap Allocation）**。

// 导出内部模块的公共接口
pub use core::Float;
pub mod core;
pub mod ops;
pub mod parse;