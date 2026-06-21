//! coin-eth-wasm —— Ethereum 钱包 WASM 绑定

use coin_base::common::*;
use coin_base::wallet::Wallet;
use coin_eth::EthWallet;
use wasm_bindgen::prelude::*;

// ── 孤儿规则适配：不能用 impl From，用自由函数 ──────

fn to_js_err(e: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&e.to_string())
}

// ── WASM 导出的 EthWallet ────────────────────────────

#[wasm_bindgen]
pub struct EthWalletWasm {
    inner: EthWallet,
}

#[wasm_bindgen]
impl EthWalletWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: EthWallet::new(),
        }
    }

    #[wasm_bindgen]
    pub fn set_mock(&mut self, address: String, public_key: String) {
        self.inner.set_mock(address, public_key);
    }

    #[wasm_bindgen]
    pub async fn get_new_address(&self, param: JsValue) -> Result<JsValue, JsValue> {
        let params: NewAddressParams =
            serde_wasm_bindgen::from_value(param).map_err(to_js_err)?;
        let result = self.inner.get_new_address(params).await.map_err(to_js_err)?;
        serde_wasm_bindgen::to_value(&result).map_err(to_js_err)
    }

    #[wasm_bindgen]
    pub async fn valid_address(&self, param: JsValue) -> Result<JsValue, JsValue> {
        let params: ValidAddressParams =
            serde_wasm_bindgen::from_value(param).map_err(to_js_err)?;
        let result = self.inner.valid_address(params).await.map_err(to_js_err)?;
        serde_wasm_bindgen::to_value(&result).map_err(to_js_err)
    }

    #[wasm_bindgen]
    pub async fn sign_transaction(&self, param: JsValue) -> Result<JsValue, JsValue> {
        let params: SignTxParams =
            serde_wasm_bindgen::from_value(param).map_err(to_js_err)?;
        let result = self.inner.sign_transaction(params).await.map_err(to_js_err)?;
        serde_wasm_bindgen::to_value(&result).map_err(to_js_err)
    }
}
