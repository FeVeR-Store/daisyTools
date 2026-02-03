use aster_macro::action;
use serde_json::Map;

#[result]
enum Result {
    #[right]
    Success(Map<String, Value>),
    #[bottom]
    ABC { nest: nesting! {} },
}

#[action(zh_cn = "注入context")]
fn inject_context_action(
    #[name(zh_cn = "context键")] key: String,
    #[name(zh_cn = "context值")] value: String,
) -> Result {
    let mut map = Map::new();
    map.insert(key, Value::String(value));
    Result::Success(map)
}
