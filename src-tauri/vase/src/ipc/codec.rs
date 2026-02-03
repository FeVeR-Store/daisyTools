use bincode::config;
use serde::{Serialize, de::DeserializeOwned};

use crate::ipc::error::Error;

#[inline]
pub fn decode<T: DeserializeOwned>(buf: &[u8]) -> Result<T, Error> {
    let cfg = config::standard();
    let (req, _len): (T, usize) = bincode::serde::decode_from_slice(buf, cfg)?;
    Ok(req)
}

#[inline]
pub fn encode<T: Serialize>(t: &T) -> Result<Vec<u8>, Error> {
    let cfg = config::standard();
    Ok(bincode::serde::encode_to_vec(t, cfg)?)
}
