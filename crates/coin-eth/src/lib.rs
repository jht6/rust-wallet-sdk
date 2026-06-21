//! coin-eth —— Ethereum 钱包实现

use alloy_primitives::Address;
use async_trait::async_trait;
use coin_base::common::*;
use coin_base::error::WalletError;
use coin_base::wallet::Wallet;
use k256::elliptic_curve::sec1::ToEncodedPoint;
use k256::SecretKey;

/// Ethereum 钱包
pub struct EthWallet;

impl EthWallet {
    pub fn new() -> Self {
        Self
    }
}

impl Default for EthWallet {
    fn default() -> Self {
        Self::new()
    }
}

// ── 辅助函数 ─────────────────────────────────────────

/// 去掉 0x 前缀并校验 hex 格式
fn strip_hex_prefix(hex_str: &str) -> Option<&str> {
    hex_str
        .strip_prefix("0x")
        .or_else(|| hex_str.strip_prefix("0X"))
        .or(Some(hex_str))
}

/// 验证私钥 hex 字符串
fn valid_private_key(hex_str: &str) -> bool {
    let hex_str = strip_hex_prefix(hex_str).unwrap_or(hex_str);
    if hex_str.len() != 64 || !hex_str.chars().all(|c| c.is_ascii_hexdigit()) {
        return false;
    }
    let bytes = match hex::decode(hex_str) {
        Ok(b) => b,
        Err(_) => return false,
    };
    if bytes.len() != 32 || bytes.iter().all(|b| *b == 0) {
        return false;
    }
    SecretKey::from_slice(&bytes).is_ok()
}

/// 从私钥推导地址和公钥
fn derive_address(pk_hex: &str) -> Result<NewAddressData, WalletError> {
    let raw = hex::decode(pk_hex.trim_start_matches("0x").trim_start_matches("0X").to_lowercase())
        .map_err(|_| WalletError::NewAddress)?;
    let sk = SecretKey::from_slice(&raw).map_err(|_| WalletError::NewAddress)?;
    let pk_bytes = sk.public_key().to_encoded_point(false);
    let addr = Address::from_raw_public_key(&pk_bytes.as_bytes()[1..]);

    Ok(NewAddressData {
        address: format!("0x{}", hex::encode(addr.0)),
        public_key: Some(format!("0x{}", hex::encode(pk_bytes.as_bytes()))),
        compressed_public_key: None,
    })
}

// ── Wallet trait 实现 ─────────────────────────────────

#[async_trait(?Send)]
impl Wallet for EthWallet {
    async fn get_derived_path(&self, param: GetDerivedPathParam) -> Result<String, WalletError> {
        let path = format!("m/44'/60'/0'/0/{}", param.index);
        Ok(path)
    }

    async fn get_new_address(
        &self,
        param: NewAddressParams,
    ) -> Result<NewAddressData, WalletError> {
        if !valid_private_key(&param.private_key) {
            return Err(WalletError::Custom("invalid key".into()));
        }
        derive_address(&param.private_key)
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

    const TEST_KEY: &str = "3c9229289a6125f7fdf1885a77bb12c37a8d3b4962d936f7e3084dece32a3ca1";

    #[test]
    fn test_valid_private_key() {
        assert!(valid_private_key(TEST_KEY));
        assert!(!valid_private_key("0x"));
        assert!(!valid_private_key("abc"));
        assert!(!valid_private_key("0000000000000000000000000000000000000000000000000000000000000000"));
    }

    #[test]
    fn test_derive_address_format() {
        let result = derive_address(TEST_KEY).unwrap();
        assert_eq!(result.address.len(), 42);
        assert!(result.address.starts_with("0x"));
        let pk = result.public_key.as_deref().unwrap();
        assert_eq!(pk.len(), 132);
        assert!(pk.starts_with("0x04"));
    }

    #[test]
    fn test_derive_address_deterministic() {
        let r1 = derive_address(TEST_KEY).unwrap();
        let r2 = derive_address(TEST_KEY).unwrap();
        assert_eq!(r1.address, r2.address);
    }

    #[test]
    fn test_different_keys_different_addresses() {
        let other = "1".repeat(64);
        let r1 = derive_address(TEST_KEY).unwrap();
        let r2 = derive_address(&other).unwrap();
        assert_ne!(r1.address, r2.address);
    }

    #[tokio::test]
    async fn test_get_new_address_rejects_invalid_key() {
        let wallet = EthWallet::new();
        let result = wallet
            .get_new_address(NewAddressParams {
                private_key: "0xdead".into(),
                address_type: None,
                version: None,
                hrp: None,
            })
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_new_address_accepts_no_prefix() {
        let wallet = EthWallet::new();
        let result = wallet
            .get_new_address(NewAddressParams {
                private_key: TEST_KEY.into(),
                address_type: None,
                version: None,
                hrp: None,
            })
            .await;
        assert!(result.is_ok());
    }

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
    }
}
