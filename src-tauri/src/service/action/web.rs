use std::{str::FromStr, time::Duration};

use serde_json::Value;
use tauri_plugin_http::reqwest::{ClientBuilder, Proxy};

use super::{error::ActionError, Action};

use ::common::ty::Data;
use aster_macro::action;

/// 声明返回分支
/// ```
///    ---------
/// -> + fetch +-- Success
///    ----+----
///        | timeout
/// ```

#[result]
enum Result {
    /// Success分支位于右侧
    #[right(zh_cn = "成功", en = "Success")]
    Success(plug!(Value as nesting! { a: nesting! { hello: String } })),
    // / Error分支位于底部
    #[bottom]
    // #[branch(error)]
    Error { timeout: nesting! { hello: plug! (String as number) } },
}

/// 声明下拉菜单
#[options]
pub enum Method {
    #[label(zh_cn = "获取", en = "Get")] // 可指定label
    Get,
    Post, // 不指定则使用默认label
    Delete,
    Put,
}

// 声明 action，并提供i18n支持
#[action(zh_cn = "网络请求", en = "Network request")]
// 多入口支持，用于可编程系统的设计
#[entry(url, method, option { proxy, http2, timeout })]
// action的描述
#[description(zh_cn = "发送一个网络请求", en = "Send a network request")]
async fn fetch_action(
    // 参数的名称与描述，用于构建表单
    #[name(zh_cn = "请求地址", en = "Request URL")]
    #[description(zh_cn = "请求的目标地址", en = "The target address of the request")]
    url: String,
    #[name(zh_cn = "请求方法", en = "Request Method")]
    #[description(
        zh_cn = "发送网络请求时使用的方法",
        en = "Method used when sending network requests"
    )]
    method: Method,
    #[name(zh_cn = "超时时间", en = "Timeout")]
    #[description(zh_cn = "请求超时时间", en = "Request timeout")]
    timeout: u64,
    #[name(zh_cn = "网络代理", en = "Network Proxy")]
    #[description(
        zh_cn = "请求时使用的代理服务器",
        en = "Proxy server to use for the request"
    )]
    proxy: Option<String>,
    #[name(zh_cn = "启用http2", en = "Enable http2")]
    #[description(
        zh_cn = "采用更高效安全的http/2",
        en = "Use the more efficient and secure http/2"
    )]
    http2: Option<bool>,
) -> Result {
    use tauri_plugin_http::reqwest::Method as HttpMethod;
    let method = method.to_string();
    let mut client_builder = ClientBuilder::new();
    client_builder = client_builder.timeout(Duration::from_millis(timeout));

    if let Some(proxy) = proxy {
        if !proxy.is_empty() {
            client_builder = client_builder.proxy(
                Proxy::http(proxy).map_err(|e| ActionError::RunActionCardError(e.to_string()))?,
            );
        }
    }

    if let Some(http2) = http2 {
        if !http2 {
            client_builder = client_builder.http1_only();
        }
    }
    let client = client_builder
        .build()
        .map_err(|e| ActionError::RunActionCardError(e.to_string()))?;
    let res = client
        .request(HttpMethod::from_str(&method)?, url)
        .send()
        .await?;
    let content_type = res
        .headers()
        .get(tauri_plugin_http::reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    // 根据 Content-Type 处理不同类型的响应
    let a = if content_type.contains("application/json") {
        // JSON 数据
        let json = res.json::<serde_json::Map<String, Value>>().await.unwrap();
        Data::Json(json)
    } else if content_type.contains("text/") {
        // 文本数据
        let text = res.text().await?;
        Data::String(text)
    } else {
        Data::Null
    };
    Result::Success(Value::Null)
}
