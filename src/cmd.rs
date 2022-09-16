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
// pub mod date;
// pub mod day;
// pub mod day_of_week;
// pub mod day_of_year;
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
// pub mod during;
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
// pub mod hours;
pub mod http;
pub mod in_timezone;
pub mod includes;
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
// pub mod iso8601;
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
// pub mod minutes;
// pub mod month;
pub mod mul;
pub mod ne;
pub mod not_;
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
// pub mod seconds;
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
// pub mod time_of_day;
// pub mod timezone;
// pub mod to_epoch_time;
pub mod to_geojson;
// pub mod to_iso8601;
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
// pub mod year;
pub mod zip;

use std::borrow::Cow;
use std::ops::{BitAnd, BitOr, BitXor};
use std::str;

use async_native_tls::TlsStream;
use async_net::TcpStream;
use futures::stream::Stream;
use futures::TryStreamExt;
use regex::Regex;
use serde::Serialize;
use serde_json::Value;

use crate::arguments::AnyParam;
use crate::prelude::{Func, Geometry};
use crate::Command;
use crate::Result;

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
    pub fn changes(self, args: impl changes::ChangesArg) -> Self {
        changes::new(args).with_parent(self)
    }

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

    pub fn skip(self, n: usize) -> Self {
        skip::new(n).with_parent(self)
    }

    pub fn limit(self, n: usize) -> Self {
        limit::new(n).with_parent(self)
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

    pub fn ungroup(self) -> Self {
        ungroup::new().with_parent(self)
    }

    pub fn reduce(self, func: Func) -> Self {
        reduce::new(func).with_parent(self)
    }

    pub fn fold(self, base: AnyParam, func: Func) -> Self {
        fold::new(base, func).with_parent(self)
    }

    pub fn count(self, args: impl count::CountArg) -> Self {
        count::new(args).with_parent(self)
    }

    pub fn sum(self, args: impl sum::SumArg) -> Self {
        sum::new(args).with_parent(self)
    }

    pub fn avg(self, args: impl avg::AvgArg) -> Self {
        avg::new(args).with_parent(self)
    }

    pub fn min(self, args: impl min::MinArg) -> Self {
        min::new(args).with_parent(self)
    }

    pub fn max(self, args: impl max::MaxArg) -> Self {
        max::new(args).with_parent(self)
    }

    pub fn distinct(self, args: impl distinct::DistinctArg) -> Self {
        distinct::new(args).with_parent(self)
    }

    pub fn contains(self, args: impl contains::ContainsArg) -> Self {
        contains::new(args).with_parent(self)
    }

    pub fn pluck(self, selector: impl Serialize) -> Self {
        pluck::new(selector).with_parent(self)
    }

    pub fn without(self, selector: impl Serialize) -> Self {
        without::new(selector).with_parent(self)
    }

    pub fn merge(self, args: impl merge::MergeArg) -> Self {
        merge::new(args).with_parent(self)
    }

    pub fn append(self, value: impl Serialize) -> Self {
        append::new(value).with_parent(self)
    }

    pub fn prepend(self, value: impl Serialize) -> Self {
        prepend::new(value).with_parent(self)
    }

    pub fn difference(self, values: Vec<impl Serialize>) -> Self {
        difference::new(values).with_parent(self)
    }

    pub fn set_insert(self, value: impl Serialize) -> Self {
        set_insert::new(value).with_parent(self)
    }

    pub fn set_union(self, values: Vec<impl Serialize>) -> Self {
        set_union::new(values).with_parent(self)
    }

    pub fn set_intersection(self, values: Vec<impl Serialize>) -> Self {
        set_intersection::new(values).with_parent(self)
    }

    pub fn set_difference(self, values: Vec<impl Serialize>) -> Self {
        set_difference::new(values).with_parent(self)
    }

    pub fn bracket(self, value: impl Serialize) -> Self {
        bracket::new(value).with_parent(self)
    }

    pub fn get_field(self, attr: impl Serialize) -> Self {
        get_field::new(attr).with_parent(self)
    }

    pub fn g(self, attr: impl Serialize) -> Self {
        get_field::new(attr).with_parent(self)
    }

    pub fn has_fields(self, selector: impl Serialize) -> Self {
        has_fields::new(selector).with_parent(self)
    }

    pub fn insert_at(self, offset: isize, value: impl Serialize) -> Self {
        insert_at::new(offset, value).with_parent(self)
    }

    pub fn splice_at(self, offset: isize, value: impl Serialize) -> Self {
        splice_at::new(offset, value).with_parent(self)
    }

    pub fn delete_at(self, args: impl delete_at::DeleteAtArg) -> Self {
        delete_at::new(args).with_parent(self)
    }

    pub fn change_at(self, offset: isize, value: impl Serialize) -> Self {
        change_at::new(offset, value).with_parent(self)
    }

    pub fn keys(self) -> Self {
        keys::new().with_parent(self)
    }

    pub fn values(self) -> Self {
        values::new().with_parent(self)
    }

    pub fn match_(self, regexp: Regex) -> Self {
        match_::new(regexp).with_parent(self)
    }

    pub fn split(self, args: impl split::SplitArg) -> Self {
        split::new(args).with_parent(self)
    }

    pub fn upcase(self) -> Self {
        upcase::new().with_parent(self)
    }

    pub fn downcase(self) -> Self {
        downcase::new().with_parent(self)
    }

    pub fn and(self, args: impl and::AndArg) -> Self {
        and::new(args).with_parent(self)
    }

    pub fn or(self, args: impl or::OrArg) -> Self {
        or::new(args).with_parent(self)
    }

    pub fn eq(self, args: impl eq::EqArg) -> Self {
        eq::new(args).with_parent(self)
    }

    pub fn ne(self, args: impl ne::NeArg) -> Self {
        ne::new(args).with_parent(self)
    }

    pub fn gt(self, args: impl gt::GtArg) -> Self {
        gt::new(args).with_parent(self)
    }

    pub fn ge(self, args: impl ge::GeArg) -> Self {
        ge::new(args).with_parent(self)
    }

    pub fn lt(self, args: impl lt::LtArg) -> Self {
        lt::new(args).with_parent(self)
    }

    pub fn le(self, args: impl le::LeArg) -> Self {
        le::new(args).with_parent(self)
    }

    pub fn not_(self) -> Self {
        not_::new(()).with_parent(self)
    }

    pub fn round(self) -> Self {
        round::new(()).with_parent(self)
    }

    pub fn ceil(self) -> Self {
        ceil::new(()).with_parent(self)
    }

    pub fn floor(self) -> Self {
        floor::new(()).with_parent(self)
    }

    pub fn bit_and(self, args: impl bit_and::BitAndArg) -> Self {
        self.bitand(args)
    }

    pub fn bit_or(self, args: impl bit_or::BitOrArg) -> Self {
        self.bitor(args)
    }

    pub fn bit_xor(self, args: impl bit_xor::BitXorArg) -> Self {
        self.bitxor(args)
    }

    pub fn bit_not(self) -> Self {
        !self
    }

    pub fn bit_sal(self, args: impl bit_sal::BitSalArg) -> Self {
        bit_sal::new(args).with_parent(self)
    }

    pub fn bit_sar(self, args: impl bit_sar::BitSarArg) -> Self {
        bit_sar::new(args).with_parent(self)
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

    pub fn do_(self, args: impl do_::DoArg) -> Self {
        do_::new(args).with_parent(self)
    }

    pub fn branch(self, args: impl branch::BranchArg) -> Self {
        branch::new(args).with_parent(self)
    }

    pub fn for_each(self, write_function: Func) -> Self {
        for_each::new(write_function).with_parent(self)
    }

    pub fn default(self, args: impl default::DefaultArg) -> Self {
        default::new(args).with_parent(self)
    }

    pub fn coerce_to(self, value: impl Serialize) -> Self {
        coerce_to::new(value).with_parent(self)
    }

    pub fn type_of(self) -> Self {
        type_of::new().with_parent(self)
    }

    pub fn info(self) -> Self {
        info::new().with_parent(self)
    }

    pub fn to_json(self) -> Self {
        to_json::new().with_parent(self)
    }

    pub fn distance(self, args: impl distance::DistanceArg) -> Self {
        distance::new(args).with_parent(self)
    }

    pub fn to_geojson(self) -> Self {
        to_geojson::new().with_parent(self)
    }

    pub fn get_intersecting(self, geometry: impl Geometry, index: &'static str) -> Self {
        get_intersecting::new(geometry, index).with_parent(self)
    }

    pub fn get_nearest(self, args: impl get_nearest::GetNearestArg) -> Self {
        get_nearest::new(args).with_parent(self)
    }

    pub fn includes(self, args: impl includes::IncludesArg) -> Self {
        includes::new(args).with_parent(self)
    }

    pub fn intersects(self, geometry: impl Geometry) -> Self {
        intersects::new(geometry).with_parent(self)
    }

    pub fn grant(self, args: impl grant::GrantArg) -> Self {
        grant::new(args).with_parent(self)
    }

    pub fn config(self) -> Self {
        config::new().with_parent(self)
    }

    pub fn rebalance(self) -> Self {
        rebalance::new().with_parent(self)
    }

    pub fn reconfigure(self, opts: reconfigure::ReconfigureOption) -> Self {
        reconfigure::new(opts).with_parent(self)
    }

    pub fn status(self) -> Self {
        status::new().with_parent(self)
    }

    /// Wait for a table or all the tables in a database to be ready.
    /// A table may be temporarily unavailable after creation,
    /// rebalancing or reconfiguring.
    /// The `wait` command blocks until the given
    /// table (or database) is fully up to date.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// table.wait(()) → response
    /// database.wait(()) → response
    /// r.wait(table) → response
    /// r.wait(database) → response
    /// table.wait(options) → response
    /// database.wait(options) → response
    /// r.wait(args!(table, options)) → response
    /// r.wait(args!(database, options)) → response
    /// ```
    ///
    /// Where:
    /// - table: [r.table(...)](crate::r::table) |
    /// [query.table(...)](Self::table)
    /// - database: [r.db(...)](crate::r::db)
    /// - options: [WaitOption](crate::cmd::wait::WaitOption)
    /// - response: [WaitResponse](crate::types::WaitResponse)
    ///
    /// ## Examples
    ///
    /// Wait on a table to be ready.
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::types::WaitResponse;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: WaitResponse = r.table("simbad")
    ///         .wait(())
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.ready == 1);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Wait on a table with timeout to be ready for reads.
    ///
    /// ```
    /// use reql_rust::arguments::WaitFor;
    /// use reql_rust::cmd::wait::WaitOption;
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::types::WaitResponse;
    /// use reql_rust::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let table_command = r.db("anim").table("simbad");
    ///     let opts = WaitOption::default()
    ///         .wait_for(WaitFor::ReadyForReads)
    ///         .timeout(8000f64);
    ///
    ///     let response: WaitResponse =  r.wait(args!(table_command, opts))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.ready == 1);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn wait(self, args: impl wait::WaitArg) -> Self {
        wait::new(args).with_parent(self)
    }

    /// Run a query on a connection,
    /// returning either a single JSON result or a cursor,
    /// depending on the query.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// query.run(&session) → value
    /// query.run(connection) → value
    /// query.run(args!(&session, options)) → value
    /// query.run(args!(connection, options)) → value
    /// query.run(&mut session) → value
    /// query.run(args!(&mut session, options)) → value
    /// ```
    ///
    /// Where:
    /// - session: [Session](crate::connection::Session)
    /// - connection: [Connection](crate::connection::Connection)
    /// - options: [RunOption](crate::cmd::run::RunOption)
    /// - stream: [impl Stream<Item = Result<Value>>](futures::stream::Stream)
    ///
    /// ## Examples
    ///
    /// If you are OK with potentially out of date data
    /// from all the tables involved in this query and
    /// want potentially faster reads,
    /// pass a flag allowing out of date data in an options object.
    /// Settings for individual tables will supercede this global
    /// setting for all tables in the query.
    ///
    /// ```
    /// use reql_rust::arguments::ReadMode;
    /// use reql_rust::cmd::run::RunOption;
    /// use reql_rust::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let opts = RunOption::default().read_mode(ReadMode::Outdated);
    ///
    ///     r.table("simbad").run(args!(&conn, opts)).await?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// If you want to specify whether to wait for a write to be written
    /// to disk (overriding the table’s default settings),
    /// you can set `durability` to `Durability::Hard`
    /// or `Durability::Soft` in the options.
    ///
    /// ```
    /// use reql_rust::arguments::Durability;
    /// use reql_rust::cmd::run::RunOption;
    /// use reql_rust::{args, r, Result};
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let opts = RunOption::default().durability(Durability::Hard);
    ///     let data = json!({
    ///         "name": "Pumba",
    ///         "live": 5
    ///     });
    ///
    ///     r.table("simbad").insert(data).run(args!(&conn, opts)).await?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// If you do not want a time object to be converted to a native date object,
    /// you can pass a time_format flag to prevent it
    /// (valid flags are `Format::Raw` and `Format::Native`).
    /// This query returns an object with two fields (epoch_time and $reql_type$)
    /// instead of a native date object.
    ///
    /// ```
    /// use reql_rust::arguments::Format;
    /// use reql_rust::cmd::run::RunOption;
    /// use reql_rust::{args, r, Result};
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let opts = RunOption::default().time_format(Format::Raw);
    ///
    ///     r.now().cmd().run(args!(&conn, opts)).await?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Specify the database to use for the query.
    ///
    /// ```
    /// use reql_rust::arguments::Format;
    /// use reql_rust::cmd::run::RunOption;
    /// use reql_rust::{args, r, Result};
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let opts = RunOption::default().db("jikoni");
    ///
    ///     r.table("simbad").run(args!(&conn, opts)).await?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Change the batching parameters for this query.
    ///
    /// ```
    /// use reql_rust::arguments::Format;
    /// use reql_rust::cmd::run::RunOption;
    /// use reql_rust::{args, r, Result};
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let opts = RunOption::default()
    ///         .max_batch_rows(16)
    ///         .max_batch_bytes(2048);
    ///
    ///     r.table("simbad").run(args!(&conn, opts)).await?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [connection](crate::r::connection)
    pub async fn run(self, arg: impl run::RunArg) -> Result<Option<Value>> {
        self.make_query(arg).try_next().await
    }

    /// Prepare query for execution
    ///
    /// This method has the same parameters as `run`.
    /// The main difference between `make_query` and `run` is that
    /// `make_query` can be used to execute multiple requests
    ///
    /// See [run](self::run) for more information.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// query.make_query(&session) → stream
    /// query.make_query(connection) → stream
    /// query.make_query(args!(&session, options)) → stream
    /// query.make_query(args!(connection, options)) → stream
    /// query.make_query(&mut session) → stream
    /// query.make_query(args!(&mut session, options)) → stream
    /// ```
    ///
    /// Where:
    /// - session: [Session](crate::connection::Session)
    /// - connection: [Connection](crate::connection::Connection)
    /// - options: [RunOption](crate::cmd::run::RunOption)
    ///
    /// ## Examples
    ///
    /// You can use `query.make_query` to get the same result than `query.run`
    ///
    /// ```
    /// use futures::TryStreamExt;
    /// use reql_rust::r;
    ///
    /// async fn example() -> reql_rust::Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     r.table("simbad").make_query(&conn).try_next().await?;
    ///     // is same than
    ///     r.table("simbad").run(&conn).await?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Run many queries
    ///
    /// ```
    /// use futures::stream::{select_all, TryStreamExt};
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::r;
    ///
    /// async fn example() -> reql_rust::Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let mut streams = Vec::new();
    ///     let expected_messages: Vec<String> = (0..10_000)
    ///         .into_iter()
    ///         .map(|i| format!("message {}", i))
    ///         .collect();
    ///
    ///     for msg in expected_messages.iter() {
    ///         streams.push(r.expr(msg).make_query(&conn));
    ///     }
    ///
    ///     let mut list = select_all(streams);
    ///     let mut response = Vec::new();
    ///
    ///     while let Some(msg) = list.try_next().await? {
    ///         let msg: String = msg.parse()?;
    ///         response.push(msg);
    ///     }
    ///
    ///     assert!(response == expected_messages);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [run](self::run)
    pub fn make_query(self, arg: impl run::RunArg) -> impl Stream<Item = Result<Value>> {
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

impl From<CmdOpts> for Option<Command> {
    fn from(command: CmdOpts) -> Self {
        if let CmdOpts::Single(arg) = command {
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
