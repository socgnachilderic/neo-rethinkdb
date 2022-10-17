use std::ops::Rem;

use ql2::term::TermType;

use crate::{Command, CommandArg};

impl<T> Rem<T> for Command
where
    T: Into<CommandArg>,
{
    type Output = Self;

    /// Find the remainder when dividing two numbers.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// number % number -> number
    /// number.rem(number) -> number
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
    ///     let response: u8 = (r.expr(2) % 2).run(&conn).await?.unwrap().parse()?;
    ///
    ///     assert_eq!(response, 0);
    ///
    ///     Ok(())
    /// }
    /// ```
    fn rem(self, arg: T) -> Self {
        arg.into().add_to_cmd(TermType::Mod).with_parent(&self)
    }
}
