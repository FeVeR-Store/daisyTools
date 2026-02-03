use bytes::Bytes;

use crate::ipc::{
    self,
    algorithm::compression::{gzip::AlgGzip, zstd::AlgZstd},
    envelope::meta::CompressionAlgorithm,
};

pub trait Compression {
    fn compress(&self, payload: Bytes, level: Option<i32>) -> ipc::error::Result<Bytes>;
    fn decompress(&self, payload: Bytes) -> ipc::error::Result<Bytes>;
}

impl Compression for CompressionAlgorithm {
    fn compress(&self, payload: Bytes, level: Option<i32>) -> ipc::error::Result<Bytes> {
        use CompressionAlgorithm::*;
        match self {
            Zstd => AlgZstd.compress(payload, level),
            Gzip => AlgGzip.compress(payload, level),
        }
    }
    fn decompress(&self, payload: Bytes) -> ipc::error::Result<Bytes> {
        use CompressionAlgorithm::*;
        match self {
            Zstd => AlgZstd.decompress(payload),
            Gzip => AlgGzip.decompress(payload),
        }
    }
}
