use bytes::BytesMut;

#[allow(dead_code)]
pub enum StrategyContext {
    Batch {
        total: u16,
        stride: u32,
        buf: BytesMut,
    },
}
