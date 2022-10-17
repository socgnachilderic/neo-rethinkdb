use std::ops::Add;

use ql2::term::TermType;

use crate::{Command, CommandArg};

impl<T> Add<T> for Command
where
    T: Into<CommandArg>,
{
    type Output = Self;

    /// Sum two or more numbers, or concatenate two or more strings or arrays.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// value + value -> value
    /// time.cmd() + number -> time
    /// value.add(value) -> value
    /// time.cmd().add(number) -> time
    /// ```
    ///
    /// Where:
    /// - number: `i8, u8, ..., isize, usize, f32, f64` | [Command](crate::Command)
    /// - value: `impl impl Serialize` | [Command](crate::Command)
    /// - time: [Time](crate::types::Time) | [Command](crate::Command)
    ///
    /// # Description
    ///
    /// The `add` command can be called in either prefix or infix form;
    /// both forms are equivalent.
    /// Note that ReQL will not perform type coercion.
    /// You cannot, for example, `add` a string and a number together.
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
    ///     let response1: u8 = (r.expr(2) + 2).run(&conn).await?.unwrap().parse()?;
    ///     let response2: String = (r.expr("foo") + "bar" + "baz")
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///     let response3: Vec<String> = (r.expr(["foo", "bar"]) + ["buzz"])
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///     let response4: Time = (r.now().cmd() + 365 * 24 * 60 * 60)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert_eq!(response1, 4);
    ///     assert_eq!(response2, "foobarbaz");
    ///     assert_eq!(response3, ["foo", "bar", "buzz"]);
    ///     assert_ne!(response4.epoch_time, 0.);
    ///
    ///     Ok(())
    /// }
    /// ```
    fn add(self, arg: T) -> Self {
        arg.into().add_to_cmd(TermType::Add).with_parent(&self)
    }
}
