use std::collections::{HashMap, VecDeque};
use std::{fmt, str};

use ql2::query::QueryType;
use ql2::term::TermType;
use serde::ser::{self, Serialize, Serializer};
use serde_json::value::{Number, Value};

use crate::cmd::run::{Db, Options};
// use crate::prelude::ReqlOps;
use crate::{err, r};

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum Datum {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Datum>),
    Object(HashMap<String, Datum>),
}

impl Default for Datum {
    fn default() -> Self {
        Self::Null
    }
}

impl Serialize for Datum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Null => serializer.serialize_none(),
            Self::Bool(boolean) => boolean.serialize(serializer),
            Self::Number(num) => num.serialize(serializer),
            Self::String(string) => string.serialize(serializer),
            Self::Array(arr) => (TermType::MakeArray as i32, arr).serialize(serializer),
            Self::Object(map) => map.serialize(serializer),
        }
    }
}

impl From<Value> for Datum {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => Self::Null,
            Value::Bool(boolean) => Self::Bool(boolean),
            Value::Number(num) => Self::Number(num),
            Value::String(string) => Self::String(string),
            Value::Array(arr) => Self::Array(arr.into_iter().map(Into::into).collect()),
            Value::Object(map) => Self::Object(
                map.into_iter()
                    .map(|(key, value)| (key, value.into()))
                    .collect(),
            ),
        }
    }
}

/// The query that will be sent to RethinkDB
#[derive(Debug, Clone)]
pub struct Command {
    typ: TermType,
    datum: Option<super::Result<Datum>>,
    #[doc(hidden)]
    pub args: VecDeque<super::Result<Command>>,
    opts: Option<super::Result<Datum>>,
    change_feed: bool,
}

impl Command {
    #[doc(hidden)]
    pub fn new(typ: TermType) -> Self {
        Self {
            typ,
            datum: None,
            args: VecDeque::new(),
            opts: None,
            change_feed: false,
        }
    }

    #[doc(hidden)]
    pub fn var(id: u64) -> Self {
        let index = Self::from_json(id);
        Self::new(TermType::Var).with_arg(index)
    }

    pub(crate) fn with_parent(mut self, parent: Command) -> Self {
        self.change_feed = self.change_feed || parent.change_feed;
        self.args.push_front(Ok(parent));
        self
    }

    #[doc(hidden)]
    pub fn with_arg(mut self, arg: impl Into<Command>) -> Self {
        let arg = arg.into();
        self.args.push_back(Ok(arg));
        self
    }

    pub(crate) fn with_opts<T>(mut self, opts: T) -> Self
    where
        T: Serialize,
    {
        let opts = serde_json::to_value(&opts)
            .map(Into::into)
            .map_err(Into::into);
        self.opts = Some(opts);
        self
    }

    #[doc(hidden)]
    pub fn from_json<T>(arg: T) -> Self
    where
        T: Serialize,
    {
        serde_json::to_value(arg)
            .map_err(super::ReqlError::from)
            .into()
    }

    pub(crate) fn mark_change_feed(mut self) -> Self {
        self.change_feed = true;
        self
    }

    pub(crate) fn change_feed(&self) -> bool {
        self.change_feed
    }
}

impl From<Datum> for Command {
    fn from(datum: Datum) -> Self {
        Ok(datum).into()
    }
}

impl From<super::Result<Datum>> for Command {
    fn from(result: super::Result<Datum>) -> Self {
        let mut query = Self::new(TermType::Datum);
        query.datum = Some(result);
        query
    }
}

#[doc(hidden)]
impl From<Value> for Command {
    fn from(value: Value) -> Self {
        Datum::from(value).into()
    }
}

#[doc(hidden)]
impl From<super::Result<Value>> for Command {
    fn from(result: super::Result<Value>) -> Self {
        match result {
            Ok(value) => Datum::from(value).into(),
            Err(error) => (Err(error) as super::Result<Datum>).into(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Query<'a>(pub(crate) &'a Command);

impl Serialize for Query<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let Query(cmd) = self;
        match cmd.typ {
            TermType::Datum => match &cmd.datum {
                Some(Ok(datum)) => datum.serialize(serializer),
                Some(Err(error)) => Err(ser::Error::custom(error)),
                _ => (None as Option<Datum>).serialize(serializer),
            },
            _ => {
                let typ = cmd.typ as i32;
                match &cmd.opts {
                    Some(Ok(map)) => (
                        typ,
                        to_query_result(&cmd.args).map_err(ser::Error::custom)?,
                        map,
                    )
                        .serialize(serializer),
                    None => (typ, to_query_result(&cmd.args).map_err(ser::Error::custom)?)
                        .serialize(serializer),
                    Some(Err(error)) => Err(ser::Error::custom(error)),
                }
            }
        }
    }
}

fn to_query_result(args: &VecDeque<super::Result<Command>>) -> super::Result<Vec<Query<'_>>> {
    let mut vec = Vec::with_capacity(args.len());
    for result in args {
        let arg = result.as_ref().map_err(|error| error.clone())?;
        vec.push(Query(arg));
    }
    Ok(vec)
}

#[derive(Debug, Clone)]
pub(crate) struct Payload<'a>(
    pub(crate) QueryType,
    pub(crate) Option<Query<'a>>,
    pub(crate) Options,
);

impl Serialize for Payload<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let Payload(typ, qry, opts) = self;
        let typ = *typ as i32;
        match qry {
            Some(query) => (typ, query, opts).serialize(serializer),
            None => (typ,).serialize(serializer),
        }
    }
}

impl Payload<'_> {
    pub(crate) fn to_bytes(&self) -> Result<Vec<u8>, err::ReqlError> {
        Ok(serde_json::to_vec(self)?)
    }
}

// for debugging purposes only
impl fmt::Display for Payload<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // print the serialised string if we can
        if let Ok(payload) = self.to_bytes() {
            if let Ok(payload) = str::from_utf8(&payload) {
                return write!(f, "{}", payload);
            }
        }
        // otherwise just print the debug form
        write!(f, "{:?}", self)
    }
}

impl Serialize for Db {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let Self(name) = self;
        let cmd = r.db(name.as_ref());
        Query(&cmd.into()).serialize(serializer)
    }
}
