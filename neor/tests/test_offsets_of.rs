use neor::{r, Converter, Result};

#[tokio::test]
async fn test_offset_of_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: Vec<usize> = r
        .expr(['a', 'b', 'c'])
        .offsets_of('c')
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response.first() == Some(&2));

    Ok(())
}
