use std::collections::HashMap;

use bytes::Bytes;
use uuid::Uuid;

use crate::{
    ipc::{
        algorithm::compression::traits::Compression,
        envelope::{Envelope, flags::Flags},
        layers::{
            connection::context::ConnectionContext,
            strategy::{context::StrategyContext, session_crypto::SessionCrypto},
            traits::Layer,
        },
    },
    utils::random::random_bytes,
};

#[allow(dead_code)]
pub struct StrategyLayer {
    session_crypto: SessionCrypto,
    context: HashMap<Uuid, StrategyContext>,
}

impl Layer for StrategyLayer {
    type In = Envelope;
    type Out = Bytes;
    async fn inbound(
        &mut self,
        x: Self::In,
        ctx: &mut ConnectionContext,
    ) -> crate::ipc::error::Result<Self::Out> {
        // 构建aad，取出数据
        let aad = x.aad();
        let metadata = x.meta;
        let flag = x.flags;
        let mut payload = x.payload;

        // 尝试从 Context 更新密钥 (如果有鉴权层协商了新密钥)
        if let Some((sk, rk, np)) = ctx.session_keys {
            // 这里简单做一个 check，避免重复重置 (可以通过 flag 或比较判断)
            // 实际生产中应有更严谨的状态机
            // 这里假设鉴权层只在协商完成时设置一次
            self.session_crypto = SessionCrypto::new(sk, rk, np);
            // 拿走密钥避免重复更新？或者保留？保留比较好，context 是共享的
            // ctx.session_keys = None;
        }

        // 处理加密
        if flag.contains(Flags::ENCRYPTED) {
            if let Some(alg) = metadata.EncryptionAlgorithm {
                if let Some(nonce) = metadata.EncryptionNonce {
                    payload = self.session_crypto.decrypt(&aad, nonce, &payload, alg)?;
                }
            };
        }

        // 处理压缩
        if flag.contains(Flags::COMPRESSED) {
            if let Some(alg) = metadata.CompressionAlgorithm {
                payload = alg.decompress(payload)?;
            }
        }

        Ok(payload)
    }
    async fn outbound(
        &mut self,
        x: Self::Out,
        ctx: &mut ConnectionContext,
    ) -> crate::ipc::error::Result<Self::In> {
        let corr = ctx.corr;
        let mut metadata = ctx.meta.clone();
        let flags = ctx.flags.clone();
        let msg_kind = ctx.msg_kind;
        let mut payload = x;

        let nonce = self.session_crypto.next_nonce();
        metadata.EncryptionNonce = Some(nonce);

        let aad = Envelope::build_aad(&flags, &msg_kind, &corr, &metadata);

        if flags.contains(Flags::COMPRESSED) {
            if let Some(alg) = metadata.CompressionAlgorithm {
                payload = alg.compress(payload, None)?;
            }
        }

        if flags.contains(Flags::ENCRYPTED) {
            if let Some(alg) = metadata.EncryptionAlgorithm {
                if let Some(nonce) = metadata.EncryptionNonce {
                    payload = self.session_crypto.encrypt(&aad, nonce, &payload, alg)?;
                }
            };
        }

        let mut env = Envelope::new(msg_kind, payload, Some(flags));
        env.corr = corr;
        env.meta = metadata;
        Ok(env)
    }

    fn new() -> Self {
        let send_key = random_bytes();
        let receive_key = random_bytes();
        let nonce_prefix = random_bytes();

        StrategyLayer {
            session_crypto: SessionCrypto::new(send_key, receive_key, nonce_prefix),
            context: Default::default(),
        }
    }
}
