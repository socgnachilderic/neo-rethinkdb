use std::ops::BitAnd;

use ql2::term::TermType;

use crate::Command;

impl<T: BitAndArg> BitAnd<T> for Command {
    type Output = Self;

    fn bitand(self, arg: T) -> Self {
        Command::new(TermType::BitAnd)
            .with_arg(arg.into_bit_and_opts())
            .with_parent(self)
    }
}

pub trait BitAndArg {
    fn into_bit_and_opts(self) -> Command;
}

impl BitAndArg for i32 {
    fn into_bit_and_opts(self) -> Command {
        Command::from_json(self)
    }
}

impl BitAndArg for Command {
    fn into_bit_and_opts(self) -> Command {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_bit_and_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let response: u8 = r.expr(5).bit_and(3).run(&conn).await?.unwrap().parse()?;

        assert!(response == 1);

        Ok(())
    }

    #[tokio::test]
    async fn test_bit_and_ops_with_command() -> Result<()> {
        let conn = r.connection().connect().await?;
        let response: u8 = r
            .bit_and(r.expr(5), r.expr(3))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response == 1);

        Ok(())
    }

    #[tokio::test]
    async fn test_bit_and_ops_with_syntax() -> Result<()> {
        let conn = r.connection().connect().await?;
        let response: u8 = (r.expr(5) & r.expr(3)).run(&conn).await?.unwrap().parse()?;

        assert!(response == 1);

        Ok(())
    }
}
