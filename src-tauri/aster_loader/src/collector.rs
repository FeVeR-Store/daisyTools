use common::action::ActionTrait;

/// Action 创建器接口
pub trait ActionCreator: Send + Sync {
    fn get_type(&self) -> &'static str;
    fn create(&self) -> Box<dyn ActionTrait>;
}

/// Action 创建器的静态信息
pub struct ActionCreatorInfo {
    pub action_type: &'static str,
    pub creator_fn: fn() -> Box<dyn ActionTrait>,
}

// 自动注册的 Action 创建器信息
inventory::collect!(ActionCreatorInfo);
