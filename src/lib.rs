use std::collections::HashMap;

use serde::Serialize;

use arguments::Permission;
pub use connection::*;
pub use err::*;
pub use proto::Command;
use time::{Date, Time, UtcOffset};
use types::{Binary, DateTime, GeoJson};

pub type Result<T> = std::result::Result<T, ReqlError>;

mod constants;
mod err;
mod proto;
#[cfg(test)]
mod spec;

pub mod arguments;
pub mod cmd;
pub mod connection;
pub mod prelude;
pub mod types;

#[macro_export]
macro_rules! args {
    ( $($a:expr),* ) => {{ $crate::arguments::Args(($($a),*)) }};
}

// TODO Put Clone Copy in all derive macro as possible

#[allow(non_camel_case_types)]
pub struct r;

impl r {
    pub fn connection(self) -> cmd::connect::ConnectionCommand {
        cmd::connect::ConnectionCommand::default()
    }

    pub fn db_create(self, db_name: &str) -> Command {
        cmd::db_create::new(db_name)
    }

    pub fn db_drop(self, db_name: &str) -> Command {
        cmd::db_drop::new(db_name)
    }

    pub fn db_list(self) -> Command {
        cmd::db_list::new()
    }

    pub fn db(self, db_name: &str) -> Command {
        cmd::db::new(db_name)
    }

    pub fn table_create(self, args: impl cmd::table_create::TableCreateArg) -> Command {
        cmd::table_create::new(args)
    }

    pub fn table_drop(self, table_name: &str) -> Command {
        cmd::table_drop::new(table_name)
    }

    pub fn table_list(self) -> Command {
        cmd::table_list::new()
    }

    pub fn table(self, args: impl cmd::table::TableArg) -> Command {
        cmd::table::new(args)
    }

    pub fn map(self, args: impl cmd::map::MapArg) -> Command {
        cmd::map::new(args)
    }

    pub fn order_by(self, args: impl cmd::order_by::OrderByArg) -> Command {
        cmd::order_by::new(args)
    }

    pub fn union(self, args: impl cmd::union::UnionArg) -> Command {
        cmd::union::new(args)
    }

    pub fn reduce(self, args: impl cmd::reduce::ReduceArg) -> Command {
        cmd::reduce::new(args)
    }

    pub fn count(self, args: impl cmd::count::CountArg) -> Command {
        cmd::count::new(args)
    }

    pub fn sum(self, args: impl cmd::sum::SumArg) -> Command {
        cmd::sum::new(args)
    }

    pub fn avg(self, args: impl cmd::avg::AvgArg) -> Command {
        cmd::avg::new(args)
    }

    pub fn min(self, args: impl cmd::min::MinArg) -> Command {
        cmd::min::new(args)
    }

    pub fn max(self, args: impl cmd::max::MaxArg) -> Command {
        cmd::max::new(args)
    }

    pub fn distinct(self, args: impl cmd::distinct::DistinctArg) -> Command {
        cmd::distinct::new(args)
    }

    pub fn contains(self, args: impl cmd::contains::ContainsArg) -> Command {
        cmd::contains::new(args)
    }

    pub fn literal(self, value: impl Serialize) -> Command {
        cmd::literal::new(value)
    }

    pub fn object(self, values: Vec<impl Serialize>) -> Command {
        cmd::object::new(values)
    }

    pub fn and(self, args: impl cmd::and::AndArg) -> Command {
        cmd::and::new(args)
    }

    pub fn or(self, args: impl cmd::or::OrArg) -> Command {
        cmd::or::new(args)
    }

    pub fn eq(self, args: impl cmd::eq::EqArg) -> Command {
        cmd::eq::new(args)
    }

    pub fn ne(self, args: impl cmd::ne::NeArg) -> Command {
        cmd::ne::new(args)
    }

    pub fn gt(self, args: impl cmd::gt::GtArg) -> Command {
        cmd::gt::new(args)
    }

    pub fn ge(self, args: impl cmd::ge::GeArg) -> Command {
        cmd::ge::new(args)
    }

    pub fn lt(self, args: impl cmd::lt::LtArg) -> Command {
        cmd::lt::new(args)
    }

    pub fn le(self, args: impl cmd::le::LeArg) -> Command {
        cmd::le::new(args)
    }

    pub fn not(self, value: bool) -> Command {
        cmd::not_::new(value)
    }

    pub fn random(self, args: impl cmd::random::RandomArg) -> Command {
        cmd::random::new(args)
    }

    pub fn round(self, value: f64) -> Command {
        cmd::round::new(value)
    }

    pub fn ceil(self, value: f64) -> Command {
        cmd::ceil::new(value)
    }

    pub fn floor(self, value: f64) -> Command {
        cmd::floor::new(value)
    }

    pub fn now(self) -> DateTime {
        DateTime::now()
    }

    pub fn time(self, date: Date, timezone: UtcOffset, time: Option<Time>) -> DateTime {
        DateTime::time(date, timezone, time)
    }

    pub fn epoch_time(self, timestamp: i64) -> Result<DateTime> {
        DateTime::epoch_time(timestamp)
    }

    //
    pub fn iso8601(
        self,
        iso_datetime: &str,
        default_timezone: Option<UtcOffset>,
    ) -> crate::Result<DateTime> {
        DateTime::iso8601(iso_datetime, default_timezone)
    }

    // TODO Review Date and Times Commands

    pub fn hash_map<T>(self, value: HashMap<T, Command>) -> Command
    where
        T: Into<String>,
    {
        cmd::hash_map::new(value)
    }

    pub fn args(self, values: Vec<impl Serialize>) -> Command {
        cmd::args::new(values)
    }

    pub fn binary(self, data: &[u8]) -> Binary {
        cmd::binary::new(data)
    }

    pub fn do_(self, args: impl cmd::do_::DoArg) -> Command {
        cmd::do_::new(args)
    }

    pub fn branch(self, test: Command, args: impl cmd::branch::BranchArg) -> Command {
        test.branch(args)
    }

    pub fn range(self, args: impl cmd::range::RangeArg) -> Command {
        cmd::range::new(args)
    }

    pub fn error(self, message: &str) -> Command {
        cmd::error::new(message)
    }

    pub fn expr(self, value: impl Serialize) -> Command {
        cmd::expr::new(value)
    }

    pub fn js(self, args: impl cmd::js::JsArg) -> Command {
        cmd::js::new(args)
    }

    pub fn info(self, any: Command) -> Command {
        any.info()
    }

    pub fn json(self, value: &str) -> Command {
        cmd::json::new(value)
    }

    pub fn http(self, args: impl cmd::http::HttpArg) -> Command {
        cmd::http::new(args)
    }

    pub fn uuid(self, args: impl cmd::uuid::UuidArg) -> Command {
        cmd::uuid::new(args)
    }

    pub fn circle(self, args: impl cmd::circle::CircleArg) -> Command {
        cmd::circle::new(args)
    }

    pub fn distance(self, args: impl cmd::distance::DistanceArg) -> Command {
        cmd::distance::new(args)
    }

    pub fn geojson<T: Serialize>(self, geojson: GeoJson<T>) -> cmd::geojson::ReqlGeoJson<T> {
        cmd::geojson::ReqlGeoJson::new(geojson)
    }

    pub fn line(self, points: &[cmd::point::Point]) -> cmd::line::Line {
        cmd::line::Line::new(points)
    }

    pub fn point(self, longitude: f64, latitude: f64) -> cmd::point::Point {
        cmd::point::Point::new(longitude, latitude)
    }

    pub fn polygon(self, points: &[cmd::point::Point]) -> cmd::polygon::Polygon {
        cmd::polygon::Polygon::new(points)
    }

    pub fn intersects(self, args: impl cmd::intersects::IntersectsArg) -> Command {
        cmd::intersects::new(args)
    }

    pub fn grant(self, username: &str, permission: Permission) -> Command {
        cmd::grant::new(username, permission)
    }

    pub fn wait(self, args: impl cmd::wait::WaitArg) -> Command {
        cmd::wait::new(args)
    }

    pub fn asc(self, args: impl cmd::asc::AscArg) -> Command {
        cmd::asc::new(args)
    }

    pub fn desc(self, args: impl cmd::desc::DescArg) -> Command {
        cmd::desc::new(args)
    }

    pub fn var(self, value: impl Serialize) -> Command {
        Command::from_json(value)
    }

    /// max_val are used with some commands such as `between`
    /// to specify absolute upper bounds.
    ///
    /// # Command syntax
    /// ```text
    /// r::min_val()
    /// ```
    ///
    /// ## Example
    ///
    /// ```
    /// use reql_rust::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("simbad")
    ///         .between(args!(r::min_val(), r.var(20)))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn min_val() -> Command {
        Command::new(ql2::term::TermType::Minval)
    }

    /// max_val are used with some commands such as `between`
    /// to specify absolute upper bounds.
    ///
    /// # Command syntax
    /// ```text
    /// r::max_val()
    /// ```
    ///
    /// ## Example
    ///
    /// ```
    /// use reql_rust::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("simbad")
    ///         .between(args!(r.var(10), r::max_val()))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn max_val() -> Command {
        Command::new(ql2::term::TermType::Maxval)
    }
}

// Helper for making writing examples less verbose
#[doc(hidden)]
pub fn example<'a, Q, F, S>(_query: Q)
where
    Q: FnOnce(r, &'a mut Session) -> async_stream::AsyncStream<(), F>,
    F: futures::Future<Output = S>,
    S: futures::Stream<Item = Result<serde_json::Value>>,
{
}
