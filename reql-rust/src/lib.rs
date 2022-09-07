#![allow(clippy::wrong_self_convention)]

pub mod cmd;
pub mod connection;
mod constants;
mod err;
// mod ops;
pub mod prelude;
mod proto;
pub mod types;

// use prelude::ReqlOps;

use futures::Future;
pub use prelude::Func;
// use serde::{de::DeserializeOwned, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
// use time::{Date, Time, UtcOffset};
// use types::{DateTime, GeoJson, Line, Point, Polygon};

pub use connection::*;
pub use err::*;
pub use proto::Command;

#[doc(hidden)]
pub static VAR_COUNTER: AtomicU64 = AtomicU64::new(1);

#[doc(hidden)]
pub fn var_counter() -> u64 {
    VAR_COUNTER.fetch_add(1, Ordering::SeqCst)
}

// #[cfg(test)]
// fn current_counter() -> u64 {
//     VAR_COUNTER.load(Ordering::SeqCst)
// }

pub type Result<T> = std::result::Result<T, ReqlError>;

#[allow(non_camel_case_types)]
pub struct r;

impl r {
    pub fn connection(self) -> cmd::connect::ConnectionCommand {
        cmd::connect::ConnectionCommand::default()
    }

    pub fn db_create(self, db_name: &str) -> Command {
        cmd::db_create::make_db_create_command(db_name)
    }

    pub fn db_drop(self, db_name: &str) -> Command {
        cmd::db_drop::make_db_drop_command(db_name)
    }

    pub fn db_list(self) -> Command {
        cmd::db_list::make_db_list_command()
    }

    pub fn db(self, db_name: &str) -> Command {
        cmd::db::make_db_command(db_name)
    }

    
    /* pub fn table_create(self, table_name: &str) -> cmd::table_create::TableCreateCommand {
        cmd::table_create::TableCreateCommand::new(table_name)
    }

    pub fn table_drop(self, table_name: &str) -> cmd::table_drop::TableDropBuilder {
        cmd::table_drop::TableDropBuilder::new(table_name)
    }

    pub fn table_list(self) -> cmd::table_list::TableListBuilder {
        cmd::table_list::TableListBuilder::new()
    }

    pub fn table<T>(self, table_name: &str) -> cmd::table::TableBuilder<T>
    where
        T: Unpin + Serialize + DeserializeOwned,
    {
        cmd::table::TableBuilder::new(table_name)
    }

    pub fn map<A: Unpin + DeserializeOwned>(
        self,
        sequences: &[impl Serialize],
        func: Func,
    ) -> cmd::map::MapBuilder<A> {
        cmd::map::MapBuilder::new(func).with_sequences(sequences)
    }

    pub fn union<A, T>(self, sequence: &[&A]) -> cmd::union::UnionBuilder<T>
    where
        A: ReqlOps,
        T: Unpin + Serialize + DeserializeOwned,
    {
        cmd::union::UnionBuilder::new(sequence)
    }

    // pub fn literal(self, document: impl Serialize) -> String {
    //     cmd::literal::LiteralBuilder::new(document)
    // }

    // pub fn object(self, arg: impl cmd::object::Arg) -> Command {
    //     arg.arg().into_cmd()
    // }

    pub fn random(self, arg: impl cmd::random::Arg) -> Command {
        arg.arg().into_cmd()
    }

    pub fn round(self, arg: impl cmd::round::Arg) -> Command {
        arg.arg().into_cmd()
    }

    pub fn ceil(self, arg: impl cmd::ceil::Arg) -> Command {
        arg.arg().into_cmd()
    }

    pub fn floor(self, arg: impl cmd::floor::Arg) -> Command {
        arg.arg().into_cmd()
    }

    pub fn now(self) -> DateTime {
        DateTime::now()
    }

    pub fn time(self, date: Date, timezone: UtcOffset, time: Option<Time>) -> DateTime {
        DateTime::time(date, timezone, time)
    }

    pub fn epoch_time(self, timestamp: i64) -> crate::Result<DateTime> {
        DateTime::epoch_time(timestamp)
    }

    pub fn iso8601(
        self,
        iso_datetime: &str,
        default_timezone: Option<UtcOffset>,
    ) -> crate::Result<DateTime> {
        DateTime::iso8601(iso_datetime, default_timezone)
    }

    pub fn do_(self, func: Func) -> cmd::do_::DoBuilder {
        cmd::do_::DoBuilder::new(func)
    }

    pub fn branch(self, arg: impl cmd::branch::Arg) -> Command {
        arg.arg().into_cmd()
    }

    pub fn range(self, arg: impl cmd::range::Arg) -> Command {
        arg.arg().into_cmd()
    }

    pub fn error(self, arg: impl cmd::error::Arg) -> Command {
        arg.arg().into_cmd()
    }*/

    pub fn expr(self, arg: impl cmd::expr::Arg) -> Command {
        arg.arg().into_cmd()
    }

    /*pub fn js(self, arg: impl cmd::js::Arg) -> Command {
        arg.arg().into_cmd()
    }

    pub fn info(self, arg: impl cmd::info::Arg) -> Command {
        arg.arg().into_cmd()
    }

    pub fn json(self, arg: impl cmd::json::Arg) -> Command {
        arg.arg().into_cmd()
    }

    pub fn http(self, arg: impl cmd::http::Arg) -> Command {
        arg.arg().into_cmd()
    }

    pub fn uuid(self, arg: impl cmd::uuid::Arg) -> Command {
        arg.arg().into_cmd()
    }

    pub fn circle(self, point: &Point, radius: u32) -> cmd::circle::CircleBuilder<Polygon> {
        cmd::circle::CircleBuilder::new(point, radius)
    }

    pub fn circle_unfill(self, point: &Point, radius: u32) -> cmd::circle::CircleBuilder<Line> {
        cmd::circle::CircleBuilder::new(point, radius).with_fill(false)
    }

    pub fn geojson<T>(self, geojson: &GeoJson<T>) -> cmd::geojson::ReqlGeoJson<T>
    where
        T: Unpin + Serialize + DeserializeOwned + Clone,
    {
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

    pub fn grant(self, username: &str) -> cmd::grant::GrantBuilder {
        cmd::grant::GrantBuilder::new(username)
    }

    pub fn asc(self, arg: impl cmd::asc::Arg) -> cmd::asc::Asc {
        cmd::asc::Asc(arg.arg().into_cmd())
    }

    pub fn desc(self, arg: impl cmd::desc::Arg) -> cmd::desc::Desc {
        cmd::desc::Desc(arg.arg().into_cmd())
    }

    pub fn index(self, arg: impl cmd::index::Arg) -> cmd::index::Index {
        cmd::index::Index(arg.arg().into_cmd())
    } */

    pub fn args<T>(self, arg: T) -> cmd::args::Args<T> {
        cmd::args::Args(arg)
    }
}

// Helper for making writing examples less verbose
#[doc(hidden)]
pub async fn example<Q, F>(query: impl FnOnce(r, Session) -> F) -> Result<()>
where
    F: Future<Output = Result<()>>,
{
    query(r, r.connection().connect().await?).await
}
