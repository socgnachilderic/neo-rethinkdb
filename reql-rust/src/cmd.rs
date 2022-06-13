pub mod add;
pub mod and;
pub mod append;
pub mod args;
pub mod asc;
pub mod avg;
pub mod between;
pub mod binary;
pub mod bit_and;
pub mod bit_not;
pub mod bit_or;
pub mod bit_sal;
pub mod bit_sar;
pub mod bit_xor;
pub mod bracket;
pub mod branch;
pub mod ceil;
pub mod change_at;
pub mod changes;
pub mod circle;
pub mod coerce_to;
pub mod concat_map;
pub mod config;
pub mod connect;
pub mod contains;
pub mod count;
pub mod date;
pub mod day;
pub mod day_of_week;
pub mod day_of_year;
pub mod db;
pub mod db_create;
pub mod db_drop;
pub mod db_list;
pub mod default;
pub mod delete;
pub mod delete_at;
pub mod desc;
pub mod difference;
pub mod distance;
pub mod distinct;
pub mod div;
pub mod do_;
pub mod downcase;
pub mod during;
pub mod epoch_time;
pub mod eq;
pub mod eq_join;
pub mod error;
pub mod expr;
pub mod fill;
pub mod filter;
pub mod floor;
pub mod fold;
pub mod for_each;
pub(crate) mod func;
pub mod ge;
pub mod geojson;
pub mod get;
pub mod get_all;
pub mod get_field;
pub mod get_intersecting;
pub mod get_nearest;
pub mod get_write_hook;
pub mod grant;
pub mod group;
pub mod gt;
pub mod has_fields;
pub mod hours;
pub mod http;
pub mod in_timezone;
pub mod includes;
pub mod index;
pub mod index_create;
pub mod index_drop;
pub mod index_list;
pub mod index_rename;
pub mod index_status;
pub mod index_wait;
pub mod info;
pub mod inner_join;
pub mod insert;
pub mod insert_at;
pub mod intersects;
pub mod is_empty;
pub mod iso8601;
pub mod js;
pub mod json;
pub mod keys;
pub mod le;
pub mod limit;
pub mod line;
pub mod literal;
pub mod lt;
pub mod map;
pub mod match_;
pub mod max;
pub mod merge;
pub mod min;
pub mod minutes;
pub mod month;
pub mod mul;
pub mod ne;
pub mod not;
pub mod now;
pub mod nth;
pub mod object;
pub mod offsets_of;
pub mod or;
pub mod order_by;
pub mod outer_join;
pub mod pluck;
pub mod point;
pub mod polygon;
pub mod polygon_sub;
pub mod prepend;
pub mod random;
pub mod range;
pub mod rebalance;
pub mod reconfigure;
pub mod reduce;
pub mod rem;
pub mod replace;
pub mod round;
pub mod run;
pub mod sample;
pub mod seconds;
pub mod set_difference;
pub mod set_insert;
pub mod set_intersection;
pub mod set_union;
pub mod set_write_hook;
pub mod skip;
pub mod slice;
pub mod splice_at;
pub mod split;
pub mod status;
pub mod sub;
pub mod sum;
pub mod sync;
pub mod table;
pub mod table_create;
pub mod table_drop;
pub mod table_list;
pub mod time;
pub mod time_of_day;
pub mod timezone;
pub mod to_epoch_time;
pub mod to_geojson;
pub mod to_iso8601;
pub mod to_json;
pub mod type_of;
pub mod ungroup;
pub mod union;
pub mod upcase;
pub mod update;
pub mod uuid;
pub mod values;
pub mod wait;
pub mod with_fields;
pub mod without;
pub mod year;
pub mod zip;

use crate::ops::{ReqlOps, ReqlOpsDocManipulation};
use crate::Command;
use async_native_tls::TlsStream;
use async_net::TcpStream;
use futures::stream::Stream;
use ql2::term::TermType;
use serde::de::DeserializeOwned;
use std::borrow::Cow;
use std::str;

pub use crate::proto::Arg;

pub trait StaticString {
    fn static_string(self) -> Cow<'static, str>;
}

#[derive(Debug)]
pub(crate) struct TcpStreamConnection {
    pub(crate) stream: TcpStream,
    pub(crate) tls_stream: Option<TlsStream<TcpStream>>,
}

impl StaticString for &'static str {
    fn static_string(self) -> Cow<'static, str> {
        Cow::from(self)
    }
}

impl StaticString for String {
    fn static_string(self) -> Cow<'static, str> {
        Cow::from(self)
    }
}

impl StaticString for &Cow<'static, str> {
    fn static_string(self) -> Cow<'static, str> {
        match self {
            Cow::Borrowed(string) => Cow::Borrowed(*string),
            Cow::Owned(string) => Cow::Owned(string.to_owned()),
        }
    }
}

impl<'a> Command {
    // pub fn merge(self, arg: impl merge::Arg) -> Self {
    //     arg.arg().into_cmd().with_parent(self)
    // }

    pub fn and(self, arg: impl and::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn or(self, arg: impl or::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn eq(self, arg: impl eq::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn ne(self, arg: impl ne::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn gt(self, arg: impl gt::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn ge(self, arg: impl ge::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn lt(self, arg: impl lt::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn le(self, arg: impl le::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn not(self, arg: impl not::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn bit_and(self, arg: impl bit_and::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn bit_or(self, arg: impl bit_or::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn bit_xor(self, arg: impl bit_xor::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn bit_not(self) -> Self {
        !self
    }

    pub fn bit_sal(self, arg: impl bit_sal::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn bit_sar(self, arg: impl bit_sar::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn round(self) -> Self {
        Self::new(TermType::Round).with_parent(self)
    }

    pub fn ceil(self) -> Self {
        Self::new(TermType::Ceil).with_parent(self)
    }

    pub fn floor(self) -> Self {
        Self::new(TermType::Floor).with_parent(self)
    }

    pub fn in_timezone(self, arg: impl in_timezone::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn timezone(self) -> Self {
        Self::new(TermType::Timezone).with_parent(self)
    }

    pub fn during(self, arg: impl during::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn date(self) -> Self {
        Self::new(TermType::Date).with_parent(self)
    }

    pub fn time_of_day(self) -> Self {
        Self::new(TermType::TimeOfDay).with_parent(self)
    }

    pub fn year(self) -> Self {
        Self::new(TermType::Year).with_parent(self)
    }

    pub fn month(self) -> Self {
        Self::new(TermType::Month).with_parent(self)
    }

    pub fn day(self) -> Self {
        Self::new(TermType::Day).with_parent(self)
    }

    pub fn day_of_week(self) -> Self {
        Self::new(TermType::DayOfWeek).with_parent(self)
    }

    pub fn day_of_year(self) -> Self {
        Self::new(TermType::DayOfYear).with_parent(self)
    }

    pub fn hours(self) -> Self {
        Self::new(TermType::Hours).with_parent(self)
    }

    pub fn minutes(self) -> Self {
        Self::new(TermType::Minutes).with_parent(self)
    }

    pub fn seconds(self) -> Self {
        Self::new(TermType::Seconds).with_parent(self)
    }

    pub fn to_iso8601(self) -> Self {
        Self::new(TermType::ToIso8601).with_parent(self)
    }

    pub fn to_epoch_time(self) -> Self {
        Self::new(TermType::ToEpochTime).with_parent(self)
    }

    pub fn binary(self, arg: impl binary::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn branch(self, arg: impl branch::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn for_each(self, arg: impl for_each::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn default(self, arg: impl default::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn coerce_to(self, arg: impl coerce_to::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn type_of(self) -> Self {
        Self::new(TermType::TypeOf).with_parent(self)
    }

    pub fn info(self) -> Self {
        Self::new(TermType::Info).with_parent(self)
    }

    pub fn to_json(self) -> Self {
        Self::new(TermType::ToJsonString).with_parent(self)
    }

    pub fn get_nearest(self, arg: impl get_nearest::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn run<A, T>(self, arg: A) -> impl Stream<Item = crate::Result<T>>
    where
        A: run::Arg,
        T: Unpin + DeserializeOwned,
    {
        Box::pin(run::new(self, arg))
    }
}

impl ReqlOpsDocManipulation for Command {}

impl ReqlOps for Command {
    fn get_parent(&self) -> Command {
        self.clone()
    }
}

// for debug purposes only
fn bytes_to_string(bytes: &[u8]) -> String {
    if let Ok(string) = str::from_utf8(bytes) {
        return string.to_owned();
    }
    format!("{:?}", bytes)
}

#[cfg(test)]
fn serialise(cmd: &Command) -> String {
    serde_json::to_string(&crate::proto::Query(cmd)).unwrap()
}
