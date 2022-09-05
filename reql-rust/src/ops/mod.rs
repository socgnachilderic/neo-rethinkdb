use regex::Regex;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::{cmd, Command, Func};

pub use document::ReqlOpsDocManipulation;
pub use sequence::ReqlOpsSequence;

mod document;
mod sequence;

pub trait ReqlOpsCommand:
    ReqlOps
    + ReqlOpsDocManipulation
    + ReqlOpsSequence<serde_json::Value>
    + ReqlOpsJoin<serde_json::Value>
    + ReqlOpsArray
    + ReqlOpsString
    + ReqlOpsGeometry
{}

pub trait ReqlOpsJoin<T: Unpin + Serialize + DeserializeOwned>: ReqlOpsSequence<T> {
    /// Used to ‘zip’ up the result of a join by merging the ‘right’ fields into ‘left’ fields of each member of the sequence.
    ///
    /// ## Example
    ///
    /// ‘zips up’ the sequence by merging the left and right fields produced by a join.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// use serde::{Serialize, Deserialize};
    /// use serde_json::Value;
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table::<Value>("marvel")
    ///         .eq_join(
    ///             "main_dc_collaborator",
    ///             &r.table::<Value>("dc"),
    ///         )
    ///         .zip()
    ///         .run(&session)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn zip(&self) -> cmd::zip::ZipBuilder {
        cmd::zip::ZipBuilder::new()._with_parent(self.get_parent())
    }
}

pub trait ReqlOpsGroupedStream<G, V>: ReqlOps
where
    G: Unpin + Serialize + DeserializeOwned,
    V: Unpin + Serialize + DeserializeOwned,
{
    /// Takes a grouped stream or grouped data and turns it into an array of objects representing the groups.
    /// Any commands chained after `ungroup` will operate on this array, rather than operating on each group individually.
    /// This is useful if you want to e.g. order the groups by the value of their reduction.
    ///
    /// The format of the array returned by `ungroup` is the same as the default native format
    /// of grouped data in the JavaScript driver and Data Explorer.
    ///
    /// ## Example
    ///
    /// Select users and all their posts.
    ///
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    ///
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("posts")
    ///         .group::<u8>(&["user_id"])
    ///         .ungroup()
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn ungroup(&self) -> cmd::ungroup::UngroupBuilder<G, V> {
        cmd::ungroup::UngroupBuilder::new()._with_parent(self.get_parent())
    }
}

pub trait ReqlOpsArray: ReqlOps {
    /// Insert a value in to an array at a given index. Returns the modified array.
    ///
    /// ## Example
    ///
    /// Hulk decides to join the avengers.
    ///
    /// ```ignore
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    ///
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.expr(&["Iron Man", "Spider-Man"])
    ///         .insert_at(1, "Hulk")
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn insert_at(&self, offset: usize, value: impl Serialize) -> cmd::insert_at::InsertAtBuilder {
        cmd::insert_at::InsertAtBuilder::new(offset, value)._with_parent(self.get_parent())
    }

    /// Insert several values in to an array at a given index. Returns the modified array.
    ///
    /// ## Example
    ///
    /// Hulk and Thor decide to join the avengers.
    ///
    /// ```ignore
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    ///
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.expr(&["Iron Man", "Spider-Man"])
    ///         .splice_at(1, &["Hulk", "Thor"])
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn splice_at(
        &self,
        offset: usize,
        values: &[impl Serialize],
    ) -> cmd::splice_at::SpliceAtBuilder {
        cmd::splice_at::SpliceAtBuilder::new(offset, values)._with_parent(self.get_parent())
    }

    /// Remove one or more elements from an array at a given index. Returns the modified array.
    ///
    /// ## Example
    ///
    /// Delete the second element of an array.
    ///
    /// ```ignore
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    ///
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r..expr(['a','b','c','d','e','f'])
    ///         .delete_at(1, None)
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn delete_at(
        &self,
        offset: isize,
        end_offset: Option<isize>,
    ) -> cmd::delete_at::DeleteAtBuilder {
        cmd::delete_at::DeleteAtBuilder::new(offset, end_offset)._with_parent(self.get_parent())
    }

    /// Change a value in an array at a given index. Returns the modified array.
    ///
    /// ## Example
    ///
    /// Bruce Banner hulks out.
    ///
    /// ```ignore
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    ///
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.expr(&["Iron Man", "Bruce", "Spider-Man"])
    ///         .change_at(1, "Hulk")
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn change_at(&self, offset: usize, value: impl Serialize) -> cmd::change_at::ChangeAtBuilder {
        cmd::change_at::ChangeAtBuilder::new(offset, value)._with_parent(self.get_parent())
    }
}

pub trait ReqlOpsString: ReqlOps {
    fn match_(&self, regex: Regex) -> cmd::match_::MatchBuilder {
        cmd::match_::MatchBuilder::new(regex)._with_parent(self.get_parent())
    }

    fn split(&self, separator: Option<&str>, max_splits: Option<&str>) -> cmd::split::SplitBuilder {
        cmd::split::SplitBuilder::new(separator, max_splits)._with_parent(self.get_parent())
    }

    fn upcase(&self) -> cmd::upcase::UpcaseBuilder {
        cmd::upcase::UpcaseBuilder::new()._with_parent(self.get_parent())
    }

    fn downcase(&self) -> cmd::downcase::DowncaseBuilder {
        cmd::downcase::DowncaseBuilder::new()._with_parent(self.get_parent())
    }
}

pub trait ReqlOpsObject<T>: ReqlOps {}

pub trait ReqlOpsGeometry: ReqlOps {
    /// Compute the distance between a point and another geometry object.
    /// At least one of the geometry objects specified must be a point.
    ///
    /// Optional methods available with `distance` are:
    ///
    /// - [with_geo_system(geo_system: reql_rust::types::GeoSystem)](crate::cmd::distance::DistanceBuilder::with_geo_system):
    /// the reference ellipsoid to use for geographic coordinates.
    /// Possible values are `GeoSystem::WGS84` (the default), a common standard for Earth’s geometry,
    /// or `GeoSystem::UnitSphere`, a perfect sphere of 1 meter radius.
    /// - [with_unit(unit: reql_rust::types::Unit)](crate::cmd::distance::DistanceBuilder::with_unit):
    /// Unit to return the distance in. Possible values are
    /// `Unit::Meter` (the default), `Unit::Kilometer`, `Unit::InternationalMile`, `Unit::NauticalMile`, `Unit::InternationalFoot`.
    ///
    /// If one of the objects is a polygon or a line, the point will be projected onto the line or polygon assuming
    /// a perfect sphere model before the distance is computed (using the model specified with geo_system).
    /// As a consequence, if the polygon or line is extremely large compared to Earth’s radius and
    /// the distance is being computed with the default WGS84 model, the results of distance should be considered
    /// approximate due to the deviation between the ellipsoid and spherical models.
    ///
    /// ## Example
    ///
    /// Compute the distance between two points on the Earth in kilometers.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// use reql_rust::types::Unit;
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let point1 = r.point(-122.423246, 37.779388);
    ///     let point2 = r.point(-117.220406, 32.719464);
    ///
    ///     let _ = point1.distance(point2)
    ///         .with_unit(Unit::Kilometer)
    ///         .run(&session)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn distance<A>(&self, geometry: A) -> cmd::distance::DistanceBuilder
    where
        A: ReqlOpsGeometry + Serialize,
    {
        cmd::distance::DistanceBuilder::new(geometry)._with_parent(self.get_parent())
    }

    /// Convert a ReQL geometry object to a [GeoJSON](https://geojson.org/) object.
    ///
    /// ## Example
    ///
    /// Compute the distance between two points on the Earth in kilometers.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// use reql_rust::types::{Unit, GeoJson};
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///
    ///     let _ = r.point(-122.423246, 37.779388)
    ///         .to_geojson::<GeoJson<[f64; 2]>>()
    ///         .run(&session)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn to_geojson<A>(&self) -> cmd::to_geojson::ToGeoJsonBuilder<A>
    where
        A: Unpin + Serialize + DeserializeOwned + Clone,
    {
        cmd::to_geojson::ToGeoJsonBuilder::new()._with_parent(self.get_parent())
    }

    fn includes<A>(&self, geometry: A) -> cmd::includes::IncludesBuilder<bool>
    where
        A: ReqlOpsGeometry + Serialize,
    {
        cmd::includes::IncludesBuilder::new(geometry)._with_parent(self.get_parent())
    }

    fn intersects<T>(&self, geometry: T) -> cmd::intersects::geometry::IntersectsBuilder
    where
        T: ReqlOpsGeometry + Serialize,
    {
        cmd::intersects::geometry::IntersectsBuilder::new(geometry)._with_parent(self.get_parent())
    }
}

pub trait ReqlOps {
    fn get_parent(&self) -> Command;

    /// Counts the number of elements in a sequence or key/value pairs in an object, or returns the size of a string or binary object.
    ///
    /// ## Example
    ///
    /// Count the number of users.
    ///
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    ///
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("table").count().run(&conn).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn count(&self) -> cmd::count::CountBuilder {
        cmd::count::CountBuilder::new()._with_parent(self.get_parent())
    }

    /// Counts the number of elements in a sequence or key/value pairs in an object, or returns the size of a string or binary object.
    ///
    /// It returns the number of elements in the sequence equal to that value or where the function returns `true` .
    /// On a binary object, `count` returns the size of the object in bytes; on strings, count returns the string’s length.
    /// This is determined by counting the number of Unicode codepoints in the string, counting combining codepoints separately.
    ///
    /// ## Example
    ///
    /// Count the number of 18 year old users.
    ///
    /// ```ignore
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    ///
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("table").bracket("age").count_by_value(18).run(&conn).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn count_by_value(&self, value: impl Serialize) -> cmd::count::CountBuilder {
        cmd::count::CountBuilder::new_by_value(value)._with_parent(self.get_parent())
    }

    /// Counts the number of elements in a sequence or key/value pairs in an object, or returns the size of a string or binary object.
    ///
    /// It returns the number of elements in the sequence equal to that value or where the function returns `true` .
    /// On a binary object, `count` returns the size of the object in bytes; on strings, count returns the string’s length.
    /// This is determined by counting the number of Unicode codepoints in the string, counting combining codepoints separately.
    ///
    /// ## Example
    ///
    /// Count the number of 18 year old users.
    ///
    /// ```ignore
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    ///
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("table").count_by_func(func!(|age| age.gt(18))).run(&conn).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn count_by_func(&self, func: Func) -> cmd::count::CountBuilder {
        cmd::count::CountBuilder::new_by_func(func)._with_parent(self.get_parent())
    }
}
