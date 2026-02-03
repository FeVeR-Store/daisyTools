use crate::ipc;

pub trait Middleware {
    type Input;
    type Output;
    fn inbound(&self, input: Self::Input) -> ipc::error::Result<Self::Output>;
    fn outbound(&self, output: Self::Output) -> ipc::error::Result<Self::Input>;
}
