use bytes::{Buf, BufMut, Bytes, BytesMut};
use serde::{Serialize, de::DeserializeOwned};
use uuid::Uuid;

use crate::ipc::{
    self, codec,
    envelope::{
        flags::{FLAG_LEN, Flags},
        meta::Metadata,
        request::Request,
        response::Response,
    },
    error::Error,
};

pub mod flags;
pub mod meta;
pub mod request;
pub mod response;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransportMode {
    Server,
    Client,
}

#[derive(Debug, Clone, Copy)]
pub enum MsgKind {
    None,
    RpcRequest,
    RpcResponse,
    Event,
    Ping,
    Pong,
    Error,
    ChangeServer,
    Unknown(u8),
}

impl From<u8> for MsgKind {
    fn from(value: u8) -> Self {
        use MsgKind::*;
        match value {
            0 => None,
            1 => RpcRequest,
            2 => RpcResponse,
            3 => Event,
            4 => Ping,
            5 => Pong,
            6 => Error,
            7 => ChangeServer,
            i => Unknown(i),
        }
    }
}
impl Into<u8> for MsgKind {
    fn into(self) -> u8 {
        use MsgKind::*;
        match self {
            None => 0,
            RpcRequest => 1,
            RpcResponse => 2,
            Event => 3,
            Ping => 4,
            Pong => 5,
            Error => 6,
            ChangeServer => 7,
            Unknown(i) => i,
        }
    }
}
impl Default for MsgKind {
    fn default() -> Self {
        MsgKind::None
    }
}

const MSG_KIND_LEN: usize = 1;

type Corr = Uuid;
const CORR_LEN: usize = 16;

const META_LEN: usize = 2;

const PAYLOAD_LEN: usize = 4;

const ENVELOPE_HEADER_LEN: usize = MSG_KIND_LEN + FLAG_LEN + CORR_LEN + META_LEN + PAYLOAD_LEN;

#[derive(Debug)]
pub struct Envelope {
    pub kind: MsgKind, // 1 byte
    pub flags: Flags,
    pub corr: Corr, // 16 bytes, all-zero 表示单向/无响应事件也可以
    pub meta: Metadata,
    pub payload: Bytes, // 内部业务负载：Request<T>/Response<T>/Event<T> 的编码
}

/// MsgKind: u8 | flags:u8 | corr:[u8; 16] | meta_len:u16 | payload_len:u32 | meta_bytes | payload_bytes
impl Envelope {
    pub fn new(kind: MsgKind, payload: Bytes, flags: Option<Flags>) -> Self {
        let corr = Uuid::new_v4();
        Envelope {
            flags: flags.unwrap_or(Flags::NONE),
            meta: Metadata::default(),
            kind,
            corr,
            payload,
        }
    }
    pub fn flags(&mut self) -> &mut Flags {
        &mut self.flags
    }
    pub fn from_request<Pa: Serialize>(payload: Request<Pa>) -> ipc::Result<Self> {
        Ok(Self::new(MsgKind::RpcRequest, payload.to_bytes()?, None))
    }
    pub fn from_response<Resp: Serialize + DeserializeOwned>(
        payload: Response<Resp>,
    ) -> ipc::Result<Self> {
        Ok(Self::new(MsgKind::RpcResponse, payload.to_bytes()?, None))
    }
    pub fn try_into_request<Pa: Serialize + DeserializeOwned>(self) -> ipc::Result<Request<Pa>> {
        let payload: Request<Pa> = codec::decode(&self.payload)?;
        Ok(payload)
    }
    pub fn try_into_response<Resp: Serialize + DeserializeOwned>(
        self,
    ) -> ipc::Result<Response<Resp>> {
        match self.kind {
            MsgKind::Error => {
                let err = codec::decode(&self.payload)?;
                Err(Error::Transport(err))
            }
            MsgKind::RpcResponse => Response::from_bytes::<Resp>(&self.payload),
            _ => Err(Error::MismatchedTypes),
        }
    }

    pub fn from_bytes_vec(bytes: Vec<u8>) -> Result<Self, Error> {
        // 如果必须收 Vec，则先转 Bytes 方便零拷贝切片
        Self::from_bytes(Bytes::from(bytes))
    }

    pub fn from_bytes(mut frame: Bytes) -> Result<Self, Error> {
        if frame.len() < ENVELOPE_HEADER_LEN {
            return Err(Error::Transport("frame too short".into()));
        }

        // 逐字段读取（BE）
        let kind_u8 = frame.get_u8();
        let flags_bits = frame.get_u8();

        let mut corr_arr = [0u8; 16];
        frame.copy_to_slice(&mut corr_arr);

        let meta_len = frame.get_u16() as usize;
        let payload_len = frame.get_u32() as usize;

        // body 是否足够
        if frame.len() < meta_len + payload_len {
            return Err(Error::Transport("truncated body".into()));
        }

        // 切出 meta / payload（零拷贝视图）
        let meta_bytes = frame.split_to(meta_len);
        let payload = frame.split_to(payload_len);

        // 映射类型
        let kind = MsgKind::from(kind_u8); // 或 TryFrom 校验未知值
        let flags = Flags::from_bits_truncate(flags_bits);
        let corr = Uuid::from_bytes(corr_arr);

        let meta = Metadata::from_bytes(&meta_bytes); // 你的 API 如果要 &Bytes，按需改

        Ok(Envelope {
            kind,
            flags,
            corr,
            meta,
            payload,
        })
    }

    pub fn to_bytes(self) -> Bytes {
        let Self {
            kind,
            flags,
            corr,
            meta,
            payload,
        } = self;

        // 取原始 meta 字节
        let meta_bytes = meta.to_bytes(); // 或 meta.freeze() / meta.as_bytes().clone()

        // 上限检查
        let meta_len = u16::try_from(meta_bytes.len()).expect("meta too long for u16");
        let payload_len = u32::try_from(payload.len()).expect("payload too long for u32");

        let mut buf =
            BytesMut::with_capacity(ENVELOPE_HEADER_LEN + meta_bytes.len() + payload.len());

        buf.put_u8(kind.into());
        buf.put_u8(flags.bits());
        buf.extend_from_slice(corr.as_bytes());
        buf.put_u16(meta_len);
        buf.put_u32(payload_len);

        buf.extend_from_slice(&meta_bytes);
        buf.extend_from_slice(&payload);

        buf.freeze()
    }
    pub fn aad(&self) -> Bytes {
        Self::build_aad(&self.flags, &self.kind, &self.corr, &self.meta)
    }

    pub fn build_aad(flags: &Flags, kind: &MsgKind, corr: &Uuid, metadata: &Metadata) -> Bytes {
        let mut buf = BytesMut::new();
        buf.put_u8(flags.bits());
        let kind: u8 = kind.clone().into();
        buf.put_u8(kind);
        buf.extend_from_slice(corr.as_bytes());
        buf.put_u16(metadata.len() as u16);
        buf.put(metadata.clone().to_bytes());
        buf.freeze()
    }

    pub fn get_corr_id_from_bytes(buf: &Bytes) -> Uuid {
        // 1. 切片：跳过前2个，取到第18个(不包含)
        // 注意：如果 buf 长度小于 18，这行代码会 Panic！
        let slice = &buf[2..18];

        // 2. 转成 [u8; 16] 数组
        // try_into() 会自动检查 slice 长度是否等于数组长度
        let bytes: [u8; 16] = slice.try_into().expect("slice length must be 16");

        // 3. 转成 UUID
        Uuid::from_bytes(bytes)
    }

    #[inline]
    pub fn get_msg_kind_from_bytes(buf: &Bytes) -> MsgKind {
        buf[0].into()
    }
}
