use serde::Serialize;

use crate::Command;

#[derive(Debug, Clone)]
pub struct AnyParam(Command);

impl AnyParam {
    pub fn new(arg: impl Serialize) -> Self {
        Self(Command::from_json(arg))
    }
}

impl From<AnyParam> for Command {
    fn from(param: AnyParam) -> Self {
        param.0
    }
}
