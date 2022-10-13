use neor::types::{MatchItem, MatchResponse};
use neor::{r, Converter, Result, StaticString};

#[tokio::test]
async fn test_match_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let data = MatchResponse {
        start: 0,
        end: 15,
        str: "name@domain.com".static_string(),
        groups: vec![MatchItem {
            start: 5,
            end: 15,
            str: "domain.com".static_string(),
        }],
    };
    let response: MatchResponse = r
        .expr("name@domain.com")
        .match_(".*@(.*)")
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == data);

    Ok(())
}

#[tokio::test]
async fn test_match_ops_return_none() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: Option<MatchResponse> = r
        .expr("name[at]domain.com")
        .match_(".*@(.*)")
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == None);

    Ok(())
}
