use ql2::term::TermType;
use serde::Serialize;

use crate::prelude::Func;
use crate::Command;

pub(crate) fn new<T>(base: T, func: Func) -> Command
where
    T: Serialize,
{
    Command::new(TermType::Fold)
        .with_arg(Command::from_json(base))
        .with_arg(func.0)
}

// #[derive(Debug, Clone, Serialize, Default)]
// #[non_exhaustive]
// pub struct FoldOption {
//     pub emit: Option<Command>,
//     pub final_emit: Option<Command>,
// }

#[cfg(test)]
mod tests {
    use crate::args;
    use crate::prelude::*;
    use crate::spec::*;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_fold_ops() -> Result<()> {
        let posts = Post::get_many_data()
            .into_iter()
            .fold(String::new(), |acc, post| {
                format!("{}{}{}", acc, if acc == "" { "" } else { ", " }, post.title)
            });
        let (conn, table, table_name) = set_up(true).await?;
        let response: String = table
            .order_by(r.expr("id"))
            .fold(
                "",
                func!(|acc, post| acc.clone()
                    + r.branch(acc.eq(""), args!(r.expr(""), r.expr(", ")))
                    + post.g("title")),
            )
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response == posts);

        tear_down(conn, &table_name).await
    }
}
