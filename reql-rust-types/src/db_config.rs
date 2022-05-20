use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct DbConfigChange {
    pub new_val: Option<DbConfigChangeValue>,
    pub old_val: Option<DbConfigChangeValue>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct DbConfigChangeValue {
    pub id: Cow<'static, str>,
    pub name: Cow<'static, str>,
}
