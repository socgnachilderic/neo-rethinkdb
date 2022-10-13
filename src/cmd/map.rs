use ql2::term::TermType;

use crate::arguments::Args;
use crate::prelude::Func;
use crate::Command;

use super::CmdOpts;

pub(crate) fn new(args: impl MapArg) -> Command {
    let (args, func) = args.into_map_opts();
    let mut command = Command::new(TermType::Map);

    if let Some(args) = args {
        command = args.add_to_cmd(command);
    }

    command.with_arg(func)
}

pub trait MapArg {
    fn into_map_opts(self) -> (Option<CmdOpts>, Command);
}

impl MapArg for Func {
    fn into_map_opts(self) -> (Option<CmdOpts>, Command) {
        (None, self.0)
    }
}

impl MapArg for Args<(Command, Func)> {
    fn into_map_opts(self) -> (Option<CmdOpts>, Command) {
        let Func(func) = self.0 .1;

        (Some(CmdOpts::Single(self.0 .0)), func)
    }
}

impl<T> MapArg for Args<(T, Func)>
where
    T: AsRef<[Command]>,
{
    fn into_map_opts(self) -> (Option<CmdOpts>, Command) {
        let Func(func) = self.0 .1;

        (Some(CmdOpts::Many(self.0 .0.as_ref().to_vec())), func)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_map_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let data_obtained: Vec<u8> = r
            .expr([1, 2, 3, 4, 5])
            .map(func!(|val| val.clone() * val))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_obtained == vec![1, 4, 9, 16, 25]);

        Ok(())
    }
}
