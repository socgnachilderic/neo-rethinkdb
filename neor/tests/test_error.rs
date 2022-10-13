use neor::err::{ReqlError, ReqlRuntimeError};
use neor::{r, Result};

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
