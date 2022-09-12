use ql2::term::TermType;
use reql_rust_macros::CommandOptions;
use serde::{Deserialize, Serialize};

use crate::Command;

pub(crate) fn new(args: impl HttpArg) -> Command {
    let (arg, opts) = args.into_http_opts();

    Command::new(TermType::Http).with_arg(arg).with_opts(opts)
}

pub trait HttpArg {
    fn into_http_opts(self) -> (Command, HttpOption);
}

impl HttpArg for &str {
    fn into_http_opts(self) -> (Command, HttpOption) {
        (Command::from_json(self), Default::default())
    }
}

impl HttpArg for (&str, HttpOption) {
    fn into_http_opts(self) -> (Command, HttpOption) {
        (Command::from_json(self.0), self.1)
    }
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct HttpOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirects: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_format: Option<HttpResultFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<HttpMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<HttpAuth>,
    // pub params: Option<HttpAuth>,
    // pub header: Option<HttpAuth>,
    // pub data: Option<HttpAuth>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
pub enum HttpResultFormat {
    Text,
    Json,
    Jsonp,
    Binary,
    Auto,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HttpAuth {
    typ: String,
    user: String,
    pass: String,
}

// TODO finish this command and write test
