use ql2::term::TermType;

use crate::{arguments::Args, prelude::Geometry, Command};

use super::CmdOpts;

pub(crate) fn new(geometry: impl IntersectsArg) -> Command {
    let (arg1, arg) = geometry.into_intersects_opts();
    let mut command = Command::new(TermType::Intersects);

    if let Some(arg) = arg1 {
        command = arg.add_to_cmd(command);
    }

    command.with_arg(arg)
}

pub trait IntersectsArg {
    fn into_intersects_opts(self) -> (Option<CmdOpts>, Command);
}

impl IntersectsArg for Command {
    fn into_intersects_opts(self) -> (Option<CmdOpts>, Command) {
        (None, self)
    }
}

impl<T> IntersectsArg for T
where
    T: Geometry,
{
    fn into_intersects_opts(self) -> (Option<CmdOpts>, Command) {
        (None, self.into())
    }
}

impl IntersectsArg for Args<(Command, Command)> {
    fn into_intersects_opts(self) -> (Option<CmdOpts>, Command) {
        (Some(CmdOpts::Single(self.0 .0)), self.0 .1)
    }
}

impl<T, R> IntersectsArg for Args<(T, R)>
where
    T: Geometry,
    R: Geometry,
{
    fn into_intersects_opts(self) -> (Option<CmdOpts>, Command) {
        (Some(CmdOpts::Single(self.0 .0.into())), self.0 .1.into())
    }
}

impl<T, R> IntersectsArg for Args<(Vec<T>, R)>
where
    T: Geometry,
    R: Geometry,
{
    fn into_intersects_opts(self) -> (Option<CmdOpts>, Command) {
        let seq = CmdOpts::Many(self.0 .0.into_iter().map(|geo| geo.get_command()).collect());

        (Some(seq), self.0 .1.into())
    }
}

impl IntersectsArg for Args<(Vec<Command>, Command)> {
    fn into_intersects_opts(self) -> (Option<CmdOpts>, Command) {
        let seq = CmdOpts::Many(self.0 .0);

        (Some(seq), self.0 .1)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{args, r, Result};

    #[tokio::test]
    async fn test_intersects_geo() -> Result<()> {
        let conn = r.connection().connect().await?;
        let point1 = r.point(-117.220406, 32.719464);
        let point2 = r.point(-117.206201, 32.725186);

        let response: bool = r
            .circle(args!(point1, 2000.))
            .intersects(point2)
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response);

        Ok(())
    }
}
