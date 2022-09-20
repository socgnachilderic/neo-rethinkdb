use std::collections::HashMap;

use serde::Serialize;

use arguments::Permission;
pub use connection::*;
pub use err::*;
pub use proto::Command;
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

    /// Compare values, testing if the left-hand value is greater than the right-hand.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// 
    /// cmd_value.gt(value) → bool
    /// cmd_value.gt(values) → bool
    /// r.gt(values) → bool
    /// ```
    ///
    /// Where:
    /// - value: [Command](crate::Command) | impl Serialize
    /// - values: [Command](crate::Command) | vec![...] | [...] | &[...]
    ///
    /// ## Examples
    ///
    /// Test if a player has scored more than 10 points.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: bool = r.table("players")
    ///         .get(1)
    ///         .g("score")
    ///         .gt(10)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == true);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Test if variables are ordered from lowest to highest, 
    /// with no values being equal to one another.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: bool = r.gt(args!([20, 10, 15]))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == true);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [eq](Self::eq)
    /// - [ne](Self::ne)
    /// - [ge](Self::ge)
    /// - [lt](Self::lt)
    /// - [le](Self::le)
    pub fn gt(self, args: impl cmd::gt::GtArg) -> Command {
        cmd::gt::new(args)
    }

    /// Compare values, testing if the left-hand value is greater than the right-hand.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// 
    /// cmd_value.ge(value) → bool
    /// cmd_value.ge(values) → bool
    /// r.ge(values) → bool
    /// ```
    ///
    /// Where:
    /// - value: [Command](crate::Command) | impl Serialize
    /// - values: [Command](crate::Command) | vec![...] | [...] | &[...]
    ///
    /// ## Examples
    ///
    /// Test if a player has scored more than 10 points.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: bool = r.table("players")
    ///         .get(1)
    ///         .g("score")
    ///         .ge(10)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == true);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Test if variables are ordered from lowest to highest, 
    /// with no values being equal to one another.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: bool = r.ge(args!([20, 10, 15]))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == true);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [eq](Self::eq)
    /// - [ne](Self::ne)
    /// - [gt](Self::gt)
    /// - [lt](Self::lt)
    /// - [le](Self::le)
    pub fn ge(self, args: impl cmd::ge::GeArg) -> Command {
        cmd::ge::new(args)
    }

    /// Compare values, testing if the left-hand value is less than the right-hand.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// 
    /// cmd_value.lt(value) → bool
    /// cmd_value.lt(values) → bool
    /// r.lt(values) → bool
    /// ```
    ///
    /// Where:
    /// - value: [Command](crate::Command) | impl Serialize
    /// - values: [Command](crate::Command) | vec![...] | [...] | &[...]
    ///
    /// ## Examples
    ///
    /// Test if a player has scored less than 10 points.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: bool = r.table("players")
    ///         .get(1)
    ///         .g("score")
    ///         .lt(10)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == true);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Test if variables are ordered from highest to lowest, 
    /// with no values being equal to one another.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: bool = r.lt(args!([20, 10, 15]))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == true);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [eq](Self::eq)
    /// - [ne](Self::ne)
    /// - [gt](Self::gt)
    /// - [ge](Self::ge)
    /// - [le](Self::le)
    pub fn lt(self, args: impl cmd::lt::LtArg) -> Command {
        cmd::lt::new(args)
    }

    /// Compare values, testing if the left-hand value is 
    /// less than or equal to the right-hand.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// 
    /// cmd_value.le(value) → bool
    /// cmd_value.le(values) → bool
    /// r.le(values) → bool
    /// ```
    ///
    /// Where:
    /// - value: [Command](crate::Command) | impl Serialize
    /// - values: [Command](crate::Command) | vec![...] | [...] | &[...]
    ///
    /// ## Examples
    ///
    /// Test if a player has scored 10 points or less.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: bool = r.table("players")
    ///         .get(1)
    ///         .g("score")
    ///         .le(10)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == true);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Test if variables are ordered from highest to lowest.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: bool = r.le(args!([20, 10, 15]))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == true);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [eq](Self::eq)
    /// - [ne](Self::ne)
    /// - [gt](Self::gt)
    /// - [ge](Self::ge)
    /// - [lt](Self::lt)
    pub fn le(self, args: impl cmd::le::LeArg) -> Command {
        cmd::le::new(args)
    }

    /// Compute the logical inverse (not) of an expression.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// !cmd_bool
    /// cmd_bool.not() → bool
    /// r.not(cmd_bool) → bool
    /// ```
    ///
    /// Where:
    /// - cmd_bool: [Command](crate::Command)
    ///
    /// # Description
    ///
    /// `not` can be called either via method chaining, immediately after
    /// an expression that evaluates as a boolean value, or by passing
    /// the expression as a parameter to `not`. All values that are not
    /// `false` or `None` will be converted to `true`.
    ///
    /// ## Examples
    ///
    /// Not true is false.
    ///
    /// ```
    /// use std::ops::Not;
    ///
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: bool = r.expr(true)
    ///         .not()
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     let response2: bool = r.not(r.expr(true))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     let response3: bool = (!r.expr(true))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(
    ///         response == false &&
    ///         response == response2 &&
    ///         response == response3
    ///     );
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [eq](Self::eq)
    /// - [ne](Self::ne)
    pub fn not(self, cmd_bool: Command) -> Command {
        !cmd_bool
    }

    /// Generate a random number between given (or implied) bounds.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.random(()) → number
    /// r.round(param_number) → number
    /// r.round(args!(param_number, param_number)) → number
    /// r.round(args!(param_number, param_number, options)) → number
    /// ```
    ///
    /// Where:
    /// - param_number: f64 | [Command](crate::Command)
    /// - cmd_number: [Command](crate::Command)
    /// - options: [RandomOption](crate::cmd::random::RandomOption)
    ///
    /// # Description
    ///
    /// `random` takes zero, one or two arguments.
    ///
    /// - With *zero* arguments, the result will be a floating-point
    /// number in the range `[0,1)` (from 0 up to but not including 1).
    /// - With *one* argument x, the result will be in the range `[0,x)`, and will
    /// be integer unless `RandomOption::default().float(true)` is given as an option.
    /// Specifying a floating point number without the float option will raise an error.
    /// - With *two* arguments x and y, the result will be in the range
    /// `[x,y)`, and will be integer unless `RandomOption::default().float(true)` is given
    /// as an option. If x and y are equal an error will occur, unless the floating-point
    /// option has been specified, in which case x will be returned.
    /// Specifying a floating point number without the float option will raise an error.
    ///
    /// ## Note
    ///
    /// The last argument given will always be the ‘open’ side of the range, but when
    /// generating a floating-point number, the ‘open’ side may be less than the ‘closed’ side.
    ///
    /// ## Examples
    ///
    /// Generate a random number in the range `[0,1)`
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: u8 = r.random(())
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == 3);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Generate a random integer in the range `[0,100)`
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: u8 = r.random(100.)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == 3);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Generate a random number in the range `(-2.24,1.59]`
    ///
    /// ```
    /// use reql_rust::cmd::random::RandomOption;
    /// use reql_rust::prelude::*;
    /// use reql_rust::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: f64 = r.random(args!(
    ///             1.59, -2.24,
    ///             RandomOption::default().float(true)
    ///         ))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == 0.);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [sample](crate::Command::sample)
    pub fn random(self, args: impl cmd::random::RandomArg) -> Command {
        cmd::random::new(args)
    }

    /// Rounds the given value to the nearest whole integer.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.round(param_number) → number
    /// cmd_number.round() → number
    /// ```
    ///
    /// Where:
    /// - param_number: f64 | [Command](crate::Command)
    /// - cmd_number: [Command](crate::Command)
    ///
    /// # Description
    ///
    /// For example, values of 1.0 up to but not including 1.5
    /// will return 1.0, similar to [floor](Self::floor); values
    /// of 1.5 up to 2.0 will return 2.0, similar to [ceil](Self::ceil).
    ///
    /// ## Examples
    ///
    /// Round 12.345 to the nearest integer.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: i32 = r.round(12.345)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == 12);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// The `round` command can also be chained after an expression.
    ///
    /// ## Examples
    ///
    /// Round -12.345 to the nearest integer.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: i32 = r.expr(-12.345)
    ///         .round()
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == -12);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [ceil](Self::ceil)
    /// - [round](Self::round)
    pub fn round(self, args: impl cmd::round::RoundArg) -> Command {
        cmd::round::new(args)
    }

    /// Rounds the given value up, returning the smallest integer value
    /// greater than or equal to the given value (the value’s ceiling).
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.ceil(param_number) → number
    /// cmd_number.ceil() → number
    /// ```
    ///
    /// Where:
    /// - param_number: f64 | [Command](crate::Command)
    /// - cmd_number: [Command](crate::Command)
    ///
    /// ## Examples
    ///
    /// Return the ceiling of 12.345.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: i32 = r.ceil(12.345)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == 13);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// The `ceil` command can also be chained after an expression.
    ///
    /// ## Examples
    ///
    /// Return the ceiling of -12.345.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: i32 = r.expr(-12.345)
    ///         .ceil()
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == -13);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [floor](Self::floor)
    /// - [round](Self::round)
    pub fn ceil(self, args: impl cmd::ceil::CeilArg) -> Command {
        cmd::ceil::new(args)
    }

    /// Rounds the given value down, returning the largest integer
    /// value less than or equal to the given value (the value’s floor).
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.floor(param_number) → number
    /// cmd_number.floor() → number
    /// ```
    ///
    /// Where:
    /// - param_number: f64 | [Command](crate::Command)
    /// - cmd_number: [Command](crate::Command)
    ///
    /// ## Examples
    ///
    /// Return the floor of 12.345.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: i32 = r.floor(12.345)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == 13);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// The `floor` command can also be chained after an expression.
    ///
    /// ## Examples
    ///
    /// Return the floor of -12.345.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: i32 = r.expr(-12.345)
    ///         .floor()
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == -13);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [ceil](Self::ceil)
    /// - [round](Self::round)
    pub fn floor(self, args: impl cmd::floor::FloorArg) -> Command {
        cmd::floor::new(args)
    }

    /// Compute the arithmetic "and" of one or more values.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// cmd_number & cmd_number
    /// number.bitand(cmd_number) → number
    /// number.bit_and(param_number) → number
    /// r.bit_and(cmd_number, param_number) → number
    /// ```
    ///
    /// Where:
    /// - param_number: i32 | [Command](crate::Command)
    /// - cmd_number: [Command](crate::Command)
    ///
    /// # Description
    ///
    /// A bitwise AND is a binary operation that takes two equal-length binary
    /// representations and performs the logical AND operation on each pair of
    /// the corresponding bits, which is equivalent to multiplying them.
    /// Thus, if both bits in the compared position are 1,
    /// the bit in the resulting binary representation is 1 (1 × 1 = 1);
    /// otherwise, the result is 0 (1 × 0 = 0 and 0 × 0 = 0).
    ///
    /// ## Examples
    ///
    /// Compute the arithmetic "and" of 5 and 3
    ///
    /// ```
    /// use std::ops::BitAnd;
    ///
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: i32 = r.expr(5)
    ///         .bit_and(3)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     let response2: i32 = r.bit_and(r.expr(5), 3)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     let response3: i32 = (r.expr(5) & r.expr(3))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     let response4: i32 = r.expr(5)
    ///         .bitand(r.expr(3))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(
    ///         response == 1 &&
    ///         response == response2 &&
    ///         response == response3 &&
    ///         response == response4
    ///     );
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [bit_or](Self::bit_or)
    /// - [bit_not](Self::bit_not)
    /// - [bit_xor](Self::bit_xor)
    /// - [bit_sal](Self::bit_sal)
    /// - [bit_sar](Self::bit_sar)
    pub fn bit_and(self, cmd_number: Command, args: impl cmd::bit_and::BitAndArg) -> Command {
        cmd_number.bit_and(args)
    }

    /// Compute the arithmetic "or" of one or more values.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// cmd_number | cmd_number
    /// number.bitor(cmd_number) → number
    /// number.bit_or(param_number) → number
    /// r.bit_or(cmd_number, param_number) → number
    /// ```
    ///
    /// Where:
    /// - param_number: i32 | [Command](crate::Command)
    /// - cmd_number: [Command](crate::Command)
    ///
    /// # Description
    ///
    /// A bitwise OR is a binary operation that takes two bit patterns
    /// of equal length and performs the logical inclusive OR operation
    /// on each pair of corresponding bits. The result in each position
    /// is 0 if both bits are 0, while otherwise the result is 1.
    ///
    /// ## Examples
    ///
    /// Compute the arithmetic "or" of 6 and 4
    ///
    /// ```
    /// use std::ops::BitOr;
    ///
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: i32 = r.expr(5)
    ///         .bit_or(3)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     let response2: i32 = r.bit_xor(r.expr(5), 3)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     let response3: i32 = (r.expr(5) ^ r.expr(3))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     let response4: i32 = r.expr(5)
    ///         .bitor(r.expr(3))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(
    ///         response == 7 &&
    ///         response == response2 &&
    ///         response == response3 &&
    ///         response == response4
    ///     );
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [bit_and](Self::bit_and)
    /// - [bit_not](Self::bit_not)
    /// - [bit_xor](Self::bit_xor)
    /// - [bit_sal](Self::bit_sal)
    /// - [bit_sar](Self::bit_sar)
    pub fn bit_or(self, cmd_number: Command, args: impl cmd::bit_or::BitOrArg) -> Command {
        cmd_number.bit_or(args)
    }

    /// Compute the arithmetic "and" of one or more values.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// cmd_number ^ cmd_number
    /// number.bitxor(cmd_number) → number
    /// number.bit_xor(param_number) → number
    /// r.bit_xor(cmd_number, param_number) → number
    /// ```
    ///
    /// Where:
    /// - param_number: i32 | [Command](crate::Command)
    /// - cmd_number: [Command](crate::Command)
    ///
    /// # Description
    ///
    /// A bitwise XOR is a binary operation that takes two bit patterns
    /// of equal length and performs the logical exclusive OR operation
    /// on each pair of corresponding bits. The result in each position
    /// is 1 if only the first bit is 1 or only the second bit is 1,
    /// but will be 0 if both are 0 or both are 1.
    /// In this we perform the comparison of two bits, being 1 if the
    /// two bits are different, and 0 if they are the same.
    ///
    /// ## Examples
    ///
    /// Compute the arithmetic "and" of 6 and 4
    ///
    /// ```
    /// use std::ops::BitXor;
    ///
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: i32 = r.expr(6)
    ///         .bit_xor(4)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     let response2: i32 = r.bit_xor(r.expr(6), 4)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     let response3: i32 = (r.expr(6) ^ r.expr(4))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     let response4: i32 = r.expr(6)
    ///         .bitxor(r.expr(4))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(
    ///         response == 2 &&
    ///         response == response2 &&
    ///         response == response3 &&
    ///         response == response4
    ///     );
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [bit_and](Self::bit_and)
    /// - [bit_not](Self::bit_not)
    /// - [bit_or](Self::bit_or)
    /// - [bit_sal](Self::bit_sal)
    /// - [bit_sar](Self::bit_sar)
    pub fn bit_xor(self, cmd_number: Command, args: impl cmd::bit_xor::BitXorArg) -> Command {
        cmd_number.bit_xor(args)
    }

    /// Compute the arithmetic inverse (not) of an expression.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// number.bit_not() → number
    /// r.bit_not(cmd_number) → number
    /// ```
    ///
    /// Where:
    /// - cmd_number: [Command](crate::Command)
    ///
    /// # Description
    ///
    /// A bitwise NOT, or complement, is a unary operation that performs logical
    /// negation on each bit, forming the ones’ complement of the given binary value.
    /// Bits that are 0 become 1, and those that are 1 become 0.
    ///
    /// ## Examples
    ///
    /// Negate the arithmetice expression
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: i32 = r.expr(7)
    ///         .bit_not()
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     let response2: i32 = r.bit_not(r.expr(7))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == -8 && response == response2);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [bit_and](Self::bit_and)
    /// - [bit_or](Self::bit_or)
    /// - [bit_sal](Self::bit_sal)
    /// - [bit_sar](Self::bit_sar)
    /// - [bit_xor](Self::bit_xor)
    pub fn bit_not(self, cmd_number: Command) -> Command {
        cmd_number.bit_not()
    }

    /// Compute the left arithmetic shift (left logical shift) of one or more values.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// number.bit_sal(param_number) → number
    /// r.bit_sal(cmd_number, param_number) → number
    /// ```
    ///
    /// Where:
    /// - param_number: i32 | [Command](crate::Command)
    /// - cmd_number: [Command](crate::Command)
    ///
    /// # Description
    ///
    /// In an arithmetic shift (also referred to as signed shift),
    /// like a logical shift, the bits that slide off the end disappear
    /// (except for the last, which goes into the carry flag).
    /// But in an arithmetic shift, the spaces are filled in such a way
    /// to preserve the sign of the number being slid. For this reason,
    /// arithmetic shifts are better suited for signed numbers in two’s
    /// complement format.
    ///
    /// ## Note
    ///
    /// SHL and SAL are the same, and differentiation only happens because
    /// SAR and SHR (right shifting) has differences in their implementation.
    ///
    /// ## Examples
    ///
    /// Compute the left arithmetic shift of 5 and 4
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: u8 = r.expr(5)
    ///         .bit_sar(4)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     let response2: u8 = r.bit_sar(r.expr(5), r.expr(4))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == 80 && response == response2);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [bit_and](Self::bit_and)
    /// - [bit_not](Self::bit_not)
    /// - [bit_or](Self::bit_or)
    /// - [bit_sar](Self::bit_sar)
    /// - [bit_xor](Self::bit_xor)
    pub fn bit_sal(self, cmd_number: Command, args: impl cmd::bit_sal::BitSalArg) -> Command {
        cmd_number.bit_sal(args)
    }

    /// Compute the right arithmetic shift of one or more values.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// number.bit_sar(param_number) → number
    /// r.bit_sar(cmd_number, param_number) → number
    /// ```
    ///
    /// Where:
    /// - param_number: i32 | [Command](crate::Command)
    /// - cmd_number: [Command](crate::Command)
    ///
    /// # Description
    ///
    /// In an arithmetic shift (also referred to as signed shift),
    /// like a logical shift, the bits that slide off the end disappear
    /// (except for the last, which goes into the carry flag).
    /// But in an arithmetic shift, the spaces are filled in such
    /// a way to preserve the sign of the number being slid.
    /// For this reason, arithmetic shifts are better suited for
    /// signed numbers in two’s complement format.
    ///
    /// ## Examples
    ///
    /// Compute the right arithmetic shift of 32 and 3
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: u8 = r.expr(32)
    ///         .bit_sar(3)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     let response2: u8 = r.bit_sar(r.expr(32), r.expr(3))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == 4 && response == response2);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [bit_and](Self::bit_and)
    /// - [bit_not](Self::bit_not)
    /// - [bit_or](Self::bit_or)
    /// - [bit_sal](Self::bit_sal)
    /// - [bit_xor](Self::bit_xor)
    pub fn bit_sar(self, cmd_number: Command, args: impl cmd::bit_sar::BitSarArg) -> Command {
        cmd_number.bit_sar(args)
    }

    /// Return a time object representing the current time in UTC.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.now() → time
    /// ```
    ///
    /// Where:
    /// - time: [Time](crate::types::Time)
    ///
    /// # Description
    ///
    /// The command now() is computed once when the server receives the query,
    /// so multiple instances of r.now() will always return the same time inside a query.
    ///
    /// ## Examples
    ///
    /// Create a time
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::types::Time;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let time1: Time = r.now().value();
    ///     let time2: Time = r.now()
    ///         .cmd()
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(time1.is_valid());
    ///     assert!(time2.is_valid());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [time](Self::time)
    /// - [epoch_time](Self::epoch_time)
    /// - [iso8601](Self::iso8601)
    pub fn now(self) -> DateTime {
        DateTime::now()
    }

    /// Create a time object for a specific time.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.time(args!(date, timezone)) → time
    /// r.time(args!(date, time_, timezone)) → time
    /// ```
    ///
    /// Where:
    /// - date: [time::Date](time::Date)
    /// - time_: [time::Time](time::Time)
    /// - timezone: [time::UtcOffset](time::UtcOffset)
    /// - time: [Time](crate::types::Time)
    ///
    /// ## Examples
    ///
    /// Create a time
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::types::Time;
    /// use reql_rust::{args, r, Result};
    /// use time::macros::{date, offset, time};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let date = date!(1986 - 11 - 3);
    ///     let time = time!(09:30:40);
    ///     let timezone = offset!(+01:00);
    ///
    ///     let date_time = r.time(args!(date, time, timezone));
    ///     let time1 = date_time.clone().value();
    ///     let time2: Time = date_time.cmd()
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(time2 == time1);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [now](Self::now)
    /// - [time](Self::time)
    /// - [iso8601](Self::iso8601)
    pub fn time(self, args: impl cmd::time::TimeArg) -> DateTime {
        DateTime::time(args)
    }

    /// Create a time object based on seconds since epoch.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.epoch_time(i64) → time
    /// ```
    ///
    /// Where:
    /// - time: [Time](crate::types::Time)
    ///
    /// ## Examples
    ///
    /// Create a time
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::types::Time;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let date_time = r.epoch_time(531360000)?;
    ///     let time1 = date_time.clone().value();
    ///     let time2: Time = date_time.cmd()
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(time2 == time1);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [now](Self::now)
    /// - [time](Self::time)
    /// - [iso8601](Self::iso8601)
    pub fn epoch_time(self, timestamp: i64) -> Result<DateTime> {
        DateTime::epoch_time(timestamp)
    }

    /// Create a time object based on an ISO 8601
    /// date-time string (e.g. ‘2013-01-01T01:01:01+00:00’).
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.iso8601(string) → time
    /// r.iso8601(args!(string, default_timezone)) → time
    /// ```
    ///
    /// Where:
    /// - time: [Time](crate::types::Time)
    /// - default_timezone: UtcOffset
    ///
    /// # Description
    ///
    /// RethinkDB supports all valid ISO 8601 formats except for week dates.
    /// Read more about the ISO 8601 format at
    /// [Wikipedia](http://en.wikipedia.org/wiki/ISO_8601).
    ///
    /// If you pass an ISO 8601 string without a time zone,
    /// you must specify the time zone with the default_timezone argument.
    ///
    /// ## Examples
    ///
    /// Create a time
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::types::Time;
    /// use reql_rust::{args, r, Result};
    /// use time::macros::offset;
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let date_time = r.iso8601(args!("1986-11-03T08:30:00", offset!(+01:00)))?;
    ///     let time1 = date_time.clone().value();
    ///     let time2: Time = date_time.cmd()
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(time2 == time1);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [now](Self::now)
    /// - [time](Self::time)
    /// - [epoch_time](Self::epoch_time)
    pub fn iso8601(self, args: impl cmd::iso8601::Iso8601) -> Result<DateTime> {
        DateTime::iso8601(args)
    }

    /// Convert `HashMap` to `Command`
    pub fn hash_map<T>(self, value: HashMap<T, Command>) -> Command
    where
        T: Into<String>,
    {
        cmd::hash_map::new(value)
    }

    /// `r.args` is a special term that’s used to splice
    /// an array of arguments into another term.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.args(array) → special
    /// ```
    /// Where:
    /// - array: [Type; usize], Vec<Type>, &\[Type]
    ///
    /// # Description
    ///
    /// This is useful when you want to call a variadic term such as
    /// [get_all](crate::Command::get_all)
    /// with a set of arguments produced at runtime.
    ///
    /// Note that `args` evaluates all its arguments before passing them
    /// into the parent term, even if the parent term otherwise allows lazy evaluation.
    ///
    /// ## Examples
    ///
    /// Unpack array
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let data = vec![1, 2, 3];
    ///
    ///     let response: Vec<u8> = r.args(&data)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == data);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn args<T, S>(self, values: T) -> Command
    where
        S: Serialize,
        T: AsRef<[S]> + Serialize,
    {
        cmd::args::new(values)
    }

    /// Encapsulate binary data within a query.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.binary(data) → binary
    /// ```
    ///
    /// Where:
    /// - data: &[u8]
    /// - binary: [Binary](crate::types::Binary)
    ///
    /// # Description
    ///
    /// Binary struct returned to the client in Rust.
    /// This can be changed with the `binary_format` option
    /// provided to [run](crate::Command::run) to return “raw” objects.
    ///
    /// Only a limited subset of ReQL commands may be chained after `binary`:
    /// - [coerce_to](crate::Command::coerce_to) can coerce binary objects to string types
    /// - [count](Self::count) will return the number of bytes in the object
    /// - [slice](crate::Command::slice) will treat bytes like array indexes
    /// (i.e., slice(args!(10,20)) will return bytes 10–19)
    /// - [type_of](crate::Command::type_of) returns `TypeOf::PtypeBinary`
    /// - [info](Self::info) will return information on a binary struct.
    ///
    /// ## Examples
    ///
    /// Save an avatar image to a existing user record.
    ///
    /// ```
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let avatar_img = std::fs::read("default_avatar.png")?;
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response = r.table("images")
    ///         .insert(r.binary(&avatar_img))
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
    /// Get the size of an existing avatar image.
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: usize = r.table("images")
    ///         .get(100)
    ///         .g("avatar")
    ///         .count(())
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == 14156);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// Read more details about RethinkDB’s binary object support:
    /// [Storing binary objects]("https://rethinkdb.com/docs/storing-binary/python/").
    pub fn binary(self, data: &[u8]) -> Binary {
        cmd::binary::new(data)
    }

    // FIXME Command no work
    pub fn do_(self, args: impl cmd::do_::DoArg) -> Command {
        cmd::do_::new(args)
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
    pub fn branch(self, test: Command, args: impl cmd::branch::BranchArg) -> Command {
        test.branch(args)
    }

    /// Generate a stream of sequential integers in a specified range.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.range(()) → stream
    /// r.range(end_value) → stream
    /// r.range(args!(start_value, end_value)) → stream
    /// ```
    ///
    /// Where
    /// - start_value, end_value: isize
    ///
    /// # Description
    ///
    /// `range` takes 0, 1 or 2 arguments:
    /// - With no arguments, `range` returns an “infinite” stream
    /// from 0 up to and including the maximum integer value;
    /// - With one argument, `range` returns a stream from
    /// 0 up to but not including the end value;
    /// - With two arguments, `range` returns a stream from
    /// the start value up to but not including the end value.
    ///
    /// Note that the left bound (including the implied left
    /// bound of 0 in the 0- and 1-argument form)
    /// is always closed and the right bound is always open:
    /// the start value will always be included in the returned range
    /// and the end value will **not** be included in the returned range.
    ///
    /// Any specified arguments must be integers, or a `ReqlRuntimeError` will be thrown.
    /// If the start value is equal or to higher than the end value,
    /// no error will be thrown but a zero-element stream will be returned.
    ///
    /// ## Examples
    ///
    /// Return a four-element range of `[0, 1, 2, 3]`.
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: [u8; 4] = r.range(4)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == [0, 1, 2, 3]);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// You can also use the [limit](crate::Command::limit)
    /// command with the no-argument
    /// variant to achieve the same result in this case:
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: [u8; 4] = r.range(())
    ///         .limit(4)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == [0, 1, 2, 3]);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Return a range from -5 through 5.
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: [i8; 11] = r.range(args!(-5, 6))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == [-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5]);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn range(self, args: impl cmd::range::RangeArg) -> Command {
        cmd::range::new(args)
    }

    /// Throw a runtime error.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.expr(value) → value
    /// ```
    ///
    /// # Description
    ///
    /// If called with no arguments inside the second
    /// argument to default, re-throw the current error.
    ///
    /// ## Examples
    ///
    /// Get Error
    ///
    /// ```
    /// use reql_rust::{r, ReqlError, ReqlRuntimeError, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let msg = "Error";
    ///
    ///     let err = r.error(msg).run(&conn).await.err().unwrap();
    ///
    ///     if let ReqlError::Runtime(err) = err {
    ///         if let ReqlRuntimeError::User(err) = err {
    ///             assert!(err == msg);
    ///     
    ///             return Ok(());
    ///         }
    ///     }
    ///
    ///     assert!(false);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn error(self, message: impl Into<String>) -> Command {
        cmd::error::new(message)
    }

    /// Construct a ReQL JSON object from a native object.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.expr(value) → value
    /// ```
    ///
    /// # Description
    ///
    /// If the native object is of the `Binary` type,
    /// then expr will return a binary object.
    /// See [binary](Self::binary) for more information.
    ///
    /// ## Examples
    ///
    /// Objects wrapped with `expr` can then be manipulated by ReQL API functions.
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let data = [1, 2, 3];
    ///
    ///     let response: [u8; 3] = r.expr(data)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == data);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn expr(self, value: impl Serialize) -> Command {
        cmd::expr::new(value)
    }

    /// Create a javascript expression.
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
    /// ```text
    /// Whenever possible, you should use native ReQL
    /// commands rather than r.js for better performance.
    /// ```
    ///
    /// ## Examples
    ///
    /// Concatenate two strings using JavaScript.
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: String = r.js("'str1' + 'str2'")
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.eq("str1str2"));
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// You may also specify a timeout in seconds (defaults to 5).
    ///
    /// ```
    /// use reql_rust::cmd::js::JsOption;
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::{args, r, Result};
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response = r.js(args!(
    ///             "while(true) {}",
    ///             JsOption::default().timeout(1.3)
    ///             ))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn js(self, args: impl cmd::js::JsArg) -> Command {
        cmd::js::new(args)
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
    pub fn info(self, any: Command) -> Command {
        any.info()
    }

    /// Parse a JSON string on the server.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.json(json_string) → value
    /// ```
    ///
    /// Where:
    /// - json_string: impl Into<String>
    ///
    /// ## Examples
    ///
    /// Send an array to the server.
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: [u8; 3] = r.json("[1,2,3]")
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == [1, 2, 3]);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn json(self, value: impl Into<String>) -> Command {
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
