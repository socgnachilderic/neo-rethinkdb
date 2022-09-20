use std::ops::BitOr;

use ql2::term::TermType;

use crate::Command;

impl<T: BitOrArg> BitOr<T> for Command {
    type Output = Self;

    fn bitor(self, arg: T) -> Self {
        Command::new(TermType::BitOr)
            .with_arg(arg.into_bit_or_opts())
            .with_parent(self)
    }
}

pub trait BitOrArg {
    fn into_bit_or_opts(self) -> Command;
}

impl BitOrArg for i32 {
    fn into_bit_or_opts(self) -> Command {
        Command::from_json(self)
    }
}

impl BitOrArg for Command {
    fn into_bit_or_opts(self) -> Command {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_bit_or_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let response: u8 = r.expr(5).bit_or(3).run(&conn).await?.unwrap().parse()?;

        assert!(response == 7);

        Ok(())
    }

    #[tokio::test]
    async fn test_bit_or_ops_with_command() -> Result<()> {
        let conn = r.connection().connect().await?;
        let response: u8 = r
            .bit_or(r.expr(5), r.expr(3))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response == 7);

        Ok(())
    }

    #[tokio::test]
    async fn test_bit_or_ops_with_syntax() -> Result<()> {
        let conn = r.connection().connect().await?;
        let response: u8 = (r.expr(7) | r.expr(3)).run(&conn).await?.unwrap().parse()?;

        assert!(response == 7);

        Ok(())
    }
}
