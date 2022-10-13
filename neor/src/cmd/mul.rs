use std::ops::Mul;

use ql2::term::TermType;

use crate::{Command, CommandArg};

impl<T> Mul<T> for Command
where
    T: Into<CommandArg>,
{
    type Output = Self;

    /// Multiply two numbers, or make a periodic array.
    /// 
    /// # Command syntax
    ///
    /// ```text
    /// number * number -> number
    /// array * number -> time
    /// number.mul(number) -> number
    /// array.mul(number) -> time
    /// ```
    ///
    /// Where:
    /// - number: `i8, u8, ..., isize, usize, f32, f64` | [Command](crate::Command)
    /// - array: `impl IntoIterator<T>` | [Command](crate::Command)
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
    ///     let response1: u8 = (r.expr(2) * 2).run(&conn).await?.unwrap().parse()?;
    ///     let response2: Vec<String> = (r.expr(["This", "is", "the", "song", "that", "never", "ends."]) * 100)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert_eq!(response1, 4);
    ///     assert_eq!(!response2.len(), 700);
    ///
    ///     Ok(())
    /// }
    /// ```
    fn mul(self, arg: T) -> Self {
        arg.into().add_to_cmd(TermType::Mul).with_parent(&self)
    }
}
