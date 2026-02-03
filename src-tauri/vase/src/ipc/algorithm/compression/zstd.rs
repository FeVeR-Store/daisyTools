use bytes::{Buf, Bytes};

use crate::ipc::algorithm::compression::traits::Compression;

pub struct AlgZstd;
impl Compression for AlgZstd {
    fn compress(&self, payload: Bytes, level: Option<i32>) -> crate::ipc::error::Result<Bytes> {
        let compressed = zstd::stream::encode_all(payload.reader(), level.unwrap_or(0))?;
        Ok(Bytes::from(compressed))
    }
    fn decompress(&self, payload: Bytes) -> crate::ipc::error::Result<Bytes> {
        let decompressed = zstd::stream::decode_all(payload.reader())?;
        Ok(Bytes::from(decompressed))
    }
}
