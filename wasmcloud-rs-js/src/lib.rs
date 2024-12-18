use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

use wascap::jwt::{validate_token, Component};
use wascap::wasm::extract_claims;

use nkeys::KeyPair;

// HostKey holds public key and seed for server key pair
#[wasm_bindgen]
pub struct HostKey {
    public_key: String,
    seed: String,
}

// HostKey nkeys implementation
#[wasm_bindgen]
impl HostKey {
    #[wasm_bindgen(constructor)]
    // new creates a new nkeys server key pair
    pub fn new() -> HostKey {
        let key_pair = KeyPair::new_server();
        let seed = key_pair.seed().unwrap();
        let public_key = key_pair.public_key();
        HostKey { public_key, seed }
    }

    // public key returns the HostKey public key
    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> String {
        return String::from(&self.public_key);
    }

    // seed returns the HostKey seed
    #[wasm_bindgen(getter)]
    pub fn seed(&self) -> String {
        return String::from(&self.seed);
    }
}

// extract_jwt extracts the jwt token from a wasm module as js_sys::Uint8Array
#[wasm_bindgen]
pub fn extract_jwt(contents: &Uint8Array) -> Result<String, JsValue> {
    let claims = extract_claims(contents.to_vec());
    match claims {
        Ok(token) => Ok(String::from(token.unwrap().jwt)),
        Err(err) => Err(JsValue::from_str(&format!("{}", err))),
    }
}

// validate_jwt validates the jwt token
#[wasm_bindgen]
pub fn validate_jwt(jwt: &str) -> bool {
    let validate = validate_token::<Component>(jwt);
    match validate {
        Ok(validation) if !validation.expired && !validation.cannot_use_yet => true,
        _ => false,
    }
}
