use serde::Serialize;

use crate::Command;

pub(crate) fn new(value: impl Serialize) -> Command {
    Command::from_json(value)
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use crate::arguments::AnyParam;
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct Dummy {
        a: char,
        b: [u8; 3],
    }

    #[tokio::test]
    async fn test_expr_ops() -> Result<()> {
        let data = Dummy {
            a: 'b',
            b: [1, 2, 3],
        };
        let conn = r.connection().connect().await?;
        let response: Dummy = r
            .expr(json!({'a':'b'}))
            .merge(AnyParam::new(json!({'b':[1, 2, 3]})))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data == response);

        Ok(())
    }
}
