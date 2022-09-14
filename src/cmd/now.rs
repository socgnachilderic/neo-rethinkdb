use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::Now)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::types::Time;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_now_time() -> Result<()> {
        let conn = r.connection().connect().await?;
        let time1 = r.now().value();
        let time2: Time = r.now().cmd().run(&conn).await?.unwrap().parse()?;
        
        assert!(time2 >= time1);

        Ok(())
    }
}
