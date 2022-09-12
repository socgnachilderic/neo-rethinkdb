use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::TypeOf)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::types::TypeOf;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_type_of_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let array: TypeOf = r
            .expr([1, 2, 3])
            .type_of()
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;
        let boolean: TypeOf = r.expr(true).type_of().run(&conn).await?.unwrap().parse()?;
        let db: TypeOf = r.db("test").type_of().run(&conn).await?.unwrap().parse()?;
        let string: TypeOf = r.expr("foo").type_of().run(&conn).await?.unwrap().parse()?;

        assert!(array == TypeOf::Array);
        assert!(boolean == TypeOf::Bool);
        assert!(db == TypeOf::Db);
        assert!(string == TypeOf::String);

        Ok(())
    }
}
