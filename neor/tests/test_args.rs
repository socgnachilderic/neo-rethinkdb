use neor::{r, Converter, Result};

#[tokio::test]
async fn test_args_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let data = vec![1, 2, 3];
    let response: Vec<u8> = r.args(&data).run(&conn).await?.unwrap().parse()?;

    assert!(response == data);

    Ok(())
}
