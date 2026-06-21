//! Wallet trait —— 对应 JS SDK 的 abstract class BaseWallet
//!
//! 所有链的钱包实现必须实现此 trait。
//! coin-base 零 WASM 依赖，使用 `serde_json::Value` 表示泛型返回值。
//! WASM crate 中转换为 `JsValue`。

use crate::common::*;
use crate::error::WalletError;
use async_trait::async_trait;

/// 多链钱包基础 trait
///
/// 对应 TypeScript `abstract class BaseWallet`。
/// 使用 `#[async_trait(?Send)]` 因为 WASM 是单线程环境。
#[async_trait(?Send)]
pub trait Wallet {
    // ── 抽象方法：子类型必须实现 ──────────────────────

    /// 由私钥生成新地址
    async fn get_new_address(&self, param: NewAddressParams)
        -> Result<NewAddressData, WalletError>;

    /// 验证地址是否合法
    async fn valid_address(
        &self,
        param: ValidAddressParams,
    ) -> Result<ValidAddressData, WalletError>;

    /// 签名交易
    /// 返回类型用 serde_json::Value（对应 JS 的 any）
    async fn sign_transaction(&self, param: SignTxParams)
        -> Result<serde_json::Value, WalletError>;

    // ── 默认实现方法 ─────────────────────────────────

    /// 生成随机私钥（secp256k1）
    async fn get_random_private_key(&self) -> Result<String, WalletError> {
        Err(WalletError::NotImplemented)
    }

    /// 由助记词派生私钥
    async fn get_derived_private_key(
        &self,
        _param: DerivePriKeyParams,
    ) -> Result<String, WalletError> {
        Err(WalletError::NotImplemented)
    }

    /// 获取 BIP44 派生路径
    async fn get_derived_path(&self, _param: GetDerivedPathParam) -> Result<String, WalletError> {
        Err(WalletError::NotImplemented)
    }

    /// 验证私钥有效性
    async fn valid_private_key(
        &self,
        _param: ValidPrivateKeyParams,
    ) -> Result<serde_json::Value, WalletError> {
        Err(WalletError::NotImplemented)
    }

    /// 签名消息
    async fn sign_message(&self, _param: SignTxParams) -> Result<String, WalletError> {
        Err(WalletError::NotImplemented)
    }

    /// 签名通用消息（支持多种曲线）
    async fn sign_common_msg(&self, _params: SignCommonMsgParams) -> Result<String, WalletError> {
        Err(WalletError::NotImplemented)
    }

    /// 验证消息签名
    async fn verify_message(&self, _param: VerifyMessageParams) -> Result<bool, WalletError> {
        Err(WalletError::NotImplemented)
    }

    /// 从签名恢复公钥
    async fn ec_recover(
        &self,
        _message: TypedMessage,
        _signature: &str,
    ) -> Result<String, WalletError> {
        Err(WalletError::NotImplemented)
    }

    /// 通过公钥获取地址
    async fn get_address_by_public_key(
        &self,
        _param: GetAddressParams,
    ) -> Result<serde_json::Value, WalletError> {
        Err(WalletError::NotImplemented)
    }

    /// 获取 MPC 原始交易
    async fn get_mpc_raw_transaction(
        &self,
        _param: MpcRawTransactionParam,
    ) -> Result<serde_json::Value, WalletError> {
        Err(WalletError::NotImplemented)
    }

    /// 获取 MPC 签名交易
    async fn get_mpc_transaction(
        &self,
        _param: MpcTransactionParam,
    ) -> Result<serde_json::Value, WalletError> {
        Err(WalletError::NotImplemented)
    }

    /// 获取 MPC 签名消息
    async fn get_mpc_signed_message(
        &self,
        _param: MpcMessageParam,
    ) -> Result<serde_json::Value, WalletError> {
        Err(WalletError::NotImplemented)
    }

    /// 获取硬件原始交易
    async fn get_hardware_raw_transaction(
        &self,
        _param: SignTxParams,
    ) -> Result<serde_json::Value, WalletError> {
        Err(WalletError::NotImplemented)
    }

    /// 获取硬件签名交易
    async fn get_hardware_signed_transaction(
        &self,
        _param: HardwareRawTransactionParam,
    ) -> Result<serde_json::Value, WalletError> {
        Err(WalletError::NotImplemented)
    }

    /// 获取硬件消息哈希
    async fn get_hardware_message_hash(
        &self,
        _param: SignTxParams,
    ) -> Result<serde_json::Value, WalletError> {
        Err(WalletError::NotImplemented)
    }

    /// 计算交易哈希
    async fn calc_tx_hash(&self, _param: CalcTxHashParams) -> Result<String, WalletError> {
        Err(WalletError::NotImplemented)
    }

    /// 获取原始交易
    async fn get_raw_transaction(
        &self,
        _param: GetRawTransactionParams,
    ) -> Result<String, WalletError> {
        Err(WalletError::NotImplemented)
    }

    /// 验证签名交易
    async fn valid_signed_transaction(
        &self,
        _param: ValidSignedTransactionParams,
    ) -> Result<serde_json::Value, WalletError> {
        Err(WalletError::NotImplemented)
    }

    /// 估算手续费
    async fn estimate_fee(&self, _param: SignTxParams) -> Result<f64, WalletError> {
        Err(WalletError::NotImplemented)
    }
}
