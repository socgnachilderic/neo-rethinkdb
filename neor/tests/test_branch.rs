use neor::{args, r, Converter, Result};

#[tokio::test]
async fn test_branch_data() -> Result<()> {
    let x = 10;
    let conn = r.connection().connect().await?;
    let response: String = r
        .branch(x > 5, args!("big", "small"))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response.eq("big"));

    Ok(())
}

#[tokio::test]
async fn test_branch_data_with_infix() -> Result<()> {
    let x = 10;
    let conn = r.connection().connect().await?;
    let response: String = r
        .expr(x > 5)
        .branch(args!("big", "small"))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response.eq("big"));

    Ok(())
}
