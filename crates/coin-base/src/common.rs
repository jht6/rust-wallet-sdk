//! 参数和返回类型
//!
//! 将 TypeScript 的 type 定义映射为 Rust struct。
//! 所有类型派生 Serialize/Deserialize（通过 serde）。
//!
//! 注意：coin-base 零 WASM 依赖。WASM crate 中通过
//! serde-wasm-bindgen 做 JsValue ↔ 这些类型的转换。

use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════
// 请求参数类型
// ═══════════════════════════════════════════════════════

/// 派生私钥参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivePriKeyParams {
    pub mnemonic: String,
    #[serde(rename = "hdPath")]
    pub hd_path: String,
}

/// 生成新地址参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAddressParams {
    #[serde(rename = "privateKey")]
    pub private_key: String,
    #[serde(
        rename = "addressType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub address_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hrp: Option<String>,
}

/// 新地址返回数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAddressData {
    pub address: String,
    #[serde(rename = "publicKey", skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(
        rename = "compressedPublicKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub compressed_public_key: Option<String>,
}

/// 验证地址参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidAddressParams {
    pub address: String,
    #[serde(
        rename = "addressType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub address_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hrp: Option<String>,
}

/// 验证地址返回
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidAddressData {
    #[serde(rename = "isValid")]
    pub is_valid: bool,
    pub address: String,
}

/// 签名交易参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignTxParams {
    #[serde(rename = "privateKey")]
    pub private_key: String,
    pub data: serde_json::Value,
}

/// 验证消息参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyMessageParams {
    pub signature: String,
    pub data: serde_json::Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

/// 类型化消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypedMessage {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    pub message: String,
    #[serde(rename = "publicKey", default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
}

/// 验证私钥参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidPrivateKeyParams {
    #[serde(rename = "privateKey")]
    pub private_key: String,
}

/// 验证私钥返回
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidPrivateKeyData {
    #[serde(rename = "isValid")]
    pub is_valid: bool,
    #[serde(rename = "privateKey")]
    pub private_key: String,
}

/// 获取派生路径参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDerivedPathParam {
    pub index: u32,
    #[serde(
        rename = "segwitType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub segwit_type: Option<u32>,
}

/// 通过公钥获取地址参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAddressParams {
    #[serde(rename = "publicKey")]
    pub public_key: String,
    #[serde(
        rename = "addressType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub address_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hrp: Option<String>,
}

/// 计算 TxHash 参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalcTxHashParams {
    pub data: serde_json::Value,
}

/// 获取原始交易参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRawTransactionParams {
    pub data: serde_json::Value,
}

/// 验证签名交易参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidSignedTransactionParams {
    pub tx: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

// ═══════════════════════════════════════════════════════
// 签名类型枚举
// ═══════════════════════════════════════════════════════

/// 签名类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SignType {
    #[default]
    Secp256k1 = 1,
    #[serde(rename = "ECDSA_P256")]
    EcdsaP256 = 2,
    #[serde(rename = "ED25519")]
    Ed25519 = 3,
    #[serde(rename = "StarknetSignType")]
    StarknetSignType = 4,
    #[serde(rename = "TezosSignType")]
    TezosSignType = 5,
}

/// 签名通用消息参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignCommonMsgParams {
    #[serde(rename = "privateKey")]
    pub private_key: String,
    #[serde(
        rename = "privateKeyHex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub private_key_hex: Option<String>,
    #[serde(rename = "publicKey", default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(rename = "signType", default)]
    pub sign_type: SignType,
    pub message: serde_json::Value,
    #[serde(
        rename = "addressType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub address_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hrp: Option<String>,
    #[serde(rename = "chainName", default, skip_serializing_if = "Option::is_none")]
    pub chain_name: Option<String>,
}

// ═══════════════════════════════════════════════════════
// MPC / 硬件 相关类型（骨架，后续扩展）
// ═══════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MpcRawTransactionParam {
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MpcTransactionParam {
    pub raw: String,
    pub sigs: serde_json::Value,
    #[serde(rename = "publicKey", default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MpcMessageParam {
    pub hash: String,
    pub sigs: serde_json::Value,
    #[serde(rename = "publicKey", default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareRawTransactionParam {
    pub raw: String,
    #[serde(rename = "pubKey", default, skip_serializing_if = "Option::is_none")]
    pub pub_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sig: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub v: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r: Option<String>,
}
