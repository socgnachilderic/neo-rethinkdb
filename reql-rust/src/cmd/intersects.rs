use ql2::term::TermType;

use crate::{prelude::Geometry, Command};

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

impl<T: Geometry> IntersectsArg for T {
    fn into_intersects_opts(self) -> (Option<CmdOpts>, Command) {
        (None, self.into())
    }
}

impl<T: Geometry, R: Geometry> IntersectsArg for (T, R) {
    fn into_intersects_opts(self) -> (Option<CmdOpts>, Command) {
        (Some(CmdOpts::Single(self.0.into())), self.1.into())
    }
}

impl<T: Geometry, R: Geometry> IntersectsArg for (Vec<T>, R) {
    fn into_intersects_opts(self) -> (Option<CmdOpts>, Command) {
        let seq = CmdOpts::Many(self.0.into_iter().map(|geo| geo.get_command()).collect());

        (Some(seq), self.1.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::{prelude::Converter, r, Result};

    #[tokio::test]
    async fn test_intersects_geo() -> Result<()> {
        let conn = r.connection().connect().await?;
        let point1 = r.point(-117.220406, 32.719464);
        let point2 = r.point(-117.206201, 32.725186);

        let response: bool = r
            .circle((point1, 2000.))
            .intersects(point2)
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response);

        Ok(())
    }
}
