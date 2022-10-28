use neor::{args, r, Result};
use serde_json::json;

#[tokio::test]
async fn test_http_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response = r.http("http://httpbin.org/get").run(&conn).await?;

    assert!(response.is_some());

    Ok(())
}

#[tokio::test]
async fn test_http_ops_with_params() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response = r
        .http(args!(
            "http://httpbin.org/get",
            json!({
                "params": {
                    "user": 1
                }
            })
        ))
        .run(&conn)
        .await?;

    assert!(response.is_some());

    Ok(())
}
