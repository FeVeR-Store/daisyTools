use std::sync::Arc;
use tokio::sync::Notify;

/// 跨任务同步信号
/// Cheap to clone (Arc inside)
#[derive(Clone, Default)]
pub struct Signal {
    inner: Arc<Notify>,
}

impl Signal {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Notify::new()),
        }
    }

    /// 发出信号：唤醒一个等待者
    /// 对应 DSL: resolve!(s)
    pub fn resolve(&self) {
        self.inner.notify_one();
    }

    /// 等待信号
    /// 对应 DSL: pending!(s)
    pub async fn pending(&self) {
        self.inner.notified().await;
    }
}
