//! coin-eth —— Ethereum 钱包实现
//!
//! 当前为骨架代码，仅 mock 实现 Wallet trait 的抽象方法。
//! 后续替代为真实的 secp256k1 地址派生、RLP 编码、EIP-1559 交易构造。

use async_trait::async_trait;
use coin_base::common::*;
use coin_base::error::WalletError;
use coin_base::wallet::Wallet;

/// Ethereum 钱包
pub struct EthWallet {
    mock_address: Option<String>,
    mock_public_key: Option<String>,
}

impl EthWallet {
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

impl Default for EthWallet {
    fn default() -> Self {
        Self::new()
    }
}

// struct GetDerivedPathParam {
//     pub index: u64,
//     pub segwit_type: Option<u8>,
// }

#[async_trait(?Send)]
impl Wallet for EthWallet {
    async fn get_derived_path(&self, param: GetDerivedPathParam) -> Result<String, WalletError> {
        let path = format!("m/44'/60'/0'/0/{}", param.index);
        Ok(path)
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_derived_path() {
        let wallet = EthWallet::new();

        let result = wallet
            .get_derived_path(GetDerivedPathParam {
                index: 0,
                segwit_type: None,
            })
            .await
            .unwrap();
        assert_eq!(result, "m/44'/60'/0'/0/0");

        let result = wallet
            .get_derived_path(GetDerivedPathParam {
                index: 5,
                segwit_type: None,
            })
            .await
            .unwrap();
        assert_eq!(result, "m/44'/60'/0'/0/5");
    }
}
