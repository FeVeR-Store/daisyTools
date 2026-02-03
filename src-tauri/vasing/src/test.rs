#[vasing_macro::test]
mod test {
    use std::sync::atomic::{AtomicI16, Ordering};

    signal!(sig1);
    signal!(sig2);

    static STATE: AtomicI16 = AtomicI16::new(0);

    #[spawn]
    async fn test() -> Result<(), Box<dyn std::error::Error>> {
        pending!(sig1);
        assert_eq!(STATE.load(Ordering::SeqCst), 2i16);
        STATE.store(3, Ordering::SeqCst);
        resolve!(sig2);
        Ok(())
    }
    #[spawn]
    async fn test() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(STATE.load(Ordering::SeqCst), 0i16);
        STATE.store(2, Ordering::SeqCst);
        resolve!(sig1);
        pending!(sig2);
        assert_eq!(STATE.load(Ordering::SeqCst), 3i16);
        Ok(())
    }
}
