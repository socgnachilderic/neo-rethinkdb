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
pub mod hash_map;
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
// pub mod minutes;
pub mod month;
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
pub mod time_of_day;
pub mod timezone;
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
pub mod year;
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
use ::time::UtcOffset;

use crate::arguments::{AnyParam, Permission};
use crate::prelude::Func;
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

    pub fn get(self, args: impl get::GetArg) -> Self {
        get::new(args).with_parent(self)
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

    pub fn in_timezone(self, timezone: UtcOffset) -> Self {
        in_timezone::new(timezone).with_parent(self)
    }

    pub fn timezone(self) -> Self {
        timezone::new().with_parent(self)
    }

    pub fn during(self, args: impl during::DuringArg) -> Self {
        during::new(args).with_parent(self)
    }

    pub fn date(self) -> Self {
        date::new().with_parent(self)
    }

    pub fn time_of_day(self) -> Self {
        time_of_day::new().with_parent(self)
    }

    pub fn year(self) -> Self {
        year::new().with_parent(self)
    }

    pub fn month(self) -> Self {
        month::new().with_parent(self)
    }

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

    // FIXME Command no work
    pub fn do_(self, args: impl do_::DoArg) -> Self {
        do_::new(args).with_parent(self)
    }

    /// Perform a branching conditional equivalent to `if-then-else`.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.branch(test, args!(true_action, false_action)) → any
    /// r.branch(test, args!(true_action, [(test2, test2_action), N], false_action)) → any
    /// query.branch(args!(true_action, false_action)) -> any
    /// query.branch(args!(true_action, [(test2, test2_action), N], false_action)) → any
    /// ```
    ///
    /// Where:
    /// - test, true_action, false_action, test2, test2_action: r.expr(...)
    ///
    /// # Description
    ///
    /// The `branch` command takes 2n+1 arguments: pairs of conditional expressions
    /// and commands to be executed if the conditionals return any value but `false`
    /// or `None` i.e., “truthy” values), with a final “else” command to be evaluated
    /// if all of the conditionals are `false` or `None`.
    ///
    /// You may call `branch` infix style on the first test.
    /// (See the second example for an illustration.)
    ///
    /// ```text
    /// r.branch(test1, args!(val1, [(test2, val2)], elseval))
    /// ```
    ///
    /// is the equivalent of the Rust statement
    ///
    /// ```text
    /// if (test1) {
    ///     val1
    /// } else if (test2) {
    ///     val2
    /// } else {
    ///     elseval
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Test the value of x.
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let x = 10;
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: String = r.branch(
    ///             r.expr(x > 5),
    ///             args!(r.expr("big"), r.expr("small"))
    ///         ).run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.eq("big"));
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// As above, infix-style.
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let x = 10;
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: String = r.expr(x > 5)
    ///         .branch(args!(r.expr("big"), r.expr("small")))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.eq("big"));
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Categorize heroes by victory counts.
    ///
    /// ```
    /// use std::ops::Add;
    ///
    /// use reql_rust::prelude::*;
    /// use reql_rust::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response = r.table("pricings")
    ///         .map(func!(|offer| r.branch(
    ///             offer.clone().g("price").gt(100),
    ///             args!(
    ///                 offer.clone().g("offer").add("premium"),
    ///                 [(
    ///                     offer.clone().g("price").gt(10),
    ///                     offer.clone().g("offer").add("standard")
    ///                 )],
    ///                 offer.g("offer").add("freemium")
    ///         ))))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [do_](Self::do_)
    pub fn branch(self, args: impl branch::BranchArg) -> Self {
        branch::new(args).with_parent(self)
    }

    /// Loop over a sequence, evaluating the given write query for each element.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// sequence.for_each(write_function) → response
    /// ```
    ///
    /// Where:
    /// - write_function: func!(...)
    /// - response: [MutationResponse](crate::types::MutationResponse)
    ///
    /// ## Examples
    ///
    /// Get information about a table such as primary key, or cache size.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::types::MutationResponse;
    /// use reql_rust::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: MutationResponse = r.table("models")
    ///         .for_each(func!(|model| r.table("cars")
    ///             .get(model.get("car_model"))
    ///             .delete(())
    ///         ))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.deleted == 5);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [map](Self::map)
    pub fn for_each(self, write_function: Func) -> Self {
        for_each::new(write_function).with_parent(self)
    }

    /// Provide a default value in case of non-existence errors.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// value.default(default_value) → any
    /// sequence.default(default_value) → any
    /// ```
    ///
    /// # Description
    ///
    /// The `default` command evaluates its first argument (the value it’s chained to).
    /// If that argument returns `None` or a non-existence error is thrown in evaluation,
    /// then `default` returns its second argument. The second argument is usually a default value,
    /// but it can be a function that returns a value.
    ///
    /// ## Examples
    ///
    /// Suppose we want to retrieve the titles and authors of the table posts.
    /// In the case where the author field is missing or null,
    /// we want to retrieve the string Anonymous.
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Debug, Serialize, Deserialize)]
    /// struct Post {
    ///     title: String,
    ///     author: String,
    /// }
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: Vec<Post> = r.table("posts")
    ///         .map(func!(|doc| {
    ///             let mut post = HashMap::new();
    ///             post.insert("title", doc.clone().g("title"));
    ///             post.insert("author", doc.clone().g("author").default("Anonymous"));
    ///             r.hash_map(post)
    ///         }))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.len() > 0);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// The `default` command can also be used to filter documents.
    /// Suppose we want to retrieve all our users who are not grown-ups or
    /// whose age is unknown (i.e., the field `age` is missing or equals `None`).
    /// We can do it with this query:
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response = r.table("users")
    ///         .filter(func!(|user| user.g("age").lt(18).default(true)))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// One more way to write the previous query is
    /// to set the age to be `-1` when the field is missing.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response = r.table("users")
    ///         .filter(func!(|user| user.g("age").default(-1).lt(18)))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// Another way to do the same query is to use hasFields.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response = r.table("users")
    ///         .filter(func!(|user| user.clone().has_fields("age").not_().or(user.g("age").lt(18))))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// Another way to do the same query is to use hasFields.
    ///
    /// ```
    /// use reql_rust::cmd::filter::FilterOption;
    /// use reql_rust::prelude::*;
    /// use reql_rust::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response = r.table("users")
    ///         .filter(args!(
    ///             func!(|user| user.g("age").lt(18).default(true)),
    ///             FilterOption::default().default_(true)
    ///         ))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn default(self, default_value: impl Serialize) -> Self {
        default::new(default_value).with_parent(self)
    }

    /// Convert a value of one type into another.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// sequence.coerce_to('array') → array
    /// value.coerce_to('string') → string
    /// string.coerce_to('number') → number
    /// array.coerce_to('object') → object
    /// sequence.coerce_to('object') → object
    /// object.coerce_to('array') → array
    /// binary.coerce_to('string') → string
    /// string.coerce_to('binary') → binary
    /// ```
    ///
    /// # Description
    ///
    /// - a sequence, selection or object can be coerced to an array
    /// - a sequence, selection or an array of key-value pairs can be coerced to an object
    /// - a string can be coerced to a number
    /// - any datum (single value) can be coerced to a string
    /// - a binary object can be coerced to a string and vice-versa
    ///
    /// ## Examples
    ///
    /// Coerce an array of pairs into an object.
    ///
    /// ```
    /// use reql_rust::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response = r.expr([["name", "Malika"], ["genre", "woman"]])
    ///         .coerce_to("object")
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Coerce a number to a string.
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: char = r.expr(1)
    ///         .coerce_to("string")
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == '1');
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [object](crate::r::object)
    pub fn coerce_to(self, value: impl Serialize) -> Self {
        coerce_to::new(value).with_parent(self)
    }

    /// Gets the type of a ReQL query’s return value.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// any.type_of() → response
    /// ```
    ///
    /// Where:
    /// - response: [TypeOf](crate::types::TypeOf)
    ///
    /// # Description
    ///
    /// Read the article on [ReQL data types](https://rethinkdb.com/docs/data-types/)
    /// for a more detailed discussion.
    /// Note that some possible return values from `type_of` are internal values,
    /// such as `TypeOf::MAXVAL`, and unlikely to be returned from queries in standard practice.
    ///
    /// ## Examples
    ///
    /// Get the type of a TypeOf.
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::types::TypeOf;
    /// use reql_rust::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: TypeOf = r.expr("foo")
    ///         .type_of()
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == TypeOf::String);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn type_of(self) -> Self {
        type_of::new().with_parent(self)
    }

    /// Get information about a ReQL value.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// any.info() → response
    /// r.info(any) → response
    /// ```
    ///
    /// Where:
    /// - response: [InfoResponse](crate::types::InfoResponse)
    ///
    /// ## Examples
    ///
    /// Get information about a table such as primary key, or cache size.
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::types::{InfoResponse, TypeOf};
    /// use reql_rust::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: InfoResponse = r.table("simbad")
    ///         .info()
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.typ == TypeOf::Table);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn info(self) -> Self {
        info::new().with_parent(self)
    }

    /// Convert a ReQL value or object to a JSON string.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// value.to_json() → String
    /// ```
    ///
    /// ## Examples
    ///
    /// Get a ReQL document as a JSON string.
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: String = r.table("simbad")
    ///         .get(1)
    ///         .to_json()
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(!response.is_empty());
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn to_json(self) -> Self {
        to_json::new().with_parent(self)
    }

    /// Compute the distance between a point and another geometry object.
    /// At least one of the geometry objects specified must be a point.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// geometry.distance(geometry) → f64
    /// geometry.distance(args!(geometry, options)) → f64
    /// r.distance(geometry.cmd(), geometry) → f64
    /// r.distance(geometry.cmd(), args!(geometry, options)) → f64
    /// ```
    ///
    /// Where:
    /// - geometry: [r.point(...)](crate::r::point) |
    /// [r.line(...)](crate::r::line) |
    /// [r.polygon(...)](crate::r::polygon)
    /// - options: [DistanceOption](crate::cmd::distance::DistanceOption)
    ///
    /// # Description
    ///
    /// If one of the objects is a polygon or a line, the point will be projected
    /// into the line or polygon assuming a perfect sphere model before the distance
    /// is computed (using the model specified with `geo_system`).
    /// As a consequence, if the polygon or line is extremely large compared
    /// to Earth’s radius and the distance is being computed with the default
    ///  WGS84 model, the results of `distance` should be considered approximate
    /// due to the deviation between the ellipsoid and spherical models.
    ///
    /// ## Examples
    ///
    /// Compute the distance between two points on the Earth in kilometers.
    ///
    /// ```
    /// use reql_rust::arguments::Unit;
    /// use reql_rust::cmd::distance::DistanceOption;
    /// use reql_rust::prelude::*;
    /// use reql_rust::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let point1 = r.point(-122.423246, 37.779388);
    ///     let point2 = r.point(-117.220406, 32.719464);
    ///     let distance_option = DistanceOption::default().unit(Unit::Kilometer);
    ///
    ///     let response: f64 = point1.cmd()
    ///         .distance(point2)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == 734.125249602186);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [polygon](crate::r::polygon)
    /// - [line](crate::r::line)
    pub fn distance(self, args: impl distance::DistanceArg) -> Self {
        distance::new(args).with_parent(self)
    }

    /// Convert a ReQL geometry object to a [GeoJSON](https://geojson.org/) object.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// geometry.to_geojson() → response
    /// ```
    ///
    /// Where:
    /// - geometry: [r.point(...)](crate::r::point) |
    /// [r.line(...)](crate::r::line) |
    /// [r.polygon(...)](crate::r::polygon)
    /// command
    /// - response: [GeoJson<T>](crate::types::GeoJson)
    ///
    /// ## Examples
    ///
    /// Convert a ReQL geometry object to a GeoJSON object.
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::types::{GeoJson, GeoType};
    /// use reql_rust::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: GeoJson<[f64; 2]> = r.table("simbad")
    ///         .get("sfo")
    ///         .g("location")
    ///         .to_geojson()
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.typ == GeoType::Point);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [geojson](crate::r::geojson)
    pub fn to_geojson(self) -> Self {
        to_geojson::new().with_parent(self)
    }

    /// Get all documents where the given geometry object intersects
    /// the geometry object of the requested geospatial index.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// table.get_intersecting(geometry, options) → selection<stream>
    /// ```
    ///
    /// Where:
    /// - geometry: [r.point(...)](crate::r::point) |
    /// [r.line(...)](crate::r::line) |
    /// [r.polygon(...)](crate::r::polygon)
    /// command
    /// - sequence: command
    /// - options: [GetIntersectingOption](crate::cmd::get_intersecting::GetIntersectingOption)
    ///
    /// # Description
    ///
    /// The `index` argument is mandatory. This command returns the same
    ///  results as `|row| row.g(index).intersects(geometry)`.
    /// The total number of results is limited to the array size limit
    /// which defaults to 100,000, but can be changed with the `array_limit`
    /// option to [run](Self::run).
    ///
    /// ## Examples
    ///
    /// Which of the locations in a list of parks intersect `circle`?
    ///
    /// ```
    /// use reql_rust::arguments::Unit;
    /// use reql_rust::cmd::circle::CircleOption;
    /// use reql_rust::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let point = r.point(-117.220406, 32.719464);
    ///     let circle_opts = CircleOption::default()
    ///         .unit(Unit::InternationalMile);
    ///     let circle = r.circle(args!(point, 10., circle_opts));
    ///
    ///     let response = r.table("simbad")
    ///         .get_intersecting(circle, "area")
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [get_nearest](Self::get_nearest)
    pub fn get_intersecting(
        self,
        geometry: impl get_intersecting::GetIntersectingArg,
        index: &'static str,
    ) -> Self {
        get_intersecting::new(geometry, index).with_parent(self)
    }

    /// Return a list of documents closest to a
    /// specified point based on a geospatial index,
    /// sorted in order of increasing distance.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// table.get_nearest(args!(geometry, &str)) → array
    /// table.get_nearest(args!(geometry, &str, options)) → array
    /// ```
    ///
    /// Where:
    /// - geometry: [r.point(...)](crate::r::point) |
    /// [r.line(...)](crate::r::line) |
    /// [r.polygon(...)](crate::r::polygon) |
    /// command
    /// - sequence: command
    /// - options: [GetNearestOption](crate::cmd::get_nearest::GetNearestOption)
    ///
    /// # Description
    ///
    /// The return value will be an array of two-item objects
    /// with the keys `dist` and `doc`, set to the distance
    /// between the specified point and the document
    /// (in the units specified with `unit`, defaulting to meters)
    /// and the document itself, respectively.
    /// The array will be sorted by the values of `dist`.
    ///
    /// ## Examples
    ///
    /// Return a list of the closest 25 enemy hideouts to the secret base.
    ///
    /// ```
    /// use reql_rust::cmd::get_nearest::GetNearestOption;
    /// use reql_rust::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let secret_base = r.point(-122.422876, 37.777128);
    ///     let opts = GetNearestOption::default().max_results(25);
    ///
    ///     let response = r.table("simbad")
    ///         .get_nearest(args!(secret_base, "location"))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [get_intersecting](Self::get_intersecting)
    pub fn get_nearest(self, args: impl get_nearest::GetNearestArg) -> Self {
        get_nearest::new(args).with_parent(self)
    }

    /// Tests whether a geometry object is completely contained within another.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// geometry.includes(geometry) → bool
    /// sequence.includes(geometry) → sequence
    /// ```
    ///
    /// Where:
    /// - geometry: [r.point(...)](crate::r::point) |
    /// [r.line(...)](crate::r::line) |
    /// [r.polygon(...)](crate::r::polygon) |
    /// command
    /// - sequence: command
    ///
    /// # Description
    ///
    /// When applied to a sequence of geometry objects,
    /// `includes` acts as a [filter](Self::filter),
    /// returning a sequence of objects from the sequence
    /// that include the argument.
    ///
    /// ## Examples
    ///
    /// Is `point2` included within a 2000-meter circle around `point1`?
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let point1 = r.point(-117.220406, 32.719464);
    ///     let point2 = r.point(-117.206201, 32.725186);
    ///
    ///     let response: bool = r.circle(args!(point1, 2000.))
    ///         .includes(point2)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Which of the locations in a list of parks include `circle`?
    ///
    /// ```
    /// use reql_rust::arguments::Unit;
    /// use reql_rust::cmd::circle::CircleOption;
    /// use reql_rust::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let point = r.point(-117.220406, 32.719464);
    ///     let circle_opts = CircleOption::default().unit(Unit::InternationalMile);
    ///     let circle = r.circle(args!(point, 10., circle_opts));
    ///
    ///     let response = r.table("parks")
    ///         .g("area")
    ///         .includes(circle)
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Rewrite the previous example with `get_intersecting`.
    ///
    /// ```
    /// use reql_rust::arguments::Unit;
    /// use reql_rust::cmd::circle::CircleOption;
    /// use reql_rust::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let point = r.point(-117.220406, 32.719464);
    ///     let circle_opts = CircleOption::default()
    ///         .unit(Unit::InternationalMile);
    ///     let circle = r.circle(args!(point, 10., circle_opts));
    ///
    ///     let response = r.table("parks")
    ///         .get_intersecting(circle.clone(), "area")
    ///         .g("area")
    ///         .includes(circle)
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [intersects](Self::intersects)
    pub fn includes(self, args: impl includes::IncludesArg) -> Self {
        includes::new(args).with_parent(self)
    }

    /// Tests whether two geometry objects intersect with one another.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// geometry.intersects(geometry) → bool
    /// r.intersects(geometry_command, geometry) → bool
    /// sequence.intersects(geometry) → sequence_response
    /// r.intersects(sequence, geometry) → sequence_response
    /// ```
    ///
    /// Where:
    /// - geometry: [r.point(...)](crate::r::point) |
    /// [r.line(...)](crate::r::line) |
    /// [r.polygon(...)](crate::r::polygon) |
    /// command
    /// - sequence, geometry_command: [Command](crate::Command)
    ///
    /// # Description
    ///
    /// When applied to a sequence of geometry objects, `intersects` acts as a
    /// [filter](Self::filter), returning a sequence of objects from
    /// the sequence that intersect with the argument.
    ///
    /// ## Examples
    ///
    /// Is `point2` within a 2000-meter circle around `point1`?
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let point1 = r.point(-117.220406, 32.719464);
    ///     let point2 = r.point(-117.206201, 32.725186);
    ///
    ///     let response: bool = r.circle(args!(point1, 2000.))
    ///         .intersects(point2)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Which of the locations in a list of parks intersect `circle`?
    ///
    /// ```
    /// use reql_rust::arguments::Unit;
    /// use reql_rust::cmd::circle::CircleOption;
    /// use reql_rust::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let point = r.point(-117.220406, 32.719464);
    ///     let circle_opts = CircleOption::default().unit(Unit::InternationalMile);
    ///     let circle = r.circle(args!(point, 10., circle_opts));
    ///
    ///     let response = r.table("parks")
    ///         .g("area")
    ///         .intersects(circle)
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [includes](Self::includes)
    /// - [get_intersecting](Self::get_intersecting)
    pub fn intersects(self, geometry: impl intersects::IntersectsArg) -> Self {
        intersects::new(geometry).with_parent(self)
    }

    /// Grant or deny access permissions for a user account,
    /// globally or on a per-database or per-table basis.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.grant(username, permission) → response
    /// table.grant(username, permission) → response
    /// db.grant(username, permission) → response
    /// ```
    ///
    /// Where:
    /// - table: [r.table(...)](crate::r::table) |
    /// [query.table(...)](Self::table)
    /// - db: [r.db(...)](crate::r::db)
    /// - response: [GrantResponse](crate::types::GrantResponse)
    ///
    /// # Description
    ///
    /// Permissions that are not defined on a local scope will
    /// be inherited from the next largest scope.
    /// For example, a write operation on a table will first
    /// check if `write` permissions are explicitly set to `true` or `false`
    /// for that table and account combination; if they are not,
    /// the `write` permissions for the database will be used
    /// if those are explicitly set; and if neither table nor database
    /// permissions are set for that account, the global `write`
    /// permissions for that account will be used.
    ///
    /// ## Note
    ///
    /// For all accounts other than the special, system-defined `admin` account,
    /// permissions that are not explicitly set in any scope will effectively be `false`.
    /// When you create a new user account by inserting a record into the
    /// [system table](https://rethinkdb.com/docs/system-tables/#users),
    /// that account will have **no** permissions until they are explicitly granted.
    ///
    /// For a full description of permissions, read
    /// [Permissions and user accounts](https://rethinkdb.com/docs/permissions-and-accounts/).
    ///
    /// ## Examples
    ///
    /// Grant the `alima` user account read and write permissions on the `users` database.
    ///
    /// ```
    /// use reql_rust::arguments::Permission;
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::types::GrantResponse;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let permission = Permission::default().read(true).write(true);
    ///
    ///     let response: GrantResponse = r.db("users")
    ///         .grant("alima", permission)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.granted == 1);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Deny write permissions from the `alima` account for the `simbad` table.
    ///
    /// ```
    /// use reql_rust::arguments::Permission;
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::types::GrantResponse;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let permission = Permission::default().write(false);
    ///
    ///     let response: GrantResponse = r.db("users")
    ///         .table("simbad")
    ///         .grant("alima", permission)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.granted == 1);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Grant `alima` the ability to use HTTP connections.
    ///
    /// ```
    /// use reql_rust::arguments::Permission;
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::types::GrantResponse;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let permission = Permission::default().connect(true);
    ///
    ///     let response: GrantResponse = r.grant("alima", permission)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.granted == 1);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Grant a `monitor` account read-only access to all databases.
    ///
    /// ```
    /// use reql_rust::arguments::Permission;
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::types::GrantResponse;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let permission = Permission::default().read(true);
    ///
    ///     let response: GrantResponse = r.grant("monitor", permission)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.granted == 1);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn grant(self, username: &str, permission: Permission) -> Self {
        grant::new(username, permission).with_parent(self)
    }

    /// Query (read and/or update) the configurations for individual tables or databases.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// table.config() → response
    /// db.config() → response
    /// ```
    ///
    /// Where:
    /// - table: [r.table(...)](crate::r::table) |
    /// [query.table(...)](Self::table)
    /// - db: [r.db(...)](crate::r::db)
    /// - response: [ConfigResponse](crate::types::ConfigResponse)
    ///
    /// # Description
    ///
    /// The config command is a shorthand way to access the `table_config` or `db_config`
    /// [System tables](https://rethinkdb.com/docs/system-tables/#configuration-tables).
    /// It will return the single row from the system that corresponds to the database
    /// or table configuration, as if [get](Self::get) had been called on the system
    /// table with the UUID of the database or table in question.
    ///
    /// ## Examples
    ///
    /// Rebalance a table.
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::types::ConfigResponse;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: ConfigResponse = r.table("simbad")
    ///         .config()
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.name == "simbad");
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn config(self) -> Self {
        config::new().with_parent(self)
    }

    /// Rebalances the shards of a table. When called on a database,
    /// all the tables in that database will be rebalanced.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// table.rebalance() → response
    /// db.rebalance() → response
    /// ```
    ///
    /// Where:
    /// - table: [r.table(...)](crate::r::table) |
    /// [query.table(...)](Self::table)
    /// - db: [r.db(...)](crate::r::db)
    /// - response: [RebalanceResponse](crate::types::RebalanceResponse)
    ///
    /// # Description
    ///
    /// The `rebalance` command operates by measuring the distribution of
    /// primary keys within a table and picking split points that will
    /// give each shard approximately the same number of documents.
    /// It won’t change the number of shards within a table,
    /// or change any other configuration aspect for the table or the database.
    ///
    /// A table will lose availability temporarily after `rebalance` is called;
    /// use the [wait](Self::wait) command to wait for the table to become available again,
    /// or [status](Self::status) to check if the table is available for writing.
    ///
    /// RethinkDB automatically rebalances tables when the number of shards are increased,
    /// and as long as your documents have evenly distributed primary keys—such as
    /// the default UUIDs—it is rarely necessary to call `rebalance` manually.
    /// Cases where `rebalance` may need to be called include:
    /// - Tables with unevenly distributed primary keys, such as incrementing integers
    /// - Changing a table’s primary key type
    /// - Increasing the number of shards on an empty table,
    /// then using non-UUID primary keys in that table
    ///
    /// The [web UI](https://rethinkdb.com/docs/administration-tools/)
    /// (and the [info](Self::info) command)
    /// can be used to tell you when a table’s shards need to be rebalanced.
    ///
    /// See the [status](Self::status) command for an explanation of
    /// the objects returned in the `old_val` and `new_val` fields.
    ///
    /// ## Examples
    ///
    /// Rebalance a table.
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::types::RebalanceResponse;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: RebalanceResponse = r.table("simbad")
    ///         .rebalance()
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.rebalanced == 1);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn rebalance(self) -> Self {
        rebalance::new().with_parent(self)
    }

    /// Reconfigure a table’s sharding and replication.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// table.reconfigure(options) → response
    /// db.reconfigure(options) → response
    /// ```
    ///
    /// Where:
    /// - table: [r.table(...)](crate::r::table) |
    /// [query.table(...)](Self::table)
    /// - db: [r.db(...)](crate::r::db)
    /// - options: [ReconfigureOption](crate::cmd::reconfigure::ReconfigureOption)
    /// - response: [ReconfigureResponse](crate::types::ReconfigureResponse)
    ///
    /// # Description
    ///
    /// A table will lose availability temporarily after `reconfigure` is called;
    /// use the [wait](Self::wait) command to wait for the table to become available again,
    /// or [status](Self::wait) to check if the table is available for writing.
    ///
    /// ## Note
    ///
    /// Whenever you call `reconfigure`, the write durability will be set to
    /// `Durability::Hard` and the write
    /// acknowledgments will be set to `ReadMode::Majority`;
    /// these can be changed by using the `config` command on the table.
    ///
    ///
    /// If `reconfigure` is called on a database,
    /// all the tables in the database will have their configurations affected.
    /// The return value will be an array of the objects described above, one per table.
    ///
    /// Read [Sharding and replication](https://rethinkdb.com/docs/sharding-and-replication/)
    /// for a complete discussion of the subject, including advanced topics.
    ///
    /// ## Examples
    ///
    /// Reconfigure a table.
    ///
    /// ```
    /// use reql_rust::arguments::Replicas;
    /// use reql_rust::cmd::reconfigure::ReconfigureOption;
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::types::ReconfigureResponse;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let opts = ReconfigureOption::default()
    ///         .shards(1)
    ///         .replicas(Replicas::Int(1));
    ///
    ///     let response: ReconfigureResponse = r.table("simbad")
    ///         .reconfigure(opts)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.reconfigured == 1);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Reconfigure a table, specifying replicas by server tags.
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// use reql_rust::arguments::Replicas;
    /// use reql_rust::cmd::reconfigure::ReconfigureOption;
    /// use reql_rust::prelude::*;
    /// use reql_rust::types::ReconfigureResponse;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let mut replicas = HashMap::new();
    ///
    ///     replicas.insert("malika".static_string(), 1);
    ///     replicas.insert("malika".static_string(), 1);
    ///
    ///     let opts = ReconfigureOption::default()
    ///         .shards(2)
    ///         .replicas(Replicas::Map {
    ///             replicas,
    ///             primary_replica_tag: "malika".static_string()
    ///         });
    ///
    ///     let response: ReconfigureResponse = r.table("simbad")
    ///         .reconfigure(opts)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.reconfigured == 1);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Perform an emergency repair on a table.
    ///
    /// ```
    /// use reql_rust::arguments::EmergencyRepair;
    /// use reql_rust::cmd::reconfigure::ReconfigureOption;
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::types::ReconfigureResponse;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let opts = ReconfigureOption::default()
    ///         .emergency_repair(EmergencyRepair::UnsafeRollback);
    ///
    ///     let response: ReconfigureResponse = r.table("simbad")
    ///         .reconfigure(opts)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.reconfigured == 1);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn reconfigure(self, opts: reconfigure::ReconfigureOption) -> Self {
        reconfigure::new(opts).with_parent(self)
    }

    /// Return the status of a table.
    ///
    /// The return value is an object providing information about
    /// the table’s shards, replicas and replica readiness states.
    /// For a more complete discussion of the object fields,
    /// read about the table_status table in
    /// [System tables](https://rethinkdb.com/docs/system-tables/#status-tables).
    ///
    /// # Command syntax
    ///
    /// ```text
    /// table.status() → response
    /// ```
    ///
    /// Where:
    /// - table: [r.table(...)](crate::r::table) |
    /// [query.table(...)](Self::table)
    /// - response: [StatusResponse](crate::types::StatusResponse)
    ///
    /// ## Examples
    ///
    /// Get a table’s status.
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::types::StatusResponse;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: StatusResponse = r.table("simbad")
    ///         .status()
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.name.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn status(self) -> Self {
        status::new().with_parent(self)
    }

    /// Wait for a table or all the tables in a database to be ready.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// table.wait(()) → response
    /// db.wait(()) → response
    /// r.wait(table) → response
    /// r.wait(database) → response
    /// table.wait(options) → response
    /// db.wait(options) → response
    /// r.wait(args!(table, options)) → response
    /// r.wait(args!(database, options)) → response
    /// ```
    ///
    /// Where:
    /// - table: [r.table(...)](crate::r::table) |
    /// [query.table(...)](Self::table)
    /// - db: [r.db(...)](crate::r::db)
    /// - options: [WaitOption](crate::cmd::wait::WaitOption)
    /// - response: [WaitResponse](crate::types::WaitResponse)
    ///
    /// # Description
    ///
    /// A table may be temporarily unavailable after creation,
    /// rebalancing or reconfiguring.
    /// The `wait` command blocks until the given
    /// table (or database) is fully up to date.
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
    /// # Description
    ///
    /// This method has the same parameters as `run`.
    /// The main difference between `make_query` and `run` is that
    /// `make_query` can be used to execute multiple requests
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
