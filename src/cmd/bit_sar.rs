use ql2::term::TermType;

use crate::Command;

pub(crate) fn new(args: impl BitSarArg) -> Command {
    Command::new(TermType::BitSar).with_arg(args.into_bit_sar_opts())
}

pub trait BitSarArg {
    fn into_bit_sar_opts(self) -> Command;
}

impl BitSarArg for f64 {
    fn into_bit_sar_opts(self) -> Command {
        Command::from_json(self)
    }
}

impl BitSarArg for Command {
    fn into_bit_sar_opts(self) -> Command {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_bit_sar_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let response: u8 = r.expr(32).bit_sar(3.).run(&conn).await?.unwrap().parse()?;

        assert!(response == 4);

        Ok(())
    }

    #[tokio::test]
    async fn test_bit_sar_ops_with_c() -> Result<()> {
        let conn = r.connection().connect().await?;
        let response: u8 = r
            .bit_sar(r.expr(32), r.expr(3))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response == 4);

        Ok(())
    }
}
