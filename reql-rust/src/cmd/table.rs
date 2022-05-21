use super::args::Args;
// use super::run;
use crate::{cmd, Command};
use crate::types::{ReadMode, IdentifierFormat};
use ql2::term::TermType;
use reql_rust_macros::CommandOptions;
use serde::Serialize;

pub struct TableBuilder(Command, TableOption, Option<Command>);

#[derive(Debug, Clone, Copy, CommandOptions, Serialize, Default, PartialEq, PartialOrd)]
#[non_exhaustive]
pub struct TableOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_mode: Option<ReadMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier_format: Option<IdentifierFormat>,
}

/* impl TableBuilder {
    pub fn new(table_name: &str) -> Self {
        let args = Command::from_json(table_name);
        let command = Command::new(TermType::Table).with_arg(args);

        Self(command, TableOption::default(), None)
    }

    pub fn run(self, arg: impl run::Arg) -> impl Stream<Item = crate::Result<TableCreateReturnType>> {
        let mut cmd = self.0.with_opts(self.1);

        if let Some(parent) = self.2 {
            cmd = cmd.with_parent(parent);
        }
            
        let cmd = cmd.into_arg::<()>()
            .into_cmd();

        cmd.run::<_, TableCreateReturnType>(arg)
    }
} */

pub trait Arg {
    fn arg(self) -> cmd::Arg<TableOption>;
}

impl Arg for Command {
    fn arg(self) -> cmd::Arg<TableOption> {
        Self::new(TermType::Table).with_arg(self).into_arg()
    }
}

impl<T> Arg for T
where
    T: Into<String>,
{
    fn arg(self) -> cmd::Arg<TableOption> {
        Command::from_json(self.into()).arg()
    }
}

impl Arg for Args<(Command, TableOption)> {
    fn arg(self) -> cmd::Arg<TableOption> {
        let Args((query, options)) = self;
        query.arg().with_opts(options)
    }
}

impl<T> Arg for Args<(T, TableOption)>
where
    T: Into<String>,
{
    fn arg(self) -> cmd::Arg<TableOption> {
        let Args((name, options)) = self;
        name.arg().with_opts(options)
    }
}

#[cfg(test)]
mod tests {
    use crate::{cmd, r};

    #[test]
    fn r_table() {
        let query = r.table("foo");
        let serialised = cmd::serialise(&query);
        let expected = r#"[15,["foo"]]"#;
        assert_eq!(serialised, expected);
    }

    #[test]
    fn r_db_table() {
        let query = r.db("foo").table("bar");
        let serialised = cmd::serialise(&query);
        let expected = r#"[15,[[14,["foo"]],"bar"]]"#;
        assert_eq!(serialised, expected);
    }
}
