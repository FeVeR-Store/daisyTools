use bytes::{BufMut, BytesMut};
use serde::{Deserialize, Serialize};
use common::tokio::io::{AsyncReadExt, AsyncWriteExt};
use super::error::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub message_type: MessageType,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MessageType {
    Request,
    Response,
    Heartbeat,
    Error,
    Push,  // 新增推送消息类型
}

impl Message {
    pub fn new(message_type: MessageType, content: String) -> Self {
        Self {
            message_type,
            content,
        }
    }

    pub async fn write_to<T: AsyncWriteExt + Unpin>(self, writer: &mut T) -> Result<()> {
        let json = serde_json::to_string(&self)?;
        let len = json.len() as u32;
        let mut buf = BytesMut::with_capacity(4 + len as usize);
        buf.put_u32(len);
        buf.put(json.as_bytes());
        writer.write_all(&buf).await?;
        Ok(())
    }

    pub async fn read_from<T: AsyncReadExt + Unpin>(reader: &mut T) -> Result<Option<Self>> {
        let len = match reader.read_u32().await {
            Ok(n) => n,
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
            Err(e) => return Err(e.into()),
        };

        let mut buf = vec![0; len as usize];
        reader.read_exact(&mut buf).await?;
        let message = serde_json::from_slice(&buf)?;
        Ok(Some(message))
    }
}
