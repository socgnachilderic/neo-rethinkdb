use ql2::term::TermType;
use regex::Regex;

use crate::Command;

pub(crate) fn new(regex: Regex) -> Command {
    let arg = Command::from_json(regex.as_str());

    Command::new(TermType::Match).with_arg(arg)
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use crate::prelude::*;
    use crate::types::{MatchItem, MatchResponse};
    use crate::{r, Result};

    #[tokio::test]
    async fn test_match_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let regexp = Regex::new(".*@(.*)")?;
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
            .match_(regexp)
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
        let regexp = Regex::new(".*@(.*)")?;
        let response: Option<MatchResponse> = r
            .expr("name[at]domain.com")
            .match_(regexp)
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response == None);

        Ok(())
    }
}
