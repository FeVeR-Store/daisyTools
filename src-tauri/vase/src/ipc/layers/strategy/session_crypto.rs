use aead::Payload;
use bytes::Bytes;

use crate::ipc::{self, algorithm::crypto::traits::Crypto};

pub struct SessionCrypto {
    send_key: [u8; 32],    // 会话密钥（握手派生或预共享）
    receive_key: [u8; 32], // 会话密钥（握手派生或预共享）
    nonce_prefix: [u8; 4], // 可用随机前缀 + 64bit 计数器拼 nonce
    counter: u64,
}

impl SessionCrypto {
    pub fn new(send_key: [u8; 32], receive_key: [u8; 32], nonce_prefix: [u8; 4]) -> SessionCrypto {
        SessionCrypto {
            send_key,
            receive_key,
            nonce_prefix,
            counter: 0,
        }
    }
    pub fn next_nonce(&mut self) -> [u8; 12] {
        let mut nonce = [0u8; 12];
        self.counter = self.counter.wrapping_add(1);
        nonce[..4].copy_from_slice(&self.nonce_prefix);
        nonce[4..].copy_from_slice(&self.counter.to_be_bytes());
        nonce
    }
    /// 加密 payload，返回 (ciphertext, nonce)
    pub fn encrypt(
        &mut self,
        aad: &[u8],
        nonce: [u8; 12],
        payload: &[u8],
        alg: impl Crypto,
    ) -> ipc::error::Result<Bytes> {
        let ct = alg.encrypt(self.send_key, nonce, Payload { msg: payload, aad })?;
        Ok(Bytes::from(ct))
    }

    /// 解密
    pub fn decrypt(
        &self,
        aad: &[u8],
        nonce: [u8; 12],
        ciphertext: &[u8],
        alg: impl Crypto,
    ) -> ipc::error::Result<Bytes> {
        let pt = alg.decrypt(
            self.receive_key,
            nonce,
            Payload {
                msg: ciphertext,
                aad,
            },
        )?;
        Ok(pt)
    }
}
