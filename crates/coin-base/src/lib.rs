//! coin-base —— 多链钱包 SDK 公共层
//!
//! 提供：
//! - `Wallet` trait：所有链钱包必须实现的接口
//! - `WalletError`：错误类型
//! - 参数和返回类型 struct
//!
//! 零 WASM 依赖，可被原生 Rust 项目直接使用。

pub mod common;
pub mod error;
pub mod wallet;
