use crate::types::Binary;

pub(crate) fn new(bytes: &[u8]) -> Binary {
    Binary::new(bytes)
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down};
    use crate::types::Binary;
    use crate::{r, Result};

    #[derive(Debug, Serialize, Deserialize)]
    struct User {
        id: u8,
        name: String,
        avatar: Binary,
    }

    #[tokio::test]
    async fn test_binary_ops() -> Result<()> {
        let avatar_img = std::fs::read("logo.png")?;
        let user = User {
            id: 1,
            name: "John Doe".to_string(),
            avatar: r.binary(&avatar_img),
        };

        let (conn, table, table_name) = set_up(false).await?;
        table.clone().insert(&user).run(&conn).await?;
        let response: User = table.get(1).run(&conn).await?.unwrap().parse()?;

        assert!(response.id == user.id);
        assert!(response.name == user.name);
        assert!(!response.avatar.data.is_empty());

        tear_down(conn, &table_name).await
    }
}
