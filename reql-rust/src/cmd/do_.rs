use crate::prelude::Func;
use crate::Command;
use futures::Stream;
use ql2::term::TermType;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct DoBuilder(Command, Option<Command>);

impl DoBuilder {
    pub fn new(func: Func) -> Self {
        let Func(func) = func;
        let command = Command::new(TermType::Funcall).with_arg(func);

        Self(command, None)
    }

    pub fn run<A, T>(self, arg: A) -> impl Stream<Item = crate::Result<T>>
    where
        A: super::run::Arg,
        T: Unpin + DeserializeOwned,
    {
        let mut cmd = self.0;

        if let Some(parent) = self.1 {
            cmd = cmd.with_parent(parent);
        }

        let cmd = cmd.into_arg::<()>().into_cmd();

        cmd.run::<_, T>(arg)
    }

    pub fn with_args<T: Serialize>(mut self, args: &[T]) -> Self {
        for arg in args {
            let arg = Command::from_json(arg);
            self.0 = self.0.with_arg(arg);
        }

        self
    }

    pub fn _with_parent(mut self, parent: Command) -> Self {
        self.1 = Some(parent);
        self
    }
}

impl Into<Command> for DoBuilder {
    fn into(self) -> Command {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::{self as reql_rust, cmd, r};

    #[test]
    fn r_do() {
        let counter = crate::current_counter();
        let query = r.do_(func!(|x, y| x + y)).with_args(&[10, 20]);
        let serialised = cmd::serialise(&query.into());
        let expected = format!(
            r#"[64,[[69,[[2,[2,3]],[24,[[10,[{}]],[10,[{}]]]]]],10,20]]"#,
            counter,
            counter + 1
        );
        assert_eq!(serialised, expected);
    }

    #[test]
    fn r_db_table_get_do() {
        let counter = crate::current_counter();
        let query = r.db("mydb").table("table1").get("johndoe@example.com");
        // .do_(func!(|doc| r
        //     .db("mydb")
        //     .table("table2")
        //     .get(doc.get_field("id"))));
        let serialised = cmd::serialise(&query.into());
        let expected = format!(
            r#"[64,[[69,[[2,[1]],[16,[[15,[[14,["mydb"]],"table2"]],[31,[[10,[{}]],"id"]]]]]],[16,[[15,[[14,["mydb"]],"table1"]],"johndoe@example.com"]]]]"#,
            counter
        );
        assert_eq!(serialised, expected);
        todo!();
    }
}
