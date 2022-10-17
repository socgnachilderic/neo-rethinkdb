use neor::arguments::{DistanceOption, Unit};
use neor::{args, r, Converter, Geometry, Result};

#[tokio::test]
async fn test_distance_data() -> Result<()> {
    let conn = r.connection().connect().await?;
    let point1 = r.point(-122.423246, 37.779388);
    let point2 = r.point(-117.220406, 32.719464);
    let distance_option = DistanceOption::default().unit(Unit::Kilometer);

    let response: f64 = r
        .distance(point1.cmd(), args!(point2, distance_option))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == 734.125249602186);

    Ok(())
}
