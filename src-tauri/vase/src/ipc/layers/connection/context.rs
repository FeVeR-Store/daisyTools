use std::{collections::HashMap, sync::Arc};

use serde_json::Value;
use tokio::sync::{Mutex, RwLock, oneshot};
use uuid::Uuid;

use crate::ipc::{
    device::traits::DeviceConfig,
    envelope::{MsgKind, flags::Flags, meta::Metadata},
};

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ConnectionContext {
    pub inflight: Option<Arc<Mutex<HashMap<Uuid, oneshot::Sender<Value>>>>>,
    pub package_routes: Option<Arc<RwLock<HashMap<String, Uuid>>>>,
    pub corr: Uuid,
    pub session_id: Uuid,
    pub flags: Flags,
    pub meta: Metadata,
    pub msg_kind: MsgKind,
    pub map: HashMap<String, Value>,
    // 传输层句柄（用于本地验证获取 PID 等）
    pub transport: Option<Arc<dyn std::any::Any + Send + Sync>>,
    // 协商后的会话密钥 (send_key, recv_key, nonce_prefix)
    pub session_keys: Option<([u8; 32], [u8; 32], [u8; 4])>,
}

#[allow(dead_code)]
impl ConnectionContext {
    pub fn from_device_config(config: &impl DeviceConfig) -> Self {
        Self {
            inflight: None,
            package_routes: None,
            corr: config.corr(),
            session_id: Uuid::max(),
            flags: config.flags(),
            meta: config.meta(),
            map: HashMap::new(),
            msg_kind: MsgKind::RpcRequest,
            transport: None,
            session_keys: None,
        }
    }
    pub fn new_broadcast() -> Self {
        Self {
            package_routes: None,
            inflight: None,
            corr: Uuid::nil(),
            session_id: Uuid::max(),
            flags: Flags::default(),
            meta: Metadata::default(),
            map: HashMap::new(),
            msg_kind: MsgKind::Event,
            transport: None,
            session_keys: None,
        }
    }
}
