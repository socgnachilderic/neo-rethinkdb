use ql2::term::TermType;
use reql_rust_macros::CommandOptions;
use serde::Serialize;

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

impl JsArg for &str {
    fn into_js_opts(self) -> (Command, JsOption) {
        (Command::from_json(self), Default::default())
    }
}

impl JsArg for (&str, JsOption) {
    fn into_js_opts(self) -> (Command, JsOption) {
        (Command::from_json(self.0), self.1)
    }
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct JsOption {
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
