use aead::Payload;
use bytes::Bytes;

use crate::ipc::{
    self,
    algorithm::crypto::{aes256gcm::AlgAes256Gcm, chacha20poly1305::AlgChaCha20Poly1305},
    envelope::meta::EncryptionAlgorithm,
};

pub trait Crypto {
    fn encrypt<'msg, 'aad>(
        &self,
        key: [u8; 32],
        nonce: [u8; 12],
        payload: impl Into<Payload<'msg, 'aad>>,
    ) -> ipc::error::Result<Bytes>;
    fn decrypt<'msg, 'aad>(
        &self,
        key: [u8; 32],
        nonce: [u8; 12],
        ciphertext: impl Into<Payload<'msg, 'aad>>,
    ) -> ipc::error::Result<Bytes>;
}

impl Crypto for EncryptionAlgorithm {
    fn encrypt<'msg, 'aad>(
        &self,
        key: [u8; 32],
        nonce: [u8; 12],
        payload: impl Into<Payload<'msg, 'aad>>,
    ) -> ipc::error::Result<Bytes> {
        use EncryptionAlgorithm::*;
        match self {
            Aes256gcm => AlgAes256Gcm.encrypt(key, nonce, payload),
            Chacha20poly1305 => AlgChaCha20Poly1305.encrypt(key, nonce, payload),
        }
    }
    fn decrypt<'msg, 'aad>(
        &self,
        key: [u8; 32],
        nonce: [u8; 12],
        ciphertext: impl Into<Payload<'msg, 'aad>>,
    ) -> ipc::error::Result<Bytes> {
        use EncryptionAlgorithm::*;
        match self {
            Aes256gcm => AlgAes256Gcm.decrypt(key, nonce, ciphertext),
            Chacha20poly1305 => AlgChaCha20Poly1305.decrypt(key, nonce, ciphertext),
        }
    }
}
