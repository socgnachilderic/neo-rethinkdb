use ql2::term::TermType;

use crate::prelude::Geometry;
use crate::Command;

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
