use crate::ipc::transport::impls::local_socket_new::LocalSocketTransport;
use vase_macro::{device, handle, listen};

device!(LocalSocketDevice {
    transport: LocalSocketTransport("vase".to_string()),
    package: vase::test::device,
    // compression: Zstd,
    // encryption: Aes256gcm,
    keepAlive,
    batch,
    ..{
        #[event]
        pub struct Message(pub String);

        pub mod Test;
    }
});

#[handle(LocalSocketDevice::Test)]
fn send_message(message: String) -> anyhow::Result<()> {
    println!("{}", message);
    Ok(())
}

#[listen(LocalSocketDevice::Message)]
fn receive_message(message: _) -> anyhow::Result<()> {
    let msg = message.0;
    println!("Received message: {}", msg);
    Ok(())
}

#[vasing::test]
pub mod device_test {
    use serde_json::json;
    use vase_macro::{expose, handle, listen};

    use crate::ipc::device::impls::local_socket::LocalSocketDevice;

    signal!(ready);
    signal!(connected);
    signal!(receive_broadcast);

    #[handle(LocalSocketDevice::Test)]
    fn test_send_message(i: i32) -> anyhow::Result<i32> {
        Ok(i * 2)
    }

    #[listen(LocalSocketDevice::Message)]
    fn test_on_message(msg: _) -> anyhow::Result<()> {
        let msg = msg.0;
        assert_eq!(msg, "Hello, World!".to_string());
        resolve!(receive_broadcast);
        Ok(())
    }

    #[expose(LocalSocketDevice::Test)]
    fn test_expose_message(message: String) -> anyhow::Result<String> {
        Ok(message)
    }

    #[spawn]
    async fn test_device_handle() -> anyhow::Result<()> {
        LocalSocketDevice::setup().await.unwrap();
        resolve!(ready);
        pending!(connected);
        LocalSocketDevice::Message("Hello, World!".to_string())
            .emit()
            .await?;
        let res = LocalSocketDevice::Test::call(
            "vase::test::device",
            "test_expose_message",
            json!("hello"),
        )
        .await
        .unwrap();
        assert_eq!(res, json!("hello"));
        Ok(())
    }
    #[spawn]
    async fn test_device_ref() -> anyhow::Result<()> {
        pending!(ready);
        LocalSocketDevice::setup_ref().await.unwrap();
        let result = LocalSocketDevice::Test::test_send_message(5).await;
        assert_eq!(result.unwrap(), 10);
        resolve!(connected);
        pending!(receive_broadcast);
        Ok(())
    }
}
