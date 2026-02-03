use serde::{Deserialize, Serialize};

use crate::ty::Data;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ActionEntry {
    LitRef {
        id: String,
        wid: String,
    },
    Inline {
        uid: String,
        r#type: String,
        data: Data,
    },
}