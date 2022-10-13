use neor::{r, Converter, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Dummy {
    a: char,
    b: [u8; 3],
}

#[tokio::test]
async fn test_expr_ops() -> Result<()> {
    let data = Dummy {
        a: 'b',
        b: [1, 2, 3],
    };
    let conn = r.connection().connect().await?;
    let response: Dummy = r
        .expr(json!({'a':'b'}))
        .merge(r.expr(json!({'b':[1, 2, 3]})))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(data == response);

    Ok(())
}
