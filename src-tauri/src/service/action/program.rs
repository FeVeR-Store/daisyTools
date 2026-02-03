use crate::runtime::{javascript::JavaScriptRuntime, Runtime};

use super::{error::ActionError, Action, ActionTrait};
use aster_macro::action;

#[result]
enum Result {
    #[right]
    Success,
}

#[options]
pub enum Lang {
    JavaScript,
}

#[action(zh_cn = "执行代码", en = "Execute Code")]
#[description(zh_cn = "执行一段代码", en = "Execute a code segment")]
pub fn program_action(
    #[name(zh_cn = "代码内容", en = "Code content")]
    #[description(zh_cn = "要执行的代码", en = "Code to execute")]
    code: Code,
    #[name(zh_cn = "语言", en = "Language")]
    #[description(zh_cn = "使用的编程语言", en = "Programming language used")]
    lang: Lang,
) -> Result {
    let runtime = JavaScriptRuntime::new();
    runtime
        .execute(code)
        .map_err(|e| ActionError::RunActionCardError(e.to_string()))?;
    Result::Success
}
