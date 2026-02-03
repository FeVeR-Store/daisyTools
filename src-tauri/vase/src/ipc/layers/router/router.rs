use std::{collections::HashMap, pin::Pin, sync::LazyLock};

pub type BoxFut<T> = Pin<Box<dyn Future<Output = T> + Send>>;

#[derive(Clone, Copy)]
// Server 端 Handler (处理 #[handle])
pub struct HandlerRegistration {
    pub module: &'static str,
    pub name: &'static str,
    pub func: fn(req: serde_json::Value) -> BoxFut<Result<serde_json::Value, anyhow::Error>>,
}

#[derive(Clone, Copy)]
// Client 端 Listener Handler (处理 #[listen]/#[on])
pub struct ListenerRegistration {
    pub event: &'static str,
    pub func: fn(req: serde_json::Value) -> BoxFut<Result<(), anyhow::Error>>,
}

// Client 端 Exposed Handler (处理 #[export])
#[derive(Clone, Copy)]
pub struct ExposedHandlerRegistration {
    pub name: &'static str,
    pub func: fn(req: serde_json::Value) -> BoxFut<Result<serde_json::Value, anyhow::Error>>,
}

inventory::collect!(HandlerRegistration);
inventory::collect!(ListenerRegistration);
inventory::collect!(ExposedHandlerRegistration);

#[allow(dead_code)]
pub static HANDLE_ROUTES: LazyLock<
    HashMap<String, fn(req: serde_json::Value) -> BoxFut<Result<serde_json::Value, anyhow::Error>>>,
> = LazyLock::new(|| {
    let mut routes = HashMap::new();
    for HandlerRegistration { module, name, func } in
        inventory::iter::<HandlerRegistration>.into_iter()
    {
        #[cfg(test)]
        println!("add route: {}::{}", module, name);
        routes.insert(format!("{}::{}", module, name), func.clone());
    }
    routes
});

#[allow(dead_code)]
pub static EXPOSED_ROUTES: LazyLock<
    HashMap<String, fn(req: serde_json::Value) -> BoxFut<Result<serde_json::Value, anyhow::Error>>>,
> = LazyLock::new(|| {
    let mut routes = HashMap::new();
    for ExposedHandlerRegistration { name, func } in
        inventory::iter::<ExposedHandlerRegistration>.into_iter()
    {
        #[cfg(test)]
        println!("add exposed route: {}", name);
        routes.insert(name.to_string(), func.clone());
    }
    routes
});

#[allow(dead_code)]
pub static LISTENERS: LazyLock<
    HashMap<String, Vec<fn(req: serde_json::Value) -> BoxFut<Result<(), anyhow::Error>>>>,
> = LazyLock::new(|| {
    let mut routes: HashMap<
        String,
        Vec<fn(req: serde_json::Value) -> BoxFut<Result<(), anyhow::Error>>>,
    > = HashMap::new();
    for ListenerRegistration { event, func } in
        inventory::iter::<ListenerRegistration>().into_iter()
    {
        routes
            .entry(event.to_string())
            .and_modify(|e| e.push(func.clone()))
            .or_insert(vec![]);
    }
    routes
});
