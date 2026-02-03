use crate::action::{form::FormData, result::ResultBranchType};

#[derive(Debug)]
pub struct FormDataCollect {
    pub name: String,
    pub data: FormData,
    pub file: String,
}

#[derive(Debug, Default)]
pub struct ResultBranchTypeCollect {
    pub name: String,
    pub data: Vec<ResultBranchType>,
    pub file: String,
}
