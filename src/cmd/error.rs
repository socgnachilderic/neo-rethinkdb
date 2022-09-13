use ql2::term::TermType;

use crate::Command;

pub(crate) fn new(message: &str) -> Command {
    let arg = Command::from_json(message);

    Command::new(TermType::Error).with_arg(arg)
}

#[cfg(test)]
mod tests {
    use crate::{r, ReqlError, ReqlRuntimeError, Result};

    #[tokio::test]
    async fn test_error_ops() -> Result<()> {
        let msg = "Error";
        let conn = r.connection().connect().await?;
        let err = r.error(msg).run(&conn).await.err().unwrap();

        if let ReqlError::Runtime(err) = err {
            if let ReqlRuntimeError::User(err) = err {
                assert!(err == msg);

                return Ok(());
            }
        }

        assert!(false);

        Ok(())
    }
}
