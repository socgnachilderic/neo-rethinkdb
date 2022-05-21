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

use crate::Command;
use futures::stream::Stream;
use ql2::term::TermType;
use serde::de::DeserializeOwned;
use std::borrow::Cow;
use std::str;

pub use crate::proto::Arg;

pub trait StaticString {
    fn static_string(self) -> Cow<'static, str>;
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

// for debug purposes only
fn bytes_to_string(bytes: &[u8]) -> String {
    if let Ok(string) = str::from_utf8(bytes) {
        return string.to_owned();
    }
    format!("{:?}", bytes)
}

impl<'a> Command {
    pub fn changes(self, arg: impl changes::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn index_create(self, arg: impl index_create::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    /// Delete a previously created secondary index of this table.
    /// 
    /// ## Example
    /// 
    /// Drop a secondary index named ‘code_name’.
    /// 
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// 
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.db("heroes")
    ///         .table("dc_universe")
    ///         .index_drop("code_name")
    ///         .run(&session)
    ///         .try_next().await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    pub fn index_drop(self, index_name: &str) -> index_drop::IndexDropBuilder {
        index_drop::IndexDropBuilder::new(index_name)._with_parent(self)
    }

    /// List all the secondary indexes of this table.
    /// 
    /// ## Example
    /// 
    /// List the available secondary indexes for this table.
    /// 
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// 
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.db("heroes")
    ///         .table("dc_universe")
    ///         .index_list()
    ///         .run(&session)
    ///         .try_next().await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    pub fn index_list(self) -> index_list::IndexListBuilder {
        index_list::IndexListBuilder::new()._with_parent(self)
    }

    /// Rename an existing secondary index on a table. 
    /// 
    /// If the `overwrite` option is specified as `true`, a previously existing index with the new name will be deleted and the index will be renamed.
    /// If `overwrite` is `false` (the default) an error will be raised if the new index name already exists.
    /// 
    /// The return value on success will be an object of the format `{"renamed": 1}`, or `{"renamed": 0}` if the old and new names are the same.
    /// 
    /// An error will be raised if the old index name does not exist, if the new index name is already in use and 
    /// `overwrite` is `false`, or if either the old or new index name are the same as the primary key field name.
    /// 
    /// ## Example
    /// 
    /// Rename an index on the comments table.
    /// 
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// 
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.db("heroes")
    ///         .table("comments")
    ///         .index_rename("postId", "messageId")
    ///         .run(&session)
    ///         .try_next().await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// ## Example
    /// 
    /// Rename an index on the users table, overwriting any existing index with the new name.
    /// 
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// 
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.db("heroes")
    ///         .table("users")
    ///         .index_rename("mail", "email")
    ///         .with_overwrite(true)
    ///         .run(&session)
    ///         .try_next().await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    pub fn index_rename(self, old_index_name: &str, new_index_name: &str) -> index_rename::IndexRenameBuilder {
        index_rename::IndexRenameBuilder::new(old_index_name, new_index_name)._with_parent(self)
    }

    pub fn index_status(self, arg: impl index_status::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn index_wait(self, arg: impl index_wait::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn set_write_hook(self, arg: impl set_write_hook::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn get_write_hook(self) -> Self {
        Self::new(TermType::GetWriteHook).with_parent(self)
    }

    pub fn insert(self, arg: impl insert::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn update(self, arg: impl update::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn replace(self, arg: impl replace::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn delete(self, arg: impl delete::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn sync(self) -> Self {
        Self::new(TermType::Sync).with_parent(self)
    }

    pub fn get(self, arg: impl get::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn get_all(self, arg: impl get_all::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn between(self, arg: impl between::Arg<'a>) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn filter(self, arg: impl filter::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn inner_join(self, arg: impl inner_join::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn outer_join(self, arg: impl outer_join::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn eq_join(self, arg: impl eq_join::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn zip(self) -> Self {
        Self::new(TermType::Zip).with_parent(self)
    }

    pub fn map(self, arg: impl map::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn with_fields(self, arg: impl with_fields::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn concat_map(self, arg: impl concat_map::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    /// Orders the result based on the given column.
    ///
    /// Argument can either be a string, `r.asc("column")` for ascending or `r.desc("column")` for descending.
    /// If the given argument is a string, the direction will default to ascending.
    ///
    /// ## Example
    ///
    /// Sort the result in descending order based on the `created_at` column.
    // ```
    // # reql_rust::example(|r, conn| async_stream::stream! {
    // r.db("database").table("users").order_by(r.desc("created_at")).run(conn)
    // # });
    // ```
    ///
    pub fn order_by(self, arg: impl order_by::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn skip(self, arg: impl skip::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    /// Limits the result set to the given amount.
    ///
    /// Argument can be an isize.
    ///
    /// ## Example
    ///
    /// Get 5 records.
    /// ```
    /// # reql_rust::example(|r, conn| async_stream::stream! {
    /// r.db("database").table("users").limit(5).run(conn)
    /// # });
    /// ```
    ///
    pub fn limit(self, arg: impl limit::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn slice(self, arg: impl slice::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn nth(self, arg: impl nth::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn offsets_of(self, arg: impl offsets_of::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn is_empty(self) -> Self {
        Self::new(TermType::IsEmpty).with_parent(self)
    }

    pub fn union(self, arg: impl union::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn sample(self, arg: impl sample::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn group(self, arg: impl group::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn ungroup(self) -> Self {
        Self::new(TermType::Ungroup).with_parent(self)
    }

    pub fn reduce(self, arg: impl reduce::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn fold(self, arg: impl fold::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn count(self, arg: impl count::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn sum(self, arg: impl sum::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn avg(self, arg: impl avg::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn min(self, arg: impl min::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn max(self, arg: impl max::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn distinct(self, arg: impl distinct::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn contains(self, arg: impl contains::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn pluck(self, arg: impl pluck::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn without(self, arg: impl without::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn merge(self, arg: impl merge::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn append(self, arg: impl append::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn prepend(self, arg: impl prepend::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn difference(self, arg: impl difference::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn set_insert(self, arg: impl set_insert::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn set_union(self, arg: impl set_union::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn set_intersection(self, arg: impl set_intersection::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn set_difference(self, arg: impl set_difference::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn bracket(self, arg: impl bracket::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn get_field(self, arg: impl get_field::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn has_fields(self, arg: impl has_fields::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn insert_at(self, arg: impl insert_at::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn splice_at(self, arg: impl splice_at::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn delete_at(self, arg: impl delete_at::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn change_at(self, arg: impl change_at::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn keys(self) -> Self {
        Self::new(TermType::Keys).with_parent(self)
    }

    pub fn values(self) -> Self {
        Self::new(TermType::Values).with_parent(self)
    }

    pub fn match_(self, arg: impl match_::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn split(self, arg: impl split::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn upcase(self) -> Self {
        Self::new(TermType::Upcase).with_parent(self)
    }

    pub fn downcase(self) -> Self {
        Self::new(TermType::Downcase).with_parent(self)
    }

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

    pub fn do_(self, arg: impl do_::Arg) -> Self {
        arg.arg(Some(self)).into_cmd()
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

    pub fn distance(self, arg: impl distance::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn fill(self) -> Self {
        Self::new(TermType::Fill).with_parent(self)
    }

    pub fn to_geojson(self) -> Self {
        Self::new(TermType::ToGeojson).with_parent(self)
    }

    pub fn get_intersecting(self, arg: impl get_intersecting::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn get_nearest(self, arg: impl get_nearest::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn includes(self, arg: impl includes::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn intersects(self, arg: impl intersects::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn polygon_sub(self, arg: impl polygon_sub::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn grant(self, arg: impl grant::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn config(self) -> Self {
        Self::new(TermType::Config).with_parent(self)
    }

    pub fn rebalance(self) -> Self {
        Self::new(TermType::Rebalance).with_parent(self)
    }

    pub fn reconfigure(self, arg: impl reconfigure::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn status(self) -> Self {
        Self::new(TermType::Status).with_parent(self)
    }

    pub fn wait(self, arg: impl wait::Arg) -> Self {
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

#[cfg(test)]
fn serialise(cmd: &Command) -> String {
    serde_json::to_string(&crate::proto::Query(cmd)).unwrap()
}
