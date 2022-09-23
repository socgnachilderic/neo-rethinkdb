use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::arguments::Args;
use crate::Command;

pub(crate) fn new(args: impl JsArg) -> Command {
    let (arg, opts) = args.into_js_opts();

    Command::new(TermType::Javascript)
        .with_arg(arg)
        .with_opts(opts)
}

pub trait JsArg {
    fn into_js_opts(self) -> (Command, JsOption);
}

impl<T> JsArg for T
where
    T: Into<String>,
{
    fn into_js_opts(self) -> (Command, JsOption) {
        (Command::from_json(self.into()), Default::default())
    }
}

impl<T> JsArg for Args<(T, JsOption)>
where
    T: Into<String>,
{
    fn into_js_opts(self) -> (Command, JsOption) {
        (Command::from_json(self.0 .0.into()), self.0 .1)
    }
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct JsOption {
    /// `timeout` is the number of seconds before r.js times out.
    /// The default value is 5 seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<f64>,
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_js_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let response: String = r.js("'str1' + 'str2'").run(&conn).await?.unwrap().parse()?;

        assert!(response.eq("str1str2"));

        Ok(())
    }
}
