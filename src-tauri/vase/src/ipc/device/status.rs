use tokio::sync::Mutex;

use crate::ipc::device::traits::DeviceRef;
use std::sync::Arc;

#[allow(dead_code)]
pub enum DeviceStatus {
    Ready,
    None,
}

#[allow(dead_code)]
pub struct DeviceInstance<DS: DeviceRef> {
    pub status: DeviceStatus,
    pub instance: Option<Arc<Mutex<DS>>>,
}

impl<DS: DeviceRef> Default for DeviceInstance<DS> {
    fn default() -> Self {
        Self {
            status: DeviceStatus::None,
            instance: None,
        }
    }
}
