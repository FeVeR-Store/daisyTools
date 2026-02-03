use common::executor::AsyncExecutor;

pub struct DefaultExecutor;

impl AsyncExecutor for DefaultExecutor {
    fn block_on<F: std::future::Future<Output = T>, T>(fut: F) -> T {
        tokio::runtime::Runtime::new().unwrap().block_on(fut)
    }
}