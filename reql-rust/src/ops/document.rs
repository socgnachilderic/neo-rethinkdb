use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::cmd;

use super::ReqlOps;

pub trait ReqlOpsDocManipulation: ReqlOps {
    /// Plucks out one or more attributes from either an object or a sequence of objects (projection).
    /// 
    /// ## Example
    /// 
    /// We just need information about IronMan’s reactor and not the rest of the document.
    /// 
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// use serde_json::Value;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<Value>("marvel")
    ///         .get("IronMan")
    ///         .pluck::<_, Value>(["reactorState", "reactorPower"])
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// ## Example
    /// 
    /// For the hero beauty contest we only care about certain qualities.
    /// 
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// use serde_json::Value;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<Value>("marvel")
    ///         .pluck::<_, Value>(["beauty", "muscleTone", "charm"])
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// ## Example
    /// 
    /// Pluck can also be used on nested objects.
    /// 
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// use serde_json::{json, Value};
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<Value>("marvel")
    ///         .pluck::<_, Value>(json!({
    ///             "abilities": {
    ///                 "damage": true,
    ///                 "mana_cost": true
    ///             },
    ///             "weapons": true
    ///         }))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// ## Example
    /// 
    /// Pluck can also be used on nested objects.
    /// 
    /// ```ignore
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// use serde_json::{json, Value};
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<Value>("marvel")
    ///         .pluck::<_, Value>(
    ///             json!({ "abilities": [ "damage", "mana cost" ] }),
    ///             "weapons"
    ///         )
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn pluck<A, B>(&self, fields: A) -> cmd::pluck::PluckBuilder<B>
    where
        A: Serialize,
        B: Unpin + Serialize + DeserializeOwned,
    {
        cmd::pluck::PluckBuilder::new(fields)._with_parent(self.get_parent())
    }

    /// The opposite of pluck; takes an object or a sequence of objects, 
    /// and returns them with the specified fields or paths removed.
    /// 
    /// ## Example
    /// 
    /// Since we don’t need it for this computation we’ll save bandwidth and 
    /// leave out the list of IronMan’s romantic conquests.
    /// 
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// use serde_json::Value;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<Value>("marvel")
    ///         .get("IronMan")
    ///         .without::<_, Value>("personalVictoriesList")
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// ## Example
    /// 
    /// Without their prized weapons, our enemies will quickly be vanquished.
    /// 
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// use serde_json::Value;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<Value>("marvel")
    ///         .without::<_, Value>("weapons")
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// ## Example
    /// 
    /// Nested objects can be used to remove the damage subfield from the weapons and abilities fields.
    /// 
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// use serde_json::{json, Value};
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<Value>("marvel")
    ///         .without::<_, Value>(json!({
    ///             "abilities": {
    ///                 "damage": true
    ///             },
    ///             "weapons": {
    ///                 "damage": true
    ///             }
    ///         }))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// ## Example
    /// 
    /// The nested syntax can quickly become overly verbose so there’s a shorthand for it.
    /// 
    /// ```ignore
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// use serde_json::{json, Value};
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<Value>("marvel")
    ///         .without::<_, Value>(json!({ 
    ///             "weapons", "damage",
    ///             "abilities": "damage"
    ///         }))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn without<A, B>(&self, fields: A) -> cmd::without::WithoutBuilder<B>
    where
        A: Serialize,
        B: Unpin + Serialize + DeserializeOwned,
    {
        cmd::without::WithoutBuilder::new(fields)._with_parent(self.get_parent())
    }

    /// Append a value to an array.
    /// 
    /// ## Example
    /// 
    /// Retrieve Iron Man’s equipment list with the addition of some new boots.
    /// 
    /// ```ignore
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("marvel")
    ///         .get("ironman")
    ///         .bracket("opponents")
    ///         .append("newBoots")
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn append<A, B>(&self, value: A) -> cmd::append::AppendBuilder<B>
    where
        A: Serialize,
        B: Unpin + Serialize + DeserializeOwned,
    {
        cmd::append::AppendBuilder::new(value)._with_parent(self.get_parent())
    }

    /// Prepend a value to an array.
    /// 
    /// ## Example
    /// 
    /// Retrieve Iron Man’s equipment list with the addition of some new boots.
    /// 
    /// ```ignore
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("marvel")
    ///         .get("ironman")
    ///         .bracket("opponents")
    ///         .prepend("newBoots")
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn prepend<A, B>(&self, value: A) -> cmd::prepend::PrependBuilder<B>
    where
        A: Serialize,
        B: Unpin + Serialize + DeserializeOwned,
    {
        cmd::prepend::PrependBuilder::new(value)._with_parent(self.get_parent())
    }

    /// Remove the elements of one array from another array.
    /// 
    /// ## Example
    /// 
    /// Retrieve Iron Man’s equipment list without boots.
    /// 
    /// ```ignore
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("marvel")
    ///         .get("IronMan")
    ///         .bracket("equipment")
    ///         .prepend("Boots")
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn difference<A, B>(&self, values: &[A]) -> cmd::difference::DifferenceBuilder<B>
    where
        A: Serialize,
        B: Unpin + Serialize + DeserializeOwned,
    {
        cmd::difference::DifferenceBuilder::new(values)._with_parent(self.get_parent())
    }

    /// Add a value to an array and return it as a set (an array with distinct values).
    /// 
    /// ## Example
    /// 
    /// Retrieve Iron Man’s equipment list with the addition of some new boots.
    /// 
    /// ```ignore
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("marvel")
    ///         .get("IronMan")
    ///         .bracket("equipment")
    ///         .setInsert("newBoots")
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn set_insert<A, B>(&self, value: A) -> cmd::set_insert::SetInsertBuilder<B>
    where
        A: Serialize,
        B: Unpin + Serialize + DeserializeOwned,
    {
        cmd::set_insert::SetInsertBuilder::new(value)._with_parent(self.get_parent())
    }

    /// Perform a set intersection of two arrays, returning an array with all unique items from both.
    /// 
    /// ## Example
    /// 
    /// Retrieve Iron Man’s equipment list with the addition of some new boots and an arc reactor.
    /// 
    /// ```ignore
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("marvel")
    ///         .get("IronMan")
    ///         .bracket("equipment")
    ///         .set_union(&["newBoots", "arc_reactor"])
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn set_union<A, B>(&self, values: &[A]) -> cmd::set_union::SetUnionBuilder<B>
    where
        A: Serialize,
        B: Unpin + Serialize + DeserializeOwned,
    {
        cmd::set_union::SetUnionBuilder::new(values)._with_parent(self.get_parent())
    }

    /// Intersect two arrays returning values that occur in both of them as a set (an array with distinct values).
    /// 
    /// ## Example
    /// 
    /// Check which pieces of equipment Iron Man has from a fixed list.
    /// 
    /// ```ignore
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("marvel")
    ///         .get("IronMan")
    ///         .bracket("equipment")
    ///         .set_intersection(&["newBoots", "arc_reactor"])
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn set_intersection<A, B>(&self, values: &[A]) -> cmd::set_intersection::SetIntersectionBuilder<B>
    where
        A: Serialize,
        B: Unpin + Serialize + DeserializeOwned,
    {
        cmd::set_intersection::SetIntersectionBuilder::new(values)._with_parent(self.get_parent())
    }

    /// Remove the elements of one array from another and return them as a set (an array with distinct values).
    /// 
    /// ## Example
    /// 
    /// Check which pieces of equipment Iron Man has, excluding a fixed list.
    /// 
    /// ```ignore
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("marvel")
    ///         .get("IronMan")
    ///         .bracket("equipment")
    ///         .set_intersection(&["newBoots", "arc_reactor"])
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn set_difference<A, B>(&self, values: &[A]) -> cmd::set_difference::SetDifferenceBuilder<B>
    where
        A: Serialize,
        B: Unpin + Serialize + DeserializeOwned,
    {
        cmd::set_difference::SetDifferenceBuilder::new(values)._with_parent(self.get_parent())
    }

    /// Get a single field from an object. If called on a sequence, 
    /// gets that field from every object in the sequence, skipping objects that lack it.
    /// 
    /// ```text
    /// Under most circumstances, you’ll want to use getField (or its shorthand g) or nth rather than bracket. 
    /// The bracket term may be useful in situations where you are unsure of the data type returned by the term you are calling bracket on.
    /// ```
    /// 
    /// ## Example
    /// 
    /// Check which pieces of equipment Iron Man has, excluding a fixed list.
    /// 
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("marvel")
    ///         .get("IronMan")
    ///         .bracket("firstAppearance")
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn bracket(&self, attr: impl Serialize) -> cmd::bracket::BracketBuilder {
        cmd::bracket::BracketBuilder::new(attr)._with_parent(self.get_parent())
    }

    /// Get a single field from an object. If called on a sequence, 
    /// gets that field from every object in the sequence, skipping objects that lack it.
    /// 
    /// ## Example
    /// 
    /// What was Iron Man’s first appearance in a comic?
    /// 
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("marvel")
    ///         .get("IronMan")
    ///         .get_field("firstAppearance")
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn get_field(&self, field: &str) -> cmd::get_field::GetFieldBuilder {
        cmd::get_field::GetFieldBuilder::new(field)._with_parent(self.get_parent())
    }

    fn has_fields(&self, fields: impl Serialize) -> cmd::has_fields::HasFieldsBuilder {
        cmd::has_fields::HasFieldsBuilder::new(fields)._with_parent(self.get_parent())
    }

}
