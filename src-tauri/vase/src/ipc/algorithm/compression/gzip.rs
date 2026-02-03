use std::io::{Read, Write};

use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::ipc::algorithm::compression::traits::Compression;

pub struct AlgGzip;

impl Compression for AlgGzip {
    fn compress(
        &self,
        payload: bytes::Bytes,
        level: Option<i32>,
    ) -> crate::ipc::error::Result<bytes::Bytes> {
        let mut encoder = flate2::write::GzEncoder::new(
            BytesMut::new().writer(),
            flate2::Compression::new(level.unwrap_or(6) as u32),
        );
        encoder.write_all(&payload)?;
        let compressed = encoder.finish()?;
        Ok(compressed.into_inner().freeze())
    }
    fn decompress(&self, payload: bytes::Bytes) -> crate::ipc::error::Result<bytes::Bytes> {
        let mut reader = flate2::read::GzDecoder::new(payload.reader());
        let mut buf = Vec::new();
        reader.read(&mut buf)?;
        Ok(Bytes::from(buf))
    }
}
