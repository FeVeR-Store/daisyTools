use tokio::runtime::Handle;

pub fn get_executor() -> Handle {
    Handle::current()
}
