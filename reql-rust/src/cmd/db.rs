use crate::Command;
use ql2::term::TermType;

pub fn make_db_command(db_name: &str) -> Command {
    let args = Command::from_json(db_name);

    Command::new(TermType::Db)
        .with_arg(args)
        .into_arg::<()>()
        .into_cmd()
}


/* #[derive(Debug, Clone)]
pub struct DbBuilder(pub(crate) Command);

impl DbBuilder {
    pub(crate) fn new(db_name: &str) -> Self {
        let args = Command::from_json(db_name);

        Self(
            Command::new(TermType::Db)
                .with_arg(args)
                .into_arg::<()>()
                .into_cmd(),
        )
    }

    pub fn table_create(self, table_name: &str) -> super::table_create::TableCreateBuilder {
        super::table_create::TableCreateBuilder::new(table_name)._with_parent(self.get_parent())
    }

    pub fn table_drop(self, table_name: &str) -> super::table_drop::TableDropBuilder {
        super::table_drop::TableDropBuilder::new(table_name)._with_parent(self.get_parent())
    }

    pub fn table_list(self) -> super::table_list::TableListBuilder {
        super::table_list::TableListBuilder::new()._with_parent(self.get_parent())
    }

    pub fn table<T>(self, table_name: &str) -> super::table::TableBuilder<T>
    where
        T: Unpin + Serialize + DeserializeOwned,
    {
        super::table::TableBuilder::new(table_name)._with_parent(self.0)
    }

    pub fn grant(self, username: &str) -> super::grant::GrantBuilder {
        super::grant::GrantBuilder::new(username)._with_parent(self.get_parent())
    }

    pub fn config(self) -> super::config::ConfigBuilder {
        super::config::ConfigBuilder::new()._with_parent(self.get_parent())
    }

    pub fn rebalance(self) -> super::rebalance::RebalanceBuilder {
        super::rebalance::RebalanceBuilder::new()._with_parent(self.get_parent())
    }

    pub fn reconfigure(self) -> super::reconfigure::ReconfigureBuilder {
        super::reconfigure::ReconfigureBuilder::new()._with_parent(self.get_parent())
    }

    pub fn wait(self) -> super::wait::WaitBuilder {
        super::wait::WaitBuilder::new()._with_parent(self.get_parent())
    }
} */

#[cfg(test)]
mod tests {
    use crate::{r, ReqlError, ReqlRuntimeError, Result};

    #[tokio::test]
    async fn test_select_db() -> Result<()> {
        let conn = r.connection().connect().await?;
        let response = r.db("test").run(&conn).await.err().unwrap();

        if let ReqlError::Runtime(err) = response {
            if let ReqlRuntimeError::QueryLogic(msg) = err {
                assert!(true, "{}", msg);
                return Ok(());
            }
        }

        assert!(false);
        Ok(())
    }
}
