use ql2::term::TermType;
use serde::Serialize;

use crate::{arguments::Args, Command};

pub(crate) fn new(args: impl SpliceAtArg) -> Command {
    let (arg_offset, arg_value) = args.into_splice_at_opts();

    Command::new(TermType::SpliceAt)
        .with_arg(arg_offset)
        .with_arg(arg_value)
}

pub trait SpliceAtArg {
    fn into_splice_at_opts(self) -> (Command, Command);
}

impl<S, T> SpliceAtArg for Args<(isize, T)>
where
    S: Serialize,
    T: IntoIterator<Item = S> + Serialize,
{
    fn into_splice_at_opts(self) -> (Command, Command) {
        (Command::from_json(self.0 .0), Command::from_json(self.0 .1))
    }
}

impl SpliceAtArg for Args<(Command, Command)> {
    fn into_splice_at_opts(self) -> (Command, Command) {
        (self.0 .0, self.0 .1)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{args, r, Result};

    #[tokio::test]
    async fn test_splice_at_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let response: [String; 4] = r
            .expr(["Moussa", "Ali"])
            .splice_at(args!(1, ["Fati", "Alima"]))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response == ["Moussa", "Fati", "Alima", "Ali"]);

        Ok(())
    }
}
