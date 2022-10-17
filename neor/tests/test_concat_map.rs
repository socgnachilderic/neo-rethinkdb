use neor::{func, r, Converter, Result};

#[tokio::test]
async fn test_concat_map_data() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: Vec<u8> = r
        .expr([1, 2, 3])
        .concat_map(func!(|x| r.array([x.clone(), x * 2])))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == vec![1, 2, 2, 4, 3, 6]);

    Ok(())
}
