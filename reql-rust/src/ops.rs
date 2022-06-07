use regex::Regex;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::Command;
use crate::Func;

use crate::cmd;
use crate::cmd::table::TableBuilder;
use crate::types::Document;
use crate::types::Sequence;
use crate::types::WritingResponseType;

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

pub trait ReqlOpsSequence<T: Unpin + Serialize + DeserializeOwned>: ReqlOpsDocManipulation {
    /// Turn a query into a changefeed, an infinite stream of objects
    /// representing changes to the query’s results as they occur.
    /// A changefeed may return changes to a table or an individual document (a “point” changefeed).
    /// Commands such as filter or map may be used before the changes command to transform or filter the output,
    /// and many commands that operate on sequences can be chained after changes.
    fn changes(&self) -> cmd::changes::ChangesBuilder<Document<T>> {
        cmd::changes::ChangesBuilder::new()._with_parent(self.get_parent())
    }

    /// Update JSON documents in a table. Accepts a JSON document, 
    /// a ReQL expression, or a combination of the two.
    /// 
    /// You can use the following method to setting query:
    /// 
    /// * [with_durability(durability: reql_rust::types::Durability)](cmd::update::UpdateBuilder::with_durability)
    /// possible values are `Durability::Hard` and `Durability::Soft`. This option will override the table or query’s durability setting (set in [run](cmd::run)). 
    /// In soft durability mode RethinkDB will acknowledge the write immediately after receiving it, but before the write has been committed to disk.
    /// * [with_return_changes(return_changes: reql_rust::types::ReturnChanges)](cmd::update::UpdateBuilder::with_return_changes) :
    ///     - `ReturnChanges::Bool(true)` : return a `changes` array consisting of `old_val`/`new_val` objects describing the changes made, 
    ///         only including the documents actually updated.
    ///     - `ReturnChanges::Bool(false)` : do not return a `changes` array (the default).
    ///     - `ReturnChanges::Always"` : behave as `ReturnChanges::Bool(true)`, 
    ///         but include all documents the command tried to update whether or not the update was successful.
    /// * [with_non_atomic(non_atomic: bool)](cmd::update::UpdateBuilder::with_non_atomic)
    /// if set to `true`, executes the update and distributes the result to replicas in a non-atomic fashion. 
    /// This flag is required to perform non-deterministic updates, such as those that require
    /// * [with_ignore_write_hook(ignore_write_hook: bool)](cmd::update::UpdateBuilder::with_ignore_write_hook)
    /// If `true`, and if the user has the config permission, ignores any [write hook](cmd::set_write_hook::SetWriteHookBuilder) when performing the update.
    /// 
    /// Update returns a struct [WritingResponseType](crate::types::WritingResponseType):
    /// 
    /// ## Example
    /// 
    /// Update the status of all posts to published.
    /// 
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     let updated_data = json!({ "status": "published" });
    ///     
    ///     r.table("heroes").insert(&[updated_data]).run(&conn).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn update(&self, document: impl Serialize) -> cmd::update::UpdateBuilder<WritingResponseType<Document<T>>> {
        cmd::update::UpdateBuilder::new(document)._with_parent(self.get_parent())
    }
    
    /// Update JSON documents in a table. Accepts a JSON document, 
    /// a ReQL expression, or a combination of the two.
    /// 
    /// See [update](#method.update) for more information
    /// 
    /// ## Example
    /// 
    /// Remove the field `status` from all posts.
    /// 
    /// ```ignore
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    ///
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table("heroes")
    ///         .update_by_func(func!(|post| post.without("status")))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn update_by_func(&self, func: Func) -> cmd::update::UpdateBuilder<WritingResponseType<Document<T>>> {
        cmd::update::UpdateBuilder::new_by_func(func)._with_parent(self.get_parent())
    }

    /// Replace documents in a table. Accepts a JSON document or a ReQL expression, 
    /// and replaces the original document with the new one. 
    /// The new document must have the same primary key as the original document.
    /// 
    /// The `replace` command can be used to both insert and delete documents. 
    /// If the `“replaced”` document has a primary key that doesn’t exist in the table, 
    /// the document will be inserted; if an existing document is replaced with `None`, 
    /// the document will be deleted. Since `update`, `replace` and `replace_by_func` operations are performed atomically, 
    /// this allows atomic inserts and deletes as well.
    /// 
    /// See [update](#method.update) for more informations to setting
    /// 
    /// Replace returns a struct [WritingResponseType](crate::types::WritingResponseType):
    /// 
    /// ## Example
    /// 
    /// Remove the field `status` from all posts.
    /// 
    /// ```ignore
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table("heroes")
    ///         .replace(&json!({ "id": 1; "status": "published"; }))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn replace(&self, document: impl Serialize) -> cmd::replace::ReplaceBuilder<WritingResponseType<Document<T>>> {
        cmd::replace::ReplaceBuilder::new(document)._with_parent(self.get_parent())
    }

    /// Replace documents in a table. Accepts a JSON document or a ReQL expression, 
    /// and replaces the original document with the new one. 
    /// 
    /// See [replace](#method.replace) for more information
    /// 
    /// ## Example
    /// 
    /// Remove the field `status` from all posts.
    /// 
    /// ```ignore
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    ///
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table("heroes")
    ///         .replace_by_func(func!(|post| post.without("status")))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn replace_by_func(&self, func: Func) -> cmd::replace::ReplaceBuilder<WritingResponseType<Document<T>>> {
        cmd::replace::ReplaceBuilder::new_by_func(func)._with_parent(self.get_parent())
    }

    /// Delete one or more documents from a table.
    /// 
    /// You can use the following method to setting query:
    /// 
    /// * [with_durability(durability: reql_rust::types::Durability)](cmd::update::UpdateBuilder::with_durability)
    /// possible values are `Durability::Hard` and `Durability::Soft`. This option will override the table or query’s durability setting (set in [run](cmd::run)). 
    /// In soft durability mode RethinkDB will acknowledge the write immediately after receiving it, but before the write has been committed to disk.
    /// * [with_return_changes(return_changes: reql_rust::types::ReturnChanges)](cmd::update::UpdateBuilder::with_return_changes) :
    ///     - `ReturnChanges::Bool(true)` : return a `changes` array consisting of `old_val`/`new_val` objects describing the changes made, 
    ///         only including the documents actually updated.
    ///     - `ReturnChanges::Bool(false)` : do not return a `changes` array (the default).
    ///     - `ReturnChanges::Always"` : behave as `ReturnChanges::Bool(true)`, 
    ///         but include all documents the command tried to update whether or not the update was successful.
    /// * [with_ignore_write_hook(ignore_write_hook: bool)](cmd::update::UpdateBuilder::with_ignore_write_hook)
    /// If `true`, and if the user has the config permission, ignores any [write hook](cmd::set_write_hook::SetWriteHookBuilder), 
    /// which might have prohibited the deletion.
    /// 
    /// ## Example
    /// 
    /// Delete a single document from the table `heroes` .
    /// 
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// use serde::{Serialize, Deserialize};
    /// 
    /// #[derive(Serialize, Deserialize)]
    /// struct Heroes {
    ///     id: String,
    ///     name: String,
    /// }
    ///
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<Heroes>("heroes").delete().run(&conn).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn delete(&self) -> cmd::delete::DeleteBuilder<WritingResponseType<Document<T>>> {
        cmd::delete::DeleteBuilder::new()._with_parent(self.get_parent())
    }

    /// Return all the elements in a sequence for which the given predicate is true.
    /// The return value of `filter` will be the same as the input (sequence, stream, or array).
    /// Documents can be filtered in a variety of ways—ranges, nested values, boolean conditions,
    /// and the results of anonymous functions.
    fn filter(&self, func: Func) -> cmd::filter::FilterBuilder<Sequence<T>> {
        cmd::filter::FilterBuilder::new(func)._with_parent(self.get_parent())
    }
    
    /// Returns an inner join of two sequences.
    ///
    /// The returned sequence represents an intersection of the left-hand sequence and the right-hand sequence:
    /// each row of the left-hand sequence will be compared with
    /// each row of the right-hand sequence  to find all pairs of rows which satisfy the predicate.
    /// Each matched pair of rows of both sequences are combined  into a result row.
    /// In most cases, you will want to follow the join with [zip](self::ReqlOpsJoin::zip) to combine the left and right results.
    ///
    /// ```text
    /// Note that inner_join is slower and much less efficient than using eq_join or concat_map with get_all.
    /// You should avoid using inner_join in commands when possible.
    /// ```
    ///
    /// ## Example
    ///
    /// Return a list of all matchups between Marvel and DC heroes in which the DC hero could beat the Marvel hero in a fight.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Serialize, Deserialize)]
    /// struct Users {
    ///     id: u8,
    ///     full_name: String,
    /// }
    ///
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct Posts {
    ///     id: u8,
    ///     title: String,
    ///     content: String,
    ///     user_id: u8,
    /// }
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table::<Posts>("posts")
    ///         .inner_join(
    ///             &r.table::<Users>("users"),
    ///             func!(|post, _user| post.bracket("user_id").eq(1)),
    ///         )
    ///         .run(&session)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn inner_join<A: Unpin + Serialize + DeserializeOwned>(
        &self,
        other_table: &TableBuilder<A>,
        func: Func,
    ) -> cmd::inner_join::InnerJoinBuilder<A, T> {
        cmd::inner_join::InnerJoinBuilder::new(other_table, func)._with_parent(self.get_parent())
    }

    /// Returns a left outer join of two sequences.
    /// The returned sequence represents a union of the left-hand sequence and the right-hand sequence:
    /// all documents in the left-hand sequence will be returned,
    /// each matched with a document in the right-hand sequence if one satisfies the predicate condition.
    /// In most cases, you will want to follow the join with [zip](self::ReqlOpsJoin::zip) to combine the left and right results.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Serialize, Deserialize)]
    /// struct Users {
    ///     id: u8,
    ///     full_name: String,
    /// }
    ///
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct Posts {
    ///     id: u8,
    ///     title: String,
    ///     content: String,
    ///     user_id: u8,
    /// }
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table::<Posts>("posts")
    ///         .outer_join(
    ///             &r.table::<Users>("users"),
    ///             func!(|post, _user| post.bracket("user_id").eq(1)),
    ///         )
    ///         .run(&session)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn outer_join<A: Unpin + Serialize + DeserializeOwned>(
        &self,
        other_table: &TableBuilder<A>,
        func: Func,
    ) -> cmd::outer_join::OuterJoinBuilder<A, T> {
        cmd::outer_join::OuterJoinBuilder::new(other_table, func)._with_parent(self.get_parent())
    }

    /// Join tables using a field or function on the left-hand sequence matching primary keys or secondary indexes on the right-hand table. 
    /// `eq_join` is more efficient than other ReQL join types, and operates much faster. 
    /// Documents in the result set consist of pairs of left-hand and right-hand documents, 
    /// matched when the field on the left-hand side exists and is non-null and an entry 
    /// with that field’s value exists in the specified index on the right-hand side.
    /// 
    /// The result set of `eq_join` is a stream or array of objects. 
    /// Each object in the returned set will be an object of the form { "left": <left-document>, "right": <right-document> }, 
    /// where the values of left and right will be the joined documents. 
    /// Use the [zip](self::ReqlOpsJoin::zip) command to merge the left and right fields together.
    /// 
    /// The results from `eq_join` are, by default, not ordered. Providing [with_ordered(true)](cmd::eq_join::EqJoinBuilder::with_ordered) 
    /// will cause `eq_join` to order the output based on the left side input stream. 
    /// (If there are multiple matches on the right side for a document on the left side, 
    /// their order is not guaranteed even if ordered is true.) Requiring ordered results can significantly slow down `eq_join`, 
    /// and in many circumstances this ordering will not be required. 
    /// (See the first example, in which ordered results are obtained by using `order_by` after `eq_join`.)
    /// 
    /// ## Example
    /// 
    /// Match posts with the users they’ve posted against one another.
    /// 
    /// Join these tables using `user_id` on the users table and `id` on the posts table:
    /// 
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Serialize, Deserialize)]
    /// struct Users {
    ///     id: u8,
    ///     full_name: String,
    /// }
    ///
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct Posts {
    ///     id: u8,
    ///     title: String,
    ///     content: String,
    ///     user_id: u8,
    /// }
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table::<Posts>("posts")
    ///         .eq_join(
    ///             "user_id",
    ///             &r.table::<Users>("users"),
    ///         )
    ///         .run(&session)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn eq_join<A: Unpin + Serialize + DeserializeOwned>(
        &self,
        left_field: &str,
        right_table: &TableBuilder<A>,
    ) -> cmd::eq_join::EqJoinBuilder<A, T> {
        cmd::eq_join::EqJoinBuilder::new(left_field, right_table)._with_parent(self.get_parent())
    }

    /// Join tables using a field or function on the left-hand sequence matching primary keys or secondary indexes on the right-hand table. 
    /// `eq_join` is more efficient than other ReQL join types, and operates much faster. 
    /// Documents in the result set consist of pairs of left-hand and right-hand documents, 
    /// matched when the field on the left-hand side exists and is non-null and an entry 
    /// with that field’s value exists in the specified index on the right-hand side.
    /// 
    /// See [eq_join](#method.eq_join) for more informations
    /// 
    /// ## Example
    /// 
    /// Match posts with the users they’ve posted against one another.
    /// 
    /// Join these tables using `user_id` on the users table and `id` on the posts table:
    /// 
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Serialize, Deserialize)]
    /// struct Users {
    ///     id: u8,
    ///     full_name: String,
    /// }
    ///
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct Posts {
    ///     id: u8,
    ///     title: String,
    ///     content: String,
    ///     user_id: u8,
    /// }
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table::<Posts>("posts")
    ///         .eq_join_by_func(
    ///             func!(|row| row.bracket("user_id")),
    ///             &r.table::<Users>("users"),
    ///         )
    ///         .run(&session)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn eq_join_by_func<A: Unpin + Serialize + DeserializeOwned>(
        &self,
        func: Func,
        right_table: &TableBuilder<A>,
    ) -> cmd::eq_join::EqJoinBuilder<A, T> {
        cmd::eq_join::EqJoinBuilder::new_by_func(func, right_table)._with_parent(self.get_parent())
    }

    /// Transform each element of one or more sequences by applying a mapping function to them. 
    /// If `map` is run with two or more sequences, it will iterate for as many items as there are in the shortest sequence.
    /// 
    /// ## Note
    /// 
    /// Note that `map` can only be applied to sequences, not single values. 
    /// If you wish to apply a function to a single value/selection (including an array), use the do_ command.
    /// See [r.map](crate::r::map) for more information.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// use serde::{Serialize, Deserialize};
    /// 
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct Posts {
    ///     id: u8,
    ///     title: String,
    ///     content: String,
    /// }
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table::<Posts>("posts")
    ///         .map::<String>(
    ///             func!(|row| row.bracket("title"))
    ///         )
    ///         .run(&session)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn map<A: Unpin + DeserializeOwned>(&self, func: Func) -> cmd::map::MapBuilder<Sequence<Document<A>>> {
        cmd::map::MapBuilder::new(func)._with_parent(self.get_parent())
    }

    /// Plucks one or more attributes from a sequence of objects, 
    /// filtering out any objects in the sequence that do not have the specified fields. 
    /// Functionally, this is identical to `hasFields` followed by `pluck` on a sequence.
    /// 
    /// ## Example
    /// 
    /// Get a list of users and their posts, excluding any users who have not made any posts
    /// 
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Serialize, Deserialize)]
    /// struct Users {
    ///     id: u8,
    ///     user: String,
    ///     email: String,
    ///     posts: Option<[u8; 3]>,
    /// }
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let user_table = r.table::<Users>("users");
    ///     let users = [
    ///         Users { id: 1, user: "bob".to_string(), email: "bob@foo.com".to_string(), posts: Some([1, 4, 5]) },
    ///         Users { id: 2, user: "george".to_string(), email: "george@foo.com".to_string(), posts: None },
    ///         Users { id: 3, user: "jane".to_string(), email: "jane@foo.com".to_string(), posts: Some([2, 3, 6]) },
    ///     ];
    /// 
    ///     user_table.insert(&users).run(&session).await?;
    /// 
    ///     let _ = user_table.with_fields::<serde_json::Value>(&["id", "user"])
    ///         .run(&session)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn with_fields<A: Unpin + DeserializeOwned>(&self, fields: &[&str]) -> cmd::with_fields::WithFieldsBuilder<Sequence<Document<A>>> {
        cmd::with_fields::WithFieldsBuilder::new(fields)._with_parent(self.get_parent())
    }

    /// Concatenate one or more elements into a single sequence using a mapping function.
    /// 
    /// concatMap works in a similar fashion to map, applying the given function to each element in a sequence, 
    /// but it will always return a single sequence. If the mapping function returns a sequence, 
    /// map would produce a sequence of sequences:
    /// 
    /// ## Example
    /// 
    /// Construct a sequence of all posts wroten by Marvel users. The field posts is an array of one or more posts.
    /// 
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Serialize, Deserialize)]
    /// struct Users {
    ///     id: u8,
    ///     user: String,
    ///     posts: Option<[u8; 3]>,
    /// }
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let user_table = r.table::<Users>("users");
    ///     let users = [
    ///         Users { id: 1, user: "bob".to_string(), posts: Some([1, 4, 5]) },
    ///         Users { id: 2, user: "george".to_string(), posts: None },
    ///         Users { id: 3, user: "jane".to_string(), posts: Some([2, 3, 6]) },
    ///     ];
    /// 
    ///     user_table.insert(&users).run(&session).await?;
    /// 
    ///     let _ = user_table.concat_map::<serde_json::Value>(func!(|row| row.bracket("posts")))
    ///         .run(&session)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn concat_map<A: Unpin + DeserializeOwned>(&self, func: Func) -> cmd::concat_map::ConcatMapBuilder<Sequence<Document<A>>> {
        cmd::concat_map::ConcatMapBuilder::new(func)._with_parent(self.get_parent())
    }

    fn order_by_key(&self, key: &str) -> cmd::order_by::OrderByBuilder<Sequence<T>> {
        cmd::order_by::OrderByBuilder::new_by_key(key)._with_parent(self.get_parent())
    }

    fn order_by_func(&self, func: Func) -> cmd::order_by::OrderByBuilder<T> {
        cmd::order_by::OrderByBuilder::new_by_func(func)._with_parent(self.get_parent())
    }

    /// Skip a number of elements from the head of the sequence.
    /// 
    /// ## Example
    /// 
    /// Here in conjunction with `orderBy` we choose to ignore the most successful heroes
    /// 
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("posts").skip(2).run(&conn).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn skip(&self, number_of_element: usize) -> cmd::skip::SkipBuilder<Sequence<T>> {
        cmd::skip::SkipBuilder::new(number_of_element)._with_parent(self.get_parent())
    }
    
    /// End the sequence after the given number of elements.
    /// 
    /// ## Example
    /// 
    /// Only so many can fit in our Pantheon of heroes
    /// 
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// use serde::{Serialize, Deserialize};
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("posts").limit(3).run(&conn).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn limit(&self, number_of_element: usize) -> cmd::limit::LimitBuilder<Sequence<T>> {
        cmd::limit::LimitBuilder::new(number_of_element)._with_parent(self.get_parent())
    }

    /// Return the elements of a sequence within the specified range.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// use serde::{Serialize, Deserialize};
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("posts").slice(2, Some(5)).run(&conn).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn slice(&self, start_offset: usize, end_offset: Option<usize>) -> cmd::slice::SliceBuilder<Sequence<T>> {
        cmd::slice::SliceBuilder::new(start_offset, end_offset)._with_parent(self.get_parent())
    }

    /// Get the nth element of a sequence, counting from zero. If the argument is negative, count from the last element.
    /// 
    /// ## Example
    /// 
    /// Select the bronze medalist from the competitors
    /// 
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// use serde::{Serialize, Deserialize};
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("players").nth(3).run(&conn).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// ## Example
    /// 
    /// Select the last place competitor
    /// 
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// use serde::{Serialize, Deserialize};
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("players").nth(-1).run(&conn).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn nth(&self, index: isize) -> cmd::nth::NthBuilder<T> {
        cmd::nth::NthBuilder::new(index)._with_parent(self.get_parent())
    }

    /// Get the indexes of an element in a sequence. If the argument is a predicate, get the indexes of all elements matching it.
    fn offsets_of(&self, datum: impl Serialize) -> cmd::offsets_of::OffsetsOfBuilder<Sequence<Document<T>>> {
        cmd::offsets_of::OffsetsOfBuilder::new(datum)._with_parent(self.get_parent())
    }

    fn offsets_of_by_func(&self, func: Func) -> cmd::offsets_of::OffsetsOfBuilder<Sequence<Document<T>>> {
        cmd::offsets_of::OffsetsOfBuilder::new_by_func(func)._with_parent(self.get_parent())
    }

    /// Test if a sequence is empty.
    /// 
    /// ## Example
    /// 
    /// Are there any documents in the marvel table?
    /// 
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("marvel").is_empty().run(&conn).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn is_empty(&self) -> cmd::is_empty::IsEmptyBuilder {
        cmd::is_empty::IsEmptyBuilder::new()._with_parent(self.get_parent())
    }

    /// Merge two or more sequences.
    /// 
    /// The [with_interleave(reql_rust::types::Interleave)](cmd::union::UnionBuilder::with_interleave) method controls how the sequences will be merged:
    /// 
    /// - `Interleave::Bool(true)` : results will be mixed together; this is the fastest setting, but ordering of elements is not guaranteed. 
    /// (This is the default.)
    /// - `Interleave::Bool(false)` : input sequences will be appended to one another, left to right.
    /// - `Interleave::FieldName("field_name")` : a string will be taken as the name of a field to perform a merge-sort on. 
    /// The input sequences must be ordered before being passed to `union` .
    /// 
    /// ## Example
    /// 
    /// Construct a stream of all heroes
    /// 
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// use serde::{Serialize, Deserialize};
    /// 
    /// #[derive(Debug, Serialize, Deserialize)]
    /// struct Users {
    ///     id: u8,
    ///     full_name: String,
    ///     posts: [u8; 2],
    /// }
    ///
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct Posts {
    ///     id: u8,
    ///     title: String,
    ///     content: String,
    ///     user_id: u8,
    /// }
    ///
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct MergePostAndUser {
    ///     id: u8,
    ///     full_name: Option<String>,
    ///     posts: Option<[u8; 2]>,
    ///     title: Option<String>,
    ///     content: Option<String>,
    ///     user_id: Option<u8>,
    /// }
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let marvel_table = r.table::<Users>("users");
    ///     let dc_table = r.table::<Posts>("marvel");
    /// 
    ///     let _ = marvel_table.union::<_, MergePostAndUser>(&[&dc_table])
    ///         .run(&session)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn union<A, B>(&self, sequence: &[&A]) -> cmd::union::UnionBuilder<Sequence<Document<B>>>
    where
        A: SuperOps,
        B: Unpin + Serialize + DeserializeOwned,
    {
        assert!(sequence.len() > 0);
        cmd::union::UnionBuilder::new(sequence)._with_parent(self.get_parent())
    }

    /// Select a given number of elements from a sequence with uniform random distribution. Selection is done without replacement.
    /// 
    /// If the sequence has less than the requested number of elements (i.e., calling `sample(10)` on a sequence with only five elements), 
    /// `sample` will return the entire sequence in a random order.
    /// 
    /// ## Example
    /// 
    /// Select 3 random heroes
    /// 
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("marvel").sample(3).run(&conn).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn sample(&self, number: usize) -> cmd::sample::SampleBuilder<Sequence<T>> {
        cmd::sample::SampleBuilder::new(number)._with_parent(self.get_parent())
    }

    /// Takes a stream and partitions it into multiple groups based on the fields or functions provided.
    fn group<G>(&self, fields: &[&str]) -> cmd::group::GroupBuilder<G, T>
    where
        G: Unpin + Serialize + DeserializeOwned,
    {
        cmd::group::GroupBuilder::new(fields)._with_parent(self.get_parent())
    }

    /// Takes a stream and partitions it into multiple groups based on the fields or functions provided.
    fn group_by_func<G>(&self, func: Func) -> cmd::group::GroupBuilder<G, T>
    where
        G: Unpin + Serialize + DeserializeOwned,
    {
        cmd::group::GroupBuilder::new_by_func(func)._with_parent(self.get_parent())
    }

    fn reduce<A>(&self, func: Func) -> cmd::reduce::ReduceBuilder<A>
    where
        A: Unpin + Serialize + DeserializeOwned,
    {
        cmd::reduce::ReduceBuilder::new(func)._with_parent(self.get_parent())
    }

    fn fold<A, B>(&self, base: A, func: Func) -> cmd::fold::FoldBuilder<A, B>
    where
        A: Serialize,
        B: Unpin + Serialize + DeserializeOwned,
    {
        cmd::fold::FoldBuilder::new(base, func)._with_parent(self.get_parent())
    }

    /// Sums all the elements of a sequence. If called with a field name, 
    /// sums all the values of that field in the sequence, skipping elements of the sequence that lack that field. 
    /// If called with a function, calls that function on every element of the sequence and sums the results, 
    /// skipping elements of the sequence where that function returns null or a non-existence error.
    /// 
    /// Returns 0 when called on an empty sequence.
    fn sum(&self) -> cmd::sum::SumBuilder {
        cmd::sum::SumBuilder::new()._with_parent(self.get_parent())
    }

    /// See [sum](#methods.sum) for more informations
    /// 
    /// ## Example
    /// 
    /// How many points have been scored across all games?
    /// 
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("games").sum_by_field("points").run(&conn).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn sum_by_field(&self, field_name: &str) -> cmd::sum::SumBuilder {
        cmd::sum::SumBuilder::new_by_field(field_name)._with_parent(self.get_parent())
    }

    /// See [sum](#methods.sum) for more informations
    /// 
    /// ## Example
    /// 
    /// How many points have been scored across all games, counting bonus point?
    /// 
    /// ```ignore
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("games").sum_by_func(func!(
    ///         |game| game.bracket("points").add(game.bracket("bonus_points"))
    ///     )).run(&conn).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn sum_by_func(&self, func: Func) -> cmd::sum::SumBuilder {
        cmd::sum::SumBuilder::new_by_func(func)._with_parent(self.get_parent())
    }

    /// Averages all the elements of a sequence. 
    /// If called with a field name, averages all the values of that field in the sequence, 
    /// skipping elements of the sequence that lack that field. 
    /// If called with a function, calls that function on every element of the sequence and averages the results, 
    /// skipping elements of the sequence where that function returns null or a non-existence error.
    /// 
    /// Produces a non-existence error when called on an empty sequence. You can handle this case with default.
    fn avg(&self) -> cmd::avg::AvgBuilder {
        cmd::avg::AvgBuilder::new()._with_parent(self.get_parent())
    }

    /// See [avg](#methods.avg) for more informations
    /// 
    /// ## Example
    /// 
    /// What’s the average number of points scored in a game?
    /// 
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("games").avg_by_field("points").run(&conn).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn avg_by_field(&self, field_name: &str) -> cmd::avg::AvgBuilder {
        cmd::avg::AvgBuilder::new_by_field(field_name)._with_parent(self.get_parent())
    }

    /// See [avg](#methods.avg) for more informations
    /// 
    /// ## Example
    /// 
    /// How many points have been scored across all games, counting bonus point?
    /// 
    /// ```ignore
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("games").avg_by_func(func!(
    ///         |game| game.bracket("points").add(game.bracket("bonus_points"))
    ///     )).run(&conn).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn avg_by_func(&self, func: Func) -> cmd::avg::AvgBuilder {
        cmd::avg::AvgBuilder::new_by_func(func)._with_parent(self.get_parent())
    }

    /// Finds the minimum element of a sequence.
    /// 
    /// an index (the primary key or a secondary index) via [with_index(index: &'static str)](cmd::min::MinBuilder::with_index), 
    /// to return the element of the sequence with the smallest value in that index.
    /// 
    /// Calling `min` on an empty sequence will throw a non-existence error; this can be handled using the default command.
    /// 
    /// ## Example
    /// 
    /// Return the user who has scored the fewest points.
    /// 
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("users")
    ///         .min()
    ///         .with_index("points")
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn min(&self) -> cmd::min::MinBuilder<T> {
        cmd::min::MinBuilder::new()._with_parent(self.get_parent())
    }

    /// See [min](#methods.min) for more informations
    /// 
    /// Return the element of the sequence with the smallest value in that field.
    /// 
    /// ## Example
    /// 
    /// Return the user who has scored the fewest points.
    /// 
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("users").min_by_field("points").run(&conn).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn min_by_field(&self, field_name: &str) -> cmd::min::MinBuilder<T> {
        cmd::min::MinBuilder::new_by_value(field_name)._with_parent(self.get_parent())
    }

    /// See [min](#methods.min) for more informations
    /// 
    /// Ro apply the function to every element within the sequence and return the element which returns the smallest value from the function, 
    /// ignoring any elements where the function produces a non-existence error;
    /// 
    /// ## Example
    /// 
    /// Return the user who has scored the fewest points, adding in bonus points from a separate field using a function.
    /// 
    /// ```ignore
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("games").min_by_func(func!(
    ///         |user| user.bracket("points").add(user.bracket("bonus_points"))
    ///     )).run(&conn).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn min_by_func(&self, func: Func) -> cmd::min::MinBuilder<T> {
        cmd::min::MinBuilder::new_by_func(func)._with_parent(self.get_parent())
    }

    /// Finds the maximum element of a sequence.
    /// 
    /// an index (the primary key or a secondary index) via [with_index(index: &'static str)](cmd::min::MinBuilder::with_index), 
    /// to return the element of the sequence with the largest value in that index.
    /// 
    /// Calling `max` on an empty sequence will throw a non-existence error; this can be handled using the default command.
    /// 
    /// ## Example
    /// 
    /// Return the user who has scored the most points.
    /// 
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("users")
    ///         .max()
    ///         .with_index("points")
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn max(&self) -> cmd::max::MaxBuilder<T> {
        cmd::max::MaxBuilder::new()._with_parent(self.get_parent())
    }

    /// See [max](#methods.max) for more informations
    /// 
    /// Return the element of the sequence with the largest value in that field.
    /// 
    /// ## Example
    /// 
    /// Return the user who has scored the most points.
    /// 
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("users").max_by_field("points").run(&conn).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn max_by_field(&self, field_name: &str) -> cmd::max::MaxBuilder<T> {
        cmd::max::MaxBuilder::new_by_value(field_name)._with_parent(self.get_parent())
    }

    /// See [max](#methods.max) for more informations
    /// 
    /// To apply the function to every element within the sequence and return the element which returns the largest value from the function, 
    /// ignoring any elements where the function produces a non-existence error;
    /// 
    /// ## Example
    /// 
    /// Return the user who has scored the most points, adding in bonus points from a separate field using a function.
    /// 
    /// ```ignore
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("games").max_by_func(func!(
    ///         |user| user.bracket("points").add(user.bracket("bonus_points"))
    ///     )).run(&conn).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn max_by_func(&self, func: Func) -> cmd::max::MaxBuilder<T> {
        cmd::max::MaxBuilder::new_by_func(func)._with_parent(self.get_parent())
    }

    /// Removes duplicates from elements in a sequence.
    /// 
    /// The `distinct` command can be called on any sequence or table with an index.
    /// 
    /// ```text
    /// While distinct can be called on a table without an index, 
    /// the only effect will be to convert the table into a stream; 
    /// the content of the stream will not be affected.
    /// ```
    /// 
    /// ## Example
    /// 
    /// Which unique villains have been vanquished by Marvel heroes?
    /// 
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("games")
    ///         .concat_map::<serde_json::Value>(func!(
    ///             |hero| hero.bracket("villain_list")
    ///         ))
    ///         .distinct()
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// ## Example
    /// 
    /// Topics in a table of messages have a secondary index on them, 
    /// and more than one message can have the same topic. 
    /// What are the unique topics in the table?
    /// 
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("messages")
    ///         .distinct()
    ///         .with_index("topics")
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// 
    fn distinct(&self) -> cmd::distinct::DistinctBuilder<Sequence<T>> {
        cmd::distinct::DistinctBuilder::new()._with_parent(self.get_parent())
    }

    /// Returns `true` if a sequence contains all the specified values.
    /// 
    /// Values may be mixed freely in the argument list.
    /// 
    /// ## Example
    /// 
    /// Has Iron Man ever fought Superman?
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
    ///         .contains("superman")
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn contains(&self, values: impl Serialize) -> cmd::contains::ContainsBuilder {
        cmd::contains::ContainsBuilder::new(values)._with_parent(self.get_parent())
    }

    /// Returns `true` if for each predicate there exists at least one element of the stream where that predicate returns `true` .
    /// 
    /// Predicates may be mixed freely in the argument list.
    fn contains_by_funcs(&self, funcs: Vec<Func>) -> cmd::contains::ContainsBuilder {
        cmd::contains::ContainsBuilder::new_by_func(funcs)._with_parent(self.get_parent())
    }

    /// Return an array containing all of an object’s keys. 
    /// Note that the keys will be sorted as described in [ReQL data types](https://rethinkdb.com/docs/data-types/#sorting-order)
    /// (for strings, lexicographically).
    /// 
    /// ## Example
    /// 
    /// Hulk decides to join the avengers.
    /// 
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("users")
    ///         .get(1)
    ///         .keys()
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn keys(&self) -> cmd::keys::KeysBuilder {
        cmd::keys::KeysBuilder::new()._with_parent(self.get_parent())
    }

    /// Return an array containing all of an object’s values. 
    /// `values()` guarantees the values will come out in the same order as [keys](methods.keys).
    /// 
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("users")
    ///         .get(1)
    ///         .values()
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn values(&self) -> cmd::values::ValuesBuilder {
        cmd::values::ValuesBuilder::new()._with_parent(self.get_parent())
    }
}

pub trait ReqlOpsDocManipulation: SuperOps {
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

pub trait ReqlOpsGroupedStream<G, V>: SuperOps
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

pub trait ReqlOpsArray: SuperOps {
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
    fn splice_at(&self, offset: usize, values: &[impl Serialize]) -> cmd::splice_at::SpliceAtBuilder {
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
    fn delete_at(&self, offset: isize, end_offset: Option<isize>) -> cmd::delete_at::DeleteAtBuilder {
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

pub trait ReqlOpsString: SuperOps {
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
pub trait ReqlOpsObject<T>: SuperOps {
    
}

pub trait SuperOps {
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
