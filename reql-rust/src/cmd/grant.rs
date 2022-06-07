use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::Serialize;

use crate::Command;
use crate::ops::SuperOps;
use crate::types::GrantResponseType;

#[derive(Debug, Clone)]
pub struct GrantBuilder(pub(crate) Command, GrantOption);

#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd)]
#[non_exhaustive]
pub(crate) struct GrantOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<bool>,
}

impl GrantBuilder {
    pub(crate) fn new(username: &str) -> Self {
        let arg = Command::from_json(username);
        let command = Command::new(TermType::Grant).with_arg(arg);
        Self(command, GrantOption::default())
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<GrantResponseType>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(self, arg: impl super::run::Arg) -> impl Stream<Item = crate::Result<GrantResponseType>> {  
        let permissions = Command::from_json(self.1);      
        self.0.with_arg(permissions)
            .into_arg::<()>()
            .into_cmd()
            .run::<_, GrantResponseType>(arg)
    }

    pub fn permit_read(mut self, read: bool) -> Self {
        self.1.read = Some(read);
        self
    }

    pub fn permit_write(mut self, write: bool) -> Self {
        self.1.write = Some(write);
        self
    }

    pub fn permit_connect(mut self, connect: bool) -> Self {
        self.1.connect = Some(connect);
        self
    }

    pub fn permit_config(mut self, config: bool) -> Self {
        self.1.config = Some(config);
        self
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl SuperOps for GrantBuilder {
    fn get_parent(&self) -> Command {
        self.0.clone()
    }
}
