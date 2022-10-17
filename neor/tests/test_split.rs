use neor::{args, r, Converter, Result};

#[tokio::test]
async fn test_split_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let data = ["foo".to_owned(), "bar".to_owned(), "bax".to_owned()];
    let response: [String; 3] = r
        .expr("foo bar bax")
        .split(())
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == data);

    Ok(())
}

#[tokio::test]
async fn test_split_ops_entries() -> Result<()> {
    let conn = r.connection().connect().await?;
    let data = [
        "12".to_owned(),
        "37".to_owned(),
        String::new(),
        "22".to_owned(),
        String::new(),
    ];
    let response: [String; 5] = r
        .expr("12,37,,22,")
        .split(",")
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == data);

    Ok(())
}

#[tokio::test]
async fn test_split_ops_entries_limit() -> Result<()> {
    let conn = r.connection().connect().await?;
    let data = [
        "12".to_owned(),
        "37".to_owned(),
        String::new(),
        "22,".to_owned(),
    ];
    let response: [String; 4] = r
        .expr("12,37,,22,")
        .split(args!(",", 3))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == data);

    Ok(())
}
