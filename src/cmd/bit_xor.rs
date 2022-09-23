use std::ops::BitXor;

use ql2::term::TermType;

use crate::Command;

impl<T: BitXorArg> BitXor<T> for Command {
    type Output = Self;

    fn bitxor(self, arg: T) -> Self {
        Command::new(TermType::BitXor)
            .with_arg(arg.into_bit_xor_opts())
            .with_parent(self)
    }
}

pub trait BitXorArg {
    fn into_bit_xor_opts(self) -> Command;
}

impl BitXorArg for i32 {
    fn into_bit_xor_opts(self) -> Command {
        Command::from_json(self)
    }
}

impl BitXorArg for Command {
    fn into_bit_xor_opts(self) -> Command {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_bit_xor_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let response: u8 = r.expr(6).bit_xor(4).run(&conn).await?.unwrap().parse()?;

        assert!(response == 2);

        Ok(())
    }

    #[tokio::test]
    async fn test_bit_xor_ops_with_command() -> Result<()> {
        let conn = r.connection().connect().await?;
        let response: u8 = r
            .bit_xor(r.expr(6), r.expr(4))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response == 2);

        Ok(())
    }

    #[tokio::test]
    async fn test_bit_xor_ops_with_syntax() -> Result<()> {
        let conn = r.connection().connect().await?;
        let response: u8 = (r.expr(6) ^ r.expr(4)).run(&conn).await?.unwrap().parse()?;

        assert!(response == 2);

        Ok(())
    }
}
