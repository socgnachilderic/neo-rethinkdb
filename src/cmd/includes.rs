use ql2::term::TermType;

use crate::{prelude::Geometry, Command};

use super::CmdOpts;

pub(crate) fn new(geometry: impl IncludesArg) -> Command {
    geometry
        .into_includes_opts()
        .add_to_cmd(Command::new(TermType::Includes))
}

pub trait IncludesArg {
    fn into_includes_opts(self) -> CmdOpts;
}

impl<T: Geometry> IncludesArg for T {
    fn into_includes_opts(self) -> CmdOpts {
        CmdOpts::Single(self.into())
    }
}

impl IncludesArg for Command {
    fn into_includes_opts(self) -> CmdOpts {
        CmdOpts::Single(self)
    }
}

impl<T: Geometry> IncludesArg for Vec<T> {
    fn into_includes_opts(self) -> CmdOpts {
        CmdOpts::Many(self.into_iter().map(|geo| geo.get_command()).collect())
    }
}

impl IncludesArg for Vec<Command> {
    fn into_includes_opts(self) -> CmdOpts {
        CmdOpts::Many(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{args, r, Result};

    #[tokio::test]
    async fn test_includes_geo() -> Result<()> {
        let conn = r.connection().connect().await?;
        let point1 = r.point(-117.220406, 32.719464);
        let point2 = r.point(-117.206201, 32.725186);

        let response: bool = r
            .circle(args!(point1, 2000.))
            .includes(point2)
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response);

        Ok(())
    }
}
