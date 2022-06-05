use std::borrow::Cow;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use regex::Regex;

use crate::Command;
use crate::ops::{SuperOps, ReqlOpsDocManipulation};

#[derive(Debug, Clone)]
pub struct MatchBuilder(pub(crate) Command);

impl MatchBuilder {
    pub(crate) fn new(regex: Regex) -> Self {
        let arg = Command::from_json(regex.as_str());
        let command = Command::new(TermType::Match).with_arg(arg);

        Self(command)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<Vec<Cow<'static, str>>>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(self, arg: impl super::run::Arg) -> impl Stream<Item = crate::Result<Vec<Cow<'static, str>>>> {        
        self.0.into_arg::<()>().into_cmd().run::<_, Vec<Cow<'static, str>>>(arg)
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl ReqlOpsDocManipulation for MatchBuilder { }

impl SuperOps for MatchBuilder {
    fn get_parent(&self) -> Command {
        self.0.clone()
    }
}
