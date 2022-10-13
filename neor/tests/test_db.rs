use neor::err::{ReqlError, ReqlRuntimeError};
use neor::{r, Result};

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
