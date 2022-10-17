use std::ops::Div;

use ql2::term::TermType;

use crate::{Command, CommandArg};

impl<T> Div<T> for Command
where
    T: Into<CommandArg>,
{
    type Output = Self;

    /// Divide two numbers.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// number / number -> number
    /// number.div(number) -> number
    /// ```
    ///
    /// Where:
    /// - number: `i8, u8, ..., isize, usize, f32, f64` | [Command](crate::Command)
    ///
    /// ## Examples
    ///
    /// Create a simple object.
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: f64 = (r.expr(2) / 2).run(&conn).await?.unwrap().parse()?;
    ///
    ///     assert_eq!(response, 1.);
    ///
    ///     Ok(())
    /// }
    /// ```
    fn div(self, arg: T) -> Self {
        arg.into().add_to_cmd(TermType::Div).with_parent(&self)
    }
}
