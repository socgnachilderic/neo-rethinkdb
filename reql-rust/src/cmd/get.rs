use crate::Command;
use futures::Stream;
use ql2::term::TermType;
use serde::de::DeserializeOwned;

use super::run;

pub struct GetBuilder(Command);

impl GetBuilder {
    pub fn new(primary_key: &str) -> Self {
        let args = Command::from_json(primary_key);
        let command = Command::new(TermType::Get).with_arg(args);

        Self(command)
    }

    pub fn run<A, T>(self, arg: A) -> impl Stream<Item = crate::Result<T>>
    where
        A: run::Arg,
        T: Unpin + DeserializeOwned, 
    {            
        let cmd = self.0.into_arg::<()>()
            .into_cmd();

        cmd.run::<_, T>(arg)
    }

    pub fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl Into<Command> for GetBuilder {
    fn into(self) -> Command {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::{cmd, r};

    #[test]
    fn r_db_table_get() {
        let query = r
            .db("foo")
            .table("bar")
            .get("84fc23ac-9e85-43af-b6f7-f86be17237e1");
        let serialised = cmd::serialise(&query.into());
        let expected = r#"[16,[[15,[[14,["foo"]],"bar"]],"84fc23ac-9e85-43af-b6f7-f86be17237e1"]]"#;
        assert_eq!(serialised, expected);
    }
}
