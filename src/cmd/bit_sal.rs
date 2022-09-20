use ql2::term::TermType;

use crate::Command;

pub(crate) fn new(args: impl BitSalArg) -> Command {
    Command::new(TermType::BitSal).with_arg(args.into_bit_sal_opts())
}

pub trait BitSalArg {
    fn into_bit_sal_opts(self) -> Command;
}

impl BitSalArg for i32 {
    fn into_bit_sal_opts(self) -> Command {
        Command::from_json(self)
    }
}

impl BitSalArg for Command {
    fn into_bit_sal_opts(self) -> Command {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_bit_sal_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let response: u8 = r.expr(5).bit_sal(4).run(&conn).await?.unwrap().parse()?;

        assert!(response == 80);

        Ok(())
    }

    #[tokio::test]
    async fn test_bit_sal_ops_with_c() -> Result<()> {
        let conn = r.connection().connect().await?;
        let response: u8 = r
            .bit_sal(r.expr(5), r.expr(4))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response == 80);

        Ok(())
    }
}
