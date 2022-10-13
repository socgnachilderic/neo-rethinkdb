use neor::{r, Converter, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct InnerPost {
    id: String,
    title: String,
}

#[tokio::test]
async fn test_object_converted() -> Result<()> {
    let post = InnerPost {
        id: "id1".to_string(),
        title: "title1".to_string(),
    };
    let object = vec!["id", "id1", "title", "title1"];

    let conn = r.connection().connect().await?;
    let data_obtained: InnerPost = r.object(object).run(&conn).await?.unwrap().parse()?;

    assert!(data_obtained == post);

    Ok(())
}
