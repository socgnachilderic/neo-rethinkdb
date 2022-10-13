use ql2::term::TermType;

use crate::prelude::Func;
use crate::Command;

pub(crate) fn new(func: Func) -> Command {
    Command::new(TermType::ConcatMap).with_arg(func.0)
}

// FIX Bug
// #[cfg(test)]
// mod tests {
//     use crate::prelude::*;
//     use crate::{r, Result};

//     #[tokio::test]
//     async fn test_concat_map_data() -> Result<()> {
//         let conn = r.connection().connect().await?;
//         let data_obtained: Vec<u8> = r
//             .expr([1, 2, 3])
//             .concat_map(func!(|x| vec![r.expr(1), x * 2]))
//             .run(&conn)
//             .await?
//             .unwrap()
//             .parse()?;

//         assert!(data_obtained == vec![1, 2, 2, 4, 3, 6]);

//         Ok(())
//     }
// }
