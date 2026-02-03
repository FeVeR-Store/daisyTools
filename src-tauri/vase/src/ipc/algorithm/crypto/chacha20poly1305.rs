use aead::{AeadMut, KeyInit, Payload};
use bytes::Bytes;
use chacha20poly1305::Nonce;

use crate::ipc::{self, algorithm::crypto::traits::Crypto, error::Error};

pub struct AlgChaCha20Poly1305;

impl Crypto for AlgChaCha20Poly1305 {
    fn encrypt<'msg, 'aad>(
        &self,
        key: [u8; 32],
        nonce: [u8; 12],
        payload: impl Into<Payload<'msg, 'aad>>,
    ) -> ipc::error::Result<bytes::Bytes> {
        let mut cipher = chacha20poly1305::ChaCha20Poly1305::new(&key.into());
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
        ciphertext: impl Into<Payload<'msg, 'aad>>,
    ) -> ipc::error::Result<Bytes> {
        let mut cipher = chacha20poly1305::ChaCha20Poly1305::new(&key.into());
        let nonce = Nonce::from_slice(&nonce);
        let pt = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| Error::Crypto(e.to_string()))?;

        Ok(Bytes::from(pt))
    }
}
