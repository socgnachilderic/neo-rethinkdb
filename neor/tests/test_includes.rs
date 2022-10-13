use neor::{args, r, Converter, Result};

#[tokio::test]
async fn test_includes_geo() -> Result<()> {
    let conn = r.connection().connect().await?;
    let point1 = r.point(-117.220406, 32.719464);
    let point2 = r.point(-117.206201, 32.725186);

    let response: bool = r
        .circle(args!(point1, 2000.))
        .includes(point2)
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response);

    Ok(())
}
