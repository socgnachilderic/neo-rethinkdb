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

    /// Retrieve data from the specified URL over HTTP.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.http(string) → value
    /// r.http(args!(string, options)) → value
    /// ```
    ///
    /// Where:
    /// - string: impl Into<String>
    /// - options: impl Serialize
    ///
    /// # Description
    ///
    /// The return type depends on the `result_format` option,
    /// which checks the `Content-Type` of the response by default.
    ///
    /// See [External API access](https://rethinkdb.com/docs/external-api-access/)
    /// for more informations
    ///
    /// ## Examples
    ///
    /// Perform an HTTP GET and store the result in a table.
    ///
    /// ```
    /// use reql_rust::types::MutationResponse;
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: MutationResponse = r.table("simbad")
    ///         .insert(r.http("http://httpbin.org/get"))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.inserted == 1);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Perform request with parameters.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{args, r, Result};
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response = r.http(args!("http://httpbin.org/get", json!({
    ///             "params": {
    ///                 "user": 1
    ///             }
    ///         })))
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
    /// Perform a `POST` request with accompanying data.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{args, r, Result};
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response = r.http(args!("http://httpbin.org/get", json!({
    ///             "method": "method",
    ///             "data": {
    ///                 "player": "Moussa",
    ///                 "game": "AURION"
    ///             }
    ///         })))
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
    /// Perform a GitHub search and collect up to 3 pages of results.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{args, r, Result};
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response = r.http(args!("https://api.github.com/search/code?q=addClass+user:mozilla", json!({
    ///             "page": "link-next",
    ///             "page_limit": 3
    ///         })))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn http<T>(self, args: impl cmd::http::HttpArg<T>) -> Command
    where
        T: Serialize,
    {
        cmd::http::new(args)
    }

    /// Return a UUID (universally unique identifier),
    /// a string that can be used as a unique ID.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.uuid(()) → String
    /// r.uuid(&str) → String
    /// ```
    ///
    /// # Description
    ///
    /// If a string is passed to uuid as an argument,
    /// the UUID will be deterministic,
    /// derived from the string’s SHA-1 hash.
    ///
    /// RethinkDB’s UUIDs are standards-compliant.
    /// Without the optional argument,
    /// a version 4 random UUID will be generated;
    /// with that argument, a version 5 UUID will be generated,
    /// using a fixed namespace UUID of `91461c99-f89d-49d2-af96-d8e2e14e9b58`.
    /// For more information, read
    /// [Wikipedia’s UUID article](https://en.wikipedia.org/wiki/Universally_unique_identifier).
    ///
    /// ## Examples
    ///
    /// Generate a UUID.
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::{args, r, Result};
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: String = r.uuid(())
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.eq("27961a0e-f4e8-4eb3-bf95-c5203e1d87b9"));
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Generate a UUID based on a String.
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::{args, r, Result};
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: String = r.uuid("malik@example.com")
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.eq("3461d115-2c05-5af4-9906-9f6882c58a15"));
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn uuid(self, args: impl cmd::uuid::UuidArg) -> Command {
        cmd::uuid::new(args)
    }

    /// Construct a circular line or polygon.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.circle(args!(point, radius)) → polygon
    /// r.circle(args!(point, radius, options)) → polygon
    /// ```
    ///
    /// Where:
    /// - radius: f64,
    /// - point: [Point](crate::cmd::point::Point)
    /// - polygon: [Polygon](crate::cmd::polygon::Polygon)
    ///
    /// # Description
    ///
    /// A circle in RethinkDB is a polygon or line **approximating**
    /// a circle of a given radius around a given center,
    /// consisting of a specified number of vertices (default 32).
    ///
    /// The center may be specified either by two floating point numbers, the longitude
    /// (−180 to 180) and latitude (−90 to 90) of the point on a perfect sphere
    /// (See [Geospatial support](https://rethinkdb.com/docs/geo-support/python/)
    /// for more information on ReQL’s coordinate system), or by a point object.
    /// The radius is a floating point number whose units are meters by default,
    /// although that may be changed with the `unit` argument.
    ///
    /// ## Examples
    ///
    /// Define a point.
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::types::Polygon;
    /// use reql_rust::{args, r, Result};
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let circle: Polygon = r.circle(args!(r.point(-122.423246, 37.779388), 50.5))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     let response = r.table("geo")
    ///         .insert(json!({
    ///             "id": 300,
    ///             "name": "Douala",
    ///             "location": circle
    ///         }))
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
    /// - [line](Self::line)
    /// - [polygon](Self::polygon)
    /// - [point](Self::point)
    /// - [distance](crate::Command::distance)
    pub fn circle(self, args: impl cmd::circle::CircleArg) -> Command {
        cmd::circle::new(args)
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
    /// command
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
    ///     let response: f64 = r.distance(point1.cmd(), point2)
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
    pub fn distance(self, geometry: Command, args: impl cmd::distance::DistanceArg) -> Command {
        geometry.distance(args)
    }

    /// Convert a [GeoJSON](https://geojson.org/) object to a ReQL geometry object.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.geojson(geojson) → geometry
    /// ```
    ///
    /// Where:
    /// - geojson: [GeoJson](crate::types::GeoJson),
    /// - geometry: [ReqlGeoJson](crate::types::ReqlGeoJson)
    ///
    /// # Description
    ///
    /// RethinkDB only allows conversion of GeoJSON objects
    /// which have ReQL equivalents: Point, LineString, and Polygon.
    ///  MultiPoint, MultiLineString, and MultiPolygon are not supported.
    /// (You could, however, store multiple points, lines and polygons
    /// in an array and use a geospatial multi index with them.)
    ///
    /// Only longitude/latitude coordinates are supported.
    /// GeoJSON objects that use Cartesian coordinates,
    /// specify an altitude, or specify their own coordinate
    /// reference system will be rejected.
    ///
    /// ## Examples
    ///
    /// Convert a GeoJSON object to a ReQL geometry object.
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::types::{GeoJson, GeoType};
    /// use reql_rust::{args, r, Result};
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let geo_json = GeoJson {
    ///         typ: GeoType::Point,
    ///         coordinates: [-122.423246, 37.779388],
    ///     };
    ///
    ///     let response = r.table("geo")
    ///         .insert(json!({
    ///             "id": 1,
    ///             "name": "Yaoundé",
    ///             "location": r.geojson(geo_json)
    ///         }))
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
    /// - [to_geojson](crate::Command::to_geojson)
    pub fn geojson<T: Serialize>(self, geojson: GeoJson<T>) -> cmd::geojson::ReqlGeoJson<T> {
        cmd::geojson::ReqlGeoJson::new(geojson)
    }

    /// Construct a geometry object of type Polygon.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.line(points) → line
    /// ```
    ///
    /// Where:
    /// - points: &[[Point](crate::types::Point)]
    /// - line: [Polygon](crate::types::Line)
    ///
    /// # Description
    ///
    /// The line can be specified in one of two ways:
    /// - Two or more two-item arrays, specifying latitude
    /// and longitude numbers of the line’s vertices;
    /// - Two or more [Point](crate::types::Point)
    /// objects specifying the line’s vertices.
    ///
    /// Longitude (−180 to 180) and latitude (−90 to 90)
    /// of vertices are plotted on a perfect sphere.
    /// See [Geospatial support](https://rethinkdb.com/docs/geo-support/python/)
    /// for more information on ReQL’s coordinate system.
    ///
    /// ## Examples
    ///
    /// Define a line.
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::{args, r, Result};
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response = r.table("geo")
    ///         .insert(json!({
    ///             "id": 101,
    ///             "route": r.line(&[
    ///                 r.point(-122.423246, 37.779388),
    ///                 r.point(-121.886420, 37.329898),
    ///             ])
    ///         }))
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
    /// - [point](Self::point)
    /// - [polygon](Self::polygon)
    /// - [circle](Self::circle)
    pub fn line(self, points: &[cmd::point::Point]) -> cmd::line::Line {
        cmd::line::Line::new(points)
    }

    /// Construct a geometry object of type Point.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.point(longitude, latitude) → point
    /// ```
    ///
    /// Where:
    /// - latitude: f64,
    /// - longitude: f64,
    /// - points: &[[Point](crate::cmd::point::Point)]
    ///
    /// # Description
    ///
    /// The point is specified by two floating point numbers, the longitude
    /// (−180 to 180) and latitude (−90 to 90) of the point on a perfect sphere.
    /// See [Geospatial support](https://rethinkdb.com/docs/geo-support/python/)
    /// for more information on ReQL’s coordinate system.
    ///
    /// ## Examples
    ///
    /// Define a point.
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::{args, r, Result};
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response = r.table("geo")
    ///         .insert(json!({
    ///             "id": 1,
    ///             "name": "Yaoundé",
    ///             "location": r.point(-122.423246, 37.779388)
    ///         }))
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
    /// - [line](Self::line)
    /// - [polygon](Self::polygon)
    /// - [circle](Self::circle)
    pub fn point(self, longitude: f64, latitude: f64) -> cmd::point::Point {
        cmd::point::Point::new(longitude, latitude)
    }

    /// Construct a geometry object of type Polygon.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.polygon(points) → polygon
    /// ```
    ///
    /// Where:
    /// - points: &[[Point](crate::cmd::point::Point)]
    /// - polygon: [Polygon](crate::cmd::polygon::Polygon)
    ///
    /// # Description
    ///
    /// The Polygon can be specified in one of two ways:
    /// - Three or more two-item arrays, specifying latitude
    /// and longitude numbers of the polygon’s vertices;
    /// - Three or more [Point](crate::cmd::point::Point)
    /// objects specifying the polygon’s vertices.
    ///
    /// Longitude (−180 to 180) and latitude (−90 to 90)
    /// of vertices are plotted on a perfect sphere.
    /// See [Geospatial support](https://rethinkdb.com/docs/geo-support/python/)
    /// for more information on ReQL’s coordinate system.
    ///
    /// If the last point does not specify the same coordinates as
    /// the first point, `polygon` will close the polygon by connecting them.
    /// You cannot directly construct a polygon with holes in it using `polygon`,
    /// but you can use [polygon_sub](crate::types::Polygon::polygon_sub)
    /// to use a second polygon within the interior of the first to define a hole.
    ///
    /// ## Examples
    ///
    /// Define a polygon.
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::{args, r, Result};
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response = r.table("geo")
    ///         .insert(json!({
    ///             "id": 101,
    ///             "rectangle": r.polygon(&[
    ///                 r.point(-122.423246, 37.779388),
    ///                 r.point(-122.423246, 37.329898),
    ///                 r.point(-121.886420, 37.329898),
    ///                 r.point(-121.886420, 37.779388),
    ///             ])
    ///         }))
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
    /// - [point](Self::point)
    /// - [line](Self::line)
    /// - [circle](Self::circle)
    pub fn polygon(self, points: &[cmd::point::Point]) -> cmd::polygon::Polygon {
        cmd::polygon::Polygon::new(points)
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
    /// [filter](crate::Command::filter), returning a sequence of objects from
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
    ///     let circle_cmd = r.circle(args!(point1, 2000.));
    ///
    ///     let response: bool = r.intersects(circle_cmd, point2)
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
    ///     let response = r.intersects(r.table("parks").g("area"), circle)
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
    /// - [includes](crate::Command::includes)
    /// - [get_intersecting](crate::Command::get_intersecting)
    pub fn intersects(
        self,
        geometry: Command,
        args: impl cmd::intersects::IntersectsArg,
    ) -> Command {
        geometry.intersects(args)
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
    pub fn grant(self, username: &str, permission: Permission) -> Command {
        cmd::grant::new(username, permission)
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
    pub fn wait(self, args: impl cmd::wait::WaitArg) -> Command {
        cmd::wait::new(args)
    }

    /// To specify the descending ordering.
    ///
    /// # Command syntax
    /// ```text
    /// r.asc(field)
    /// r.asc(func)
    /// ```
    ///
    /// Where:
    /// - field: String, &str
    /// - func: func!(...)
    ///
    /// ## Example
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("simbad")
    ///         .order_by(args!(r.expr("id"), r.asc("character")))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn asc(self, args: impl cmd::asc::AscArg) -> Command {
        cmd::asc::new(args)
    }

    /// To specify the descending ordering.
    ///
    /// # Command syntax
    /// ```text
    /// r.desc(field)
    /// r.desc(func)
    /// ```
    ///
    /// Where:
    /// - field: String, &str
    /// - func: func!(...)
    ///
    /// ## Example
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("simbad")
    ///         .order_by(args!(r.expr("id"), r.desc("character")))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn desc(self, args: impl cmd::desc::DescArg) -> Command {
        cmd::desc::new(args)
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
    ///         .between(args!(r::min_val(), r.expr(20)))
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
    ///         .between(args!(r.expr(10), r::max_val()))
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
