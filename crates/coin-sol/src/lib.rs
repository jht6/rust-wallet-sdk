//! coin-sol —— Solana 钱包实现
//!
//! 当前为骨架代码，仅 mock 实现 Wallet trait 的抽象方法。
//! 后续替代为真实的 ed25519 签名、Solana 指令构造。

use async_trait::async_trait;
use coin_base::common::*;
use coin_base::error::WalletError;
use coin_base::wallet::Wallet;

/// Solana 钱包
pub struct SolWallet {
    mock_address: Option<String>,
    mock_public_key: Option<String>,
}

impl SolWallet {
    pub fn new() -> Self {
        Self {
            mock_address: None,
            mock_public_key: None,
        }
    }

    pub fn set_mock(&mut self, address: String, public_key: String) {
        self.mock_address = Some(address);
        self.mock_public_key = Some(public_key);
    }
}

impl Default for SolWallet {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait(?Send)]
impl Wallet for SolWallet {
    async fn get_new_address(
        &self,
        _param: NewAddressParams,
    ) -> Result<NewAddressData, WalletError> {
        Ok(NewAddressData {
            address: self.mock_address.clone().unwrap_or_default(),
            public_key: self.mock_public_key.clone(),
            compressed_public_key: None,
        })
    }

    async fn valid_address(
        &self,
        _param: ValidAddressParams,
    ) -> Result<ValidAddressData, WalletError> {
        Err(WalletError::NotImplemented)
    }

    async fn sign_transaction(
        &self,
        _param: SignTxParams,
    ) -> Result<serde_json::Value, WalletError> {
        Err(WalletError::NotImplemented)
    }
}
