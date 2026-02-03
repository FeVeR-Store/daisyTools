use aead::{AeadMut, KeyInit};
use aes_gcm::Nonce;
use bytes::Bytes;

use crate::ipc::{self, algorithm::crypto::traits::Crypto, error::Error};

pub struct AlgAes256Gcm;

impl Crypto for AlgAes256Gcm {
    fn encrypt<'msg, 'aad>(
        &self,
        key: [u8; 32],
        nonce: [u8; 12],
        payload: impl Into<aead::Payload<'msg, 'aad>>,
    ) -> ipc::error::Result<bytes::Bytes> {
        let mut cipher = aes_gcm::Aes256Gcm::new(&key.into());
        let nonce = Nonce::from_slice(&nonce);
        let ct = cipher
            .encrypt(nonce, payload)
            .map_err(|e| Error::Crypto(e.to_string()))?;
        Ok(Bytes::from(ct))
    }
    fn decrypt<'msg, 'aad>(
        &self,
        key: [u8; 32],
        nonce: [u8; 12],
        ciphertext: impl Into<aead::Payload<'msg, 'aad>>,
    ) -> ipc::error::Result<Bytes> {
        let mut cipher = aes_gcm::Aes256Gcm::new(&key.into());
        let nonce = Nonce::from_slice(&nonce);
        let pt = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| anyhow::anyhow!("decrypt failed (auth)"))
            .map_err(|e| Error::Crypto(e.to_string()))?;
        Ok(Bytes::from(pt))
    }
}
