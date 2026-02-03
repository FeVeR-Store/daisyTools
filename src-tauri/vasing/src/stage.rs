use std::future::Future;
use std::time::Duration;
use tokio::task::JoinSet;

/// Vasing 任务运行舞台
pub struct Stage {
    /// 任务集合，返回值统一为 Result<(), anyhow::Error>
    set: JoinSet<(String, anyhow::Result<()>)>,
    /// 任务超时时间
    timeout: Duration,
}

impl Stage {
    pub fn new() -> Self {
        Self {
            set: JoinSet::new(),
            timeout: Duration::from_secs(5), // 默认超时
        }
    }

    // 添加支持 Result 的任务 (支持 ?)
    pub fn spawn<F, Fut>(&mut self, name: impl Into<String>, task: F)
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = anyhow::Result<()>> + Send + 'static,
    {
        let name = name.into();
        self.set.spawn(async move {
            let res = task().await;
            (name, res)
        });
    }

    pub async fn run(mut self) {
        let timeout_fut = tokio::time::timeout(self.timeout, async {
            let mut errors = Vec::new();
            while let Some(res) = self.set.join_next().await {
                match res {
                    Err(e) => {
                        if e.is_panic() {
                            std::panic::resume_unwind(e.into_panic());
                        } else if !e.is_cancelled() {
                            errors.push(format!("Task error: {}", e));
                        }
                    }
                    Ok((name, task_res)) => {
                        if let Err(e) = task_res {
                            errors.push(format!("Task '{}' failed: {:?}", name, e));
                            self.set.abort_all(); // 快速失败
                        }
                    }
                }
            }
            if !errors.is_empty() {
                panic!("Vasing Test Failed:\n{}", errors.join("\n"));
            }
        });

        if let Err(_) = timeout_fut.await {
            panic!("Vasing Test Timed Out after {:?}", self.timeout);
        }
    }
}
