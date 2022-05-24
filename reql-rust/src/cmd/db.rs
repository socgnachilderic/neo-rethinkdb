use crate::Command;
use ql2::term::TermType;

use super::{table::TableBuilder, ReqlDbTableManipulatingOps};

pub struct DbBuilder(Command);

impl DbBuilder {
    pub fn new(db_name: &str) -> Self {
        let args = Command::from_json(db_name);

        Self(
            Command::new(TermType::Db)
                .with_arg(args)
                .into_arg::<()>()
                .into_cmd()
        )
    }

    pub fn table(self, table_name: &str) -> TableBuilder {
        TableBuilder::new(table_name)._with_parent(self.0)
    }
}

impl ReqlDbTableManipulatingOps for DbBuilder { }

impl Into<Command> for DbBuilder {
    fn into(self) -> Command {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::{cmd, r};

    #[test]
    fn r_db() {
        let query = r.db("foo").into();
        let serialised = cmd::serialise(&query);
        let expected = r#"[14,["foo"]]"#;
        assert_eq!(serialised, expected);
    }
}
