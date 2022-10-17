use neor::{r, Converter, Result};

#[tokio::test]
async fn test_js_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: String = r.js("'str1' + 'str2'").run(&conn).await?.unwrap().parse()?;

    assert!(response.eq("str1str2"));

    Ok(())
}
