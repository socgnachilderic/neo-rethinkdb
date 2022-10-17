use std::ops::Sub;

use ql2::term::TermType;

use crate::{Command, CommandArg};

impl<T> Sub<T> for Command
where
    T: Into<CommandArg>,
{
    type Output = Self;

    /// Subtract two numbers.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// number - number -> number
    /// time.cmd() - number -> time
    /// time.cmd() - time.cmd() -> time
    /// number.sub(number) -> number
    /// time.cmd().sub(number) -> time
    /// time.cmd().sub(time.cmd()) -> time
    /// ```
    ///
    /// Where:
    /// - number: `i8, u8, ..., isize, usize, f32, f64` | [Command](crate::Command)
    /// - time: [Time](crate::types::Time) | [Command](crate::Command)
    ///
    /// ## Examples
    ///
    /// Create a simple object.
    ///
    /// ```
    /// use neor::types::Time;
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response1: u8 = (r.expr(2) - 2).run(&conn).await?.unwrap().parse()?;
    ///     let response2: Time = (r.now().cmd() - 365 * 24 * 60 * 60)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert_eq!(response1, 0);
    ///     assert!(!response2.epoch_time.is_nan());
    ///
    ///     Ok(())
    /// }
    /// ```
    fn sub(self, arg: T) -> Self {
        arg.into().add_to_cmd(TermType::Sub).with_parent(&self)
    }
}
