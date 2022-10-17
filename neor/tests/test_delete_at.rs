use neor::{args, r, Converter, Result};

const DATA: [char; 6] = ['a', 'b', 'c', 'd', 'e', 'f'];

#[tokio::test]
async fn test_delete_at_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: [char; 5] = r
        .expr(&DATA)
        .delete_at(1)
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    let response2: [char; 5] = r
        .expr(&DATA)
        .delete_at(-2)
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    let response3: [char; 4] = r
        .expr(&DATA)
        .delete_at(args!(1, 3))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == ['a', 'c', 'd', 'e', 'f']);
    assert!(response2 == ['a', 'b', 'c', 'd', 'f']);
    assert!(response3 == ['a', 'd', 'e', 'f']);

    Ok(())
}
