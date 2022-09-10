// pub mod add;
// pub mod and;
// pub mod append;
pub mod args;
// pub mod asc;
// pub mod avg;
pub mod between;
// pub mod binary;
// pub mod bit_and;
// pub mod bit_not;
// pub mod bit_or;
// pub mod bit_sal;
// pub mod bit_sar;
// pub mod bit_xor;
// pub mod bracket;
// pub mod branch;
// pub mod ceil;
// pub mod change_at;
// pub mod changes;
// pub mod circle;
// pub mod coerce_to;
pub mod concat_map;
// pub mod config;
pub mod connect;
// pub mod contains;
// pub mod count;
// pub mod date;
// pub mod day;
// pub mod day_of_week;
// pub mod day_of_year;
pub mod db;
pub mod db_create;
pub mod db_drop;
pub mod db_list;
// pub mod default;
pub mod delete;
// pub mod delete_at;
// pub mod desc;
// pub mod difference;
// pub mod distance;
// pub mod distinct;
// pub mod div;
// pub mod do_;
// pub mod downcase;
// pub mod during;
// pub mod epoch_time;
// pub mod eq;
pub mod eq_join;
// pub mod error;
pub mod expr;
// pub mod fill;
pub mod filter;
// pub mod floor;
// pub mod fold;
// pub mod for_each;
pub(crate) mod func;
// pub mod ge;
// pub mod geojson;
pub mod get;
pub mod get_all;
// pub mod get_field;
// pub mod get_intersecting;
// pub mod get_nearest;
pub mod get_write_hook;
// pub mod grant;
pub mod group;
// pub mod gt;
// pub mod has_fields;
// pub mod hours;
// pub mod http;
// pub mod in_timezone;
// pub mod includes;
// pub mod index;
pub mod index_create;
pub mod index_drop;
pub mod index_list;
pub mod index_rename;
pub mod index_status;
pub mod index_wait;
// pub mod info;
pub mod inner_join;
pub mod insert;
// pub mod insert_at;
// pub mod intersects;
pub mod is_empty;
// pub mod iso8601;
// pub mod js;
// pub mod json;
// pub mod keys;
// pub mod le;
pub mod limit;
// pub mod line;
// pub mod literal;
// pub mod lt;
pub mod map;
// pub mod match_;
// pub mod max;
// pub mod merge;
// pub mod min;
// pub mod minutes;
// pub mod month;
// pub mod mul;
// pub mod ne;
// pub mod not;
// pub mod now;
pub mod nth;
// pub mod object;
pub mod offsets_of;
// pub mod or;
pub mod order_by;
pub mod outer_join;
// pub mod pluck;
// pub mod point;
// pub mod polygon;
// pub mod polygon_sub;
// pub mod prepend;
// pub mod random;
// pub mod range;
// pub mod rebalance;
// pub mod reconfigure;
// pub mod reduce;
// pub mod rem;
pub mod replace;
// pub mod round;
pub mod run;
pub mod sample;
// pub mod seconds;
// pub mod set_difference;
// pub mod set_insert;
// pub mod set_intersection;
// pub mod set_union;
pub mod set_write_hook;
pub mod skip;
pub mod slice;
// pub mod splice_at;
// pub mod split;
// pub mod status;
// pub mod sub;
// pub mod sum;
pub mod sync;
pub mod table;
pub mod table_create;
pub mod table_drop;
pub mod table_list;
// pub mod time;
// pub mod time_of_day;
// pub mod timezone;
// pub mod to_epoch_time;
// pub mod to_geojson;
// pub mod to_iso8601;
// pub mod to_json;
// pub mod type_of;
// pub mod ungroup;
pub mod union;
// pub mod upcase;
pub mod update;
// pub mod uuid;
// pub mod values;
// pub mod wait;
pub mod with_fields;
// pub mod without;
// pub mod year;
pub mod zip;

use std::borrow::Cow;
use std::str;

use async_native_tls::TlsStream;
use async_net::TcpStream;
use futures::stream::Stream;
use futures::TryStreamExt;
use ql2::term::TermType;
use serde::Serialize;
use serde_json::Value;

pub use crate::proto::Arg;
use crate::Command;
use crate::Func;
use crate::Result;
use crate::types::AnyParam;

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
    pub fn table_create(self, args: impl table_create::TableCreateArg) -> Self {
        table_create::new(args).with_parent(self)
    }

    pub fn table_drop(self, table_name: &str) -> Self {
        table_drop::new(table_name).with_parent(self)
    }

    pub fn table_list(self) -> Self {
        table_list::new().with_parent(self)
    }

    pub fn table(self, args: impl table::TableArg) -> Self {
        table::new(args).with_parent(self)
    }

    pub fn index_create(self, args: impl index_create::IndexCreateArg) -> Self {
        index_create::new(args).with_parent(self)
    }

    pub fn index_drop(self, index_name: &str) -> Self {
        index_drop::new(index_name).with_parent(self)
    }

    pub fn index_list(self) -> Self {
        index_list::new().with_parent(self)
    }

    pub fn index_rename(self, args: impl index_rename::IndexRenameArg) -> Self {
        index_rename::new(args).with_parent(self)
    }

    pub fn index_status(self, args: impl index_status::IndexStatusArg) -> Self {
        index_status::new(args).with_parent(self)
    }

    pub fn index_wait(self, args: impl index_wait::IndexWaitArg) -> Self {
        index_wait::new(args).with_parent(self)
    }

    pub fn set_write_hook(self, args: Option<impl set_write_hook::SetWriteHookArg>) -> Self {
        set_write_hook::new(args).with_parent(self)
    }

    pub fn get_write_hook(self) -> Self {
        get_write_hook::new().with_parent(self)
    }

    pub fn insert(self, args: impl insert::InsertArg) -> Self {
        insert::new(args).with_parent(self)
    }

    pub fn update(self, args: impl update::UpdateArg) -> Self {
        update::new(args).with_parent(self)
    }

    pub fn replace(self, args: impl replace::ReplaceArg) -> Self {
        replace::new(args).with_parent(self)
    }

    pub fn delete(self, args: impl delete::DeleteArg) -> Self {
        delete::new(args).with_parent(self)
    }

    pub fn sync(self) -> Self {
        sync::new().with_parent(self)
    }

    pub fn get(self, primary_key: impl Serialize) -> Self {
        get::new(primary_key).with_parent(self)
    }

    pub fn get_all(self, values: impl get_all::GetAllArg) -> Self {
        get_all::new(values).with_parent(self)
    }

    pub fn between(self, args: impl between::BetweenArg) -> Self {
        between::new(args).with_parent(self)
    }

    pub fn filter(self, args: impl filter::FilterArg) -> Self {
        filter::new(args).with_parent(self)
    }

    pub fn inner_join(self, other_sequence: Command, func: Func) -> Self {
        inner_join::new(other_sequence, func).with_parent(self)
    }

    pub fn outer_join(self, other_sequence: Command, func: Func) -> Self {
        outer_join::new(other_sequence, func).with_parent(self)
    }

    pub fn eq_join(self, args: impl eq_join::EqJoinArg) -> Self {
        eq_join::new(args).with_parent(self)
    }

    pub fn zip(self) -> Self {
        zip::new().with_parent(self)
    }

    pub fn map(self, args: impl map::MapArg) -> Self {
        map::new(args).with_parent(self)
    }

    pub fn with_fields(self, fields: AnyParam) -> Self {
        with_fields::new(fields).with_parent(self)
    }

    pub fn concat_map(self, func: Func) -> Command {
        concat_map::new(func).with_parent(self)
    }

    pub fn order_by(self, args: impl order_by::OrderByArg) -> Self {
        order_by::new(args).with_parent(self)
    }

    pub fn skip(self, number_of_element: usize) -> Self {
        skip::new(number_of_element).with_parent(self)
    }

    pub fn limit(self, number_of_element: usize) -> Self {
        limit::new(number_of_element).with_parent(self)
    }

    pub fn slice(self, args: impl slice::SliceArg) -> Self {
        slice::new(args).with_parent(self)
    }

    pub fn nth(self, index: isize) -> Self {
        nth::new(index).with_parent(self)
    }

    pub fn offsets_of(self, args: impl offsets_of::OffsetsOfArg) -> Self {
        offsets_of::new(args).with_parent(self)
    }

    pub fn is_empty(self) -> Self {
        is_empty::new().with_parent(self)
    }

    pub fn union(self, args: impl union::UnionArg) -> Self {
        union::new(args).with_parent(self)
    }

    pub fn sample(self, number: usize) -> Self {
        sample::new(number).with_parent(self)
    }

    pub fn group(self, args: impl group::GroupArg) -> Self {
        group::new(args).with_parent(self)
    }

    // fn ungroup();

    // fn reduce<A>(&self, func: Func) -> cmd::reduce::ReduceBuilder<A>
    // where
    //     A: Unpin + Serialize + DeserializeOwned,
    // {
    //     cmd::reduce::ReduceBuilder::new(func)._with_parent(self.get_parent())
    // }

    // fn fold<A, B>(&self, base: A, func: Func) -> cmd::fold::FoldBuilder<A, B>
    // where
    //     A: Serialize,
    //     B: Unpin + Serialize + DeserializeOwned,
    // {
    //     cmd::fold::FoldBuilder::new(base, func)._with_parent(self.get_parent())
    // }

    // fn count();

    // fn sum(&self) -> cmd::sum::SumBuilder {
    //     cmd::sum::SumBuilder::new()._with_parent(self.get_parent())
    // }

    // fn avg(&self) -> cmd::avg::AvgBuilder {
    //     cmd::avg::AvgBuilder::new()._with_parent(self.get_parent())
    // }

    // fn min(&self) -> cmd::min::MinBuilder<T> {
    //     cmd::min::MinBuilder::new()._with_parent(self.get_parent())
    // }

    // fn max(&self) -> cmd::max::MaxBuilder<T> {
    //     cmd::max::MaxBuilder::new()._with_parent(self.get_parent())
    // }

    // fn distinct(&self) -> cmd::distinct::DistinctBuilder<Sequence<T>> {
    //     cmd::distinct::DistinctBuilder::new()._with_parent(self.get_parent())
    // }

    // fn contains(&self, values: impl Serialize) -> cmd::contains::ContainsBuilder {
    //     cmd::contains::ContainsBuilder::new(values)._with_parent(self.get_parent())
    // }

    // fn keys(&self) -> cmd::keys::KeysBuilder {
    //     cmd::keys::KeysBuilder::new()._with_parent(self.get_parent())
    // }

    // pub fn merge(self, arg: impl merge::Arg) -> Self {
    //     arg.arg().into_cmd().with_parent(self)
    // }

    /* pub fn and(self, arg: impl and::Arg) -> Self {
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
    } */

    pub fn round(self) -> Self {
        Self::new(TermType::Round).with_parent(self)
    }

    pub fn ceil(self) -> Self {
        Self::new(TermType::Ceil).with_parent(self)
    }

    pub fn floor(self) -> Self {
        Self::new(TermType::Floor).with_parent(self)
    }

    // pub fn timezone(self) -> Self {
    //     Self::new(TermType::Timezone).with_parent(self)
    // }

    // pub fn during(self, arg: impl during::Arg) -> Self {
    //     arg.arg().into_cmd().with_parent(self)
    // }

    // pub fn date(self) -> Self {
    //     Self::new(TermType::Date).with_parent(self)
    // }

    // pub fn time_of_day(self) -> Self {
    //     Self::new(TermType::TimeOfDay).with_parent(self)
    // }

    // pub fn year(self) -> Self {
    //     Self::new(TermType::Year).with_parent(self)
    // }

    // pub fn month(self) -> Self {
    //     Self::new(TermType::Month).with_parent(self)
    // }

    // pub fn day(self) -> Self {
    //     Self::new(TermType::Day).with_parent(self)
    // }

    // pub fn day_of_week(self) -> Self {
    //     Self::new(TermType::DayOfWeek).with_parent(self)
    // }

    // pub fn day_of_year(self) -> Self {
    //     Self::new(TermType::DayOfYear).with_parent(self)
    // }

    // pub fn hours(self) -> Self {
    //     Self::new(TermType::Hours).with_parent(self)
    // }

    // pub fn minutes(self) -> Self {
    //     Self::new(TermType::Minutes).with_parent(self)
    // }

    // pub fn seconds(self) -> Self {
    //     Self::new(TermType::Seconds).with_parent(self)
    // }

    // pub fn to_iso8601(self) -> Self {
    //     Self::new(TermType::ToIso8601).with_parent(self)
    // }

    // pub fn to_epoch_time(self) -> Self {
    //     Self::new(TermType::ToEpochTime).with_parent(self)
    // }

    /* pub fn binary(self, arg: impl binary::Arg) -> Self {
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
    } */

    pub fn type_of(self) -> Self {
        Self::new(TermType::TypeOf).with_parent(self)
    }

    pub fn info(self) -> Self {
        Self::new(TermType::Info).with_parent(self)
    }

    pub fn to_json(self) -> Self {
        Self::new(TermType::ToJsonString).with_parent(self)
    }

    pub async fn run(self, arg: impl run::Arg) -> Result<Option<Value>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(self, arg: impl run::Arg) -> impl Stream<Item = Result<Value>> {
        Box::pin(run::new(self, arg))
    }
}

pub enum CmdOpts {
    Single(Command),
    Many(Vec<Command>),
}

impl CmdOpts {
    pub(crate) fn add_to_cmd(self, command: Command) -> Command {
        match self {
            Self::Single(arg) => command.with_arg(arg),
            Self::Many(args) => args.into_iter().fold(command, |cmd, arg| cmd.with_arg(arg)),
        }
    }
}

impl Into<Option<Command>> for CmdOpts {
    fn into(self) -> Option<Command> {
        if let CmdOpts::Single(arg) = self {
            Some(arg)
        } else {
             None
        }
    }
}

// for debug purposes only
fn bytes_to_string(bytes: &[u8]) -> String {
    if let Ok(string) = str::from_utf8(bytes) {
        return string.to_owned();
    }
    format!("{:?}", bytes)
}
