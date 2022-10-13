use ql2::term::TermType;

use crate::Command;

pub(crate) fn new(db_name: impl Into<String>) -> Command {
    let arg = Command::from_json(db_name.into());

    Command::new(TermType::Db).with_arg(arg)
}

#[cfg(test)]
mod tests {
    use crate::{r, ReqlError, ReqlRuntimeError, Result};

    #[tokio::test]
    async fn test_select_db() -> Result<()> {
        let conn = r.connection().connect().await?;
        let response = r.db("test").run(&conn).await.err().unwrap();

        if let ReqlError::Runtime(err) = response {
            if let ReqlRuntimeError::QueryLogic(msg) = err {
                assert!(true, "{}", msg);
                return Ok(());
            }
        }

        assert!(false);
        Ok(())
    }
}
