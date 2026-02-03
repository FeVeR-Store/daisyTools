use bytes::Bytes;

/// 传输帧的 **Meta（TLV 元数据）** 定义。
///
/// - 编码采用 TLV：`type: u8` + `len: u16(大端)` + `value`；本枚举的每个成员即一个 TLV 条目。
/// - 仅包含 **定长值**（`u*` 或 `[u8; N]`），便于零拷贝解析与一致的长度校验。
/// - 建议把 `meta` 的原始字节整体纳入 AEAD 的 AAD 以获得完整性保护。
/// - 未识别的 `type` 可按协议实现忽略或拒绝（视实现策略）。
#[vase_macro::tlv]
#[derive(Clone)]
pub enum Meta {
    /// TLV 头部——类型字段，固定为 `u8`。
    _type = u8,

    /// TLV 头部——长度字段，固定为 **大端** `u16`，表示 `value` 的字节数。
    _len = u16,

    /// 聚合类型：用于把多个 `Meta` 条目收集为一个已编码的元数据块。
    ///
    /// - 仅用于宏生成容器类型（如 `Metadata`）的场景；
    /// - 与 TLV 编码/解析流程相关，但不对应线上单独的一个 TLV 条目。
    _collect = Metadata,

    // ===== 加密相关 =====
    /// 加密算法。
    ///
    /// - `Aes256gcm = 0`：AES-256-GCM；
    /// - `Chacha20poly1305 = 1`：ChaCha20-Poly1305。
    ///
    /// 仅携带算法标识，具体密钥/nonce 等由其它字段与会话状态确定。
    EncryptionAlgorithm(
        u8! {
            /// AES-256-GCM 加密
            Aes256gcm = 0,
            /// ChaCha20-Poly1305 加密
            Chacha20poly1305 = 1,
        },
    ),

    /// 加密用 Nonce / IV（12 字节）。
    ///
    /// - 由发送方生成，接收方据此完成 AEAD 验证与解密；
    /// - 同一密钥下 **不可复用**；
    /// - 建议由“固定前缀 + 递增计数器”或等价机制构造。
    EncryptionNonce([u8; 12]),

    /// 密钥世代（Key Update）或轮次。
    ///
    /// - 用于标识本帧所使用的密钥版本；
    /// - 便于两端在重键（rekey）过程中进行一致性校验和滚动升级。
    KeyUpdate(u32),

    // ===== 压缩相关 =====
    /// 压缩算法。
    ///
    /// - `Zstd = 0`：Zstandard；
    /// - `Gzip = 1`：Gzip。
    ///
    /// 压缩阈值与级别等策略通常在会话/配置中协商，本字段仅用于逐帧标识是否启用及所用算法。
    CompressionAlgorithm(
        u8! {
            /// Zstandard 压缩
            Zstd = 0,
            /// DEFLATE 压缩
            Gzip = 1,
        },
    ),

    // ===== 批量（Batch）相关 =====
    /// 批量条目数。
    ///
    /// - 标识本帧 `payload` 中包含的逻辑子消息数量；
    /// - 若所有子消息等长，可配合 `BatchStride` 使用；
    /// - 若需要非等长精确边界，建议在 `payload` 内部使用 length-delimited 格式。
    BatchCount(u16),

    /// 批量步长（字节）。
    ///
    /// - 仅在**所有子消息等长**时使用；
    /// - 接收端可据此按固定步长切分子消息，避免额外偏移表。
    BatchStride(u32),

    // ===== 一致性/诊断 =====
    /// 负载长度影子（字节）。
    ///
    /// - 记录 `payload` 的长度副本，可与外层 framing 做交叉校验；
    /// - 推荐纳入 AEAD 的 AAD，防止长度被篡改。
    PayloadLengthShadow(u32),

    /// 追踪 ID（TraceId），128 位。
    ///
    /// - 用于端到端追踪与日志关联；
    /// - 也可拆分为 2×`u64` 实现。
    TraceId(Bytes),

    /// 优先级（Priority）。
    ///
    /// - 用于调度/队列排序等策略；
    /// - 数值越小或越大代表的含义由具体实现约定。
    Priority(u8),

    // ===== 通道迁移（ChangeServer / 数据面切换） =====
    /// 迁移协议。
    ///
    /// - `Quic = 1`：切换到 QUIC 数据面；
    /// - `WebSocket = 2`：切换到 WebSocket 数据面；
    /// - 仅标识目标协议，实际端点与令牌等信息可在会话/控制面下发或放置于业务负载。
    MigrateProtocol(
        u8! {
            /// QUIC 协议
            Quic = 1,
            /// WebSocket 协议
            WebSocket = 2,
        },
    ),
    /// 验证码
    AuthCode(Bytes),
    /// 包名
    PackageName(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_metadata() {
        let bytes = Bytes::copy_from_slice(&666_u32.to_be_bytes());
        let metadata = Meta::BatchCount(6)
            + Meta::CompressionAlgorithm(CompressionAlgorithm::Zstd)
            + Meta::EncryptionNonce([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12])
            + Meta::TraceId(bytes);
        assert!(matches!(metadata.BatchCount, Some(6)));
        assert!(matches!(
            metadata.CompressionAlgorithm,
            Some(CompressionAlgorithm::Zstd)
        ));
        assert!(matches!(
            metadata.EncryptionNonce,
            Some([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12])
        ));
        assert!(matches!(metadata.KeyUpdate, None));
        assert!(matches!(metadata.MigrateProtocol, None));
    }

    #[test]
    pub fn test_metadata_from_bytes() {
        let bytes = Bytes::copy_from_slice(&666_u32.to_be_bytes());
        let metadata = Meta::BatchCount(6)
            + Meta::CompressionAlgorithm(CompressionAlgorithm::Zstd)
            + Meta::EncryptionNonce([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12])
            + Meta::TraceId(bytes);
        let metadata_bytes: Bytes = metadata.into();

        let metadata = Metadata::from_bytes(&metadata_bytes);

        assert!(matches!(metadata.BatchCount, Some(6)));

        assert!(matches!(
            metadata.CompressionAlgorithm,
            Some(CompressionAlgorithm::Zstd)
        ));
        assert!(matches!(
            metadata.EncryptionNonce,
            Some([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12])
        ));
        assert!(matches!(metadata.KeyUpdate, None));
        assert!(matches!(metadata.MigrateProtocol, None));
    }
}
