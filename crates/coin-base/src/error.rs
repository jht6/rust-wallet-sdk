//! 钱包 SDK 错误类型
//!
//! 与 JS SDK 的字符串错误常量一一对应。
//! coin-base 不依赖 wasm-bindgen，实现 `From<WalletError> for String`。
//! WASM crate 中再封装一层 `From<WalletError> for JsValue`。

use serde::Serialize;

/// 钱包 SDK 所有错误
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum WalletError {
    /// 方法未实现
    #[serde(rename = "NotImplemented")]
    NotImplemented,
    /// 生成地址错误
    #[serde(rename = "NewAddress")]
    NewAddress,
    /// 验证地址错误
    #[serde(rename = "ValidAddress")]
    ValidAddress,
    /// 签名交易错误
    #[serde(rename = "SignTx")]
    SignTx,
    /// 签名消息错误
    #[serde(rename = "SignMsg")]
    SignMsg,
    /// 不支持的币种
    #[serde(rename = "UnsupportedCoin")]
    UnsupportedCoin,
    /// 生成私钥错误
    #[serde(rename = "GenPrivateKey")]
    GenPrivateKey,
    /// 生成助记词错误
    #[serde(rename = "GenMnemonic")]
    GenMnemonic,
    /// 派生 HD 路径错误
    #[serde(rename = "DerivePath")]
    DerivePath,
    /// 签名通用消息错误
    #[serde(rename = "SignCommonMsg")]
    SignCommonMsg,
    /// 无效私钥
    #[serde(rename = "InvalidPrivateKey")]
    InvalidPrivateKey,
    /// 计算交易哈希错误
    #[serde(rename = "CalcTxHash")]
    CalcTxHash,
    /// 估算手续费错误
    #[serde(rename = "EstimateFee")]
    EstimateFee,
    /// 自定义错误
    #[serde(rename = "Custom")]
    Custom(String),
}

impl WalletError {
    /// 返回与 JS SDK 完全一致的错误消息字符串
    pub fn message(&self) -> &str {
        match self {
            Self::NotImplemented => "no implementation method",
            Self::NewAddress => "generate address error",
            Self::ValidAddress => "valid address error",
            Self::SignTx => "sign tx error",
            Self::SignMsg => "sign message error",
            Self::UnsupportedCoin => "unsupported currency",
            Self::GenPrivateKey => "generate private key error",
            Self::GenMnemonic => "generate mnemonic error",
            Self::DerivePath => "derive hdPath error",
            Self::SignCommonMsg => "sign common msg error",
            Self::InvalidPrivateKey => "invalid private key error",
            Self::CalcTxHash => "calculate tx hash error",
            Self::EstimateFee => "estimate fee error",
            Self::Custom(msg) => msg,
        }
    }
}

impl std::fmt::Display for WalletError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message())
    }
}

impl std::error::Error for WalletError {}

impl From<WalletError> for String {
    fn from(e: WalletError) -> Self {
        e.message().to_string()
    }
}

impl From<&str> for WalletError {
    fn from(s: &str) -> Self {
        Self::Custom(s.to_string())
    }
}

impl From<String> for WalletError {
    fn from(s: String) -> Self {
        Self::Custom(s)
    }
}
