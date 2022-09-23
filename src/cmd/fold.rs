use ql2::term::TermType;

use crate::arguments::AnyParam;
use crate::prelude::Func;
use crate::Command;

pub(crate) fn new(base: AnyParam, func: Func) -> Command {
    let Func(func) = func;
    let arg: Command = base.into();

    Command::new(TermType::Fold).with_arg(arg).with_arg(func)
}

// #[derive(Debug, Clone, Serialize, Default)]
// #[non_exhaustive]
// pub struct FoldOption {
//     pub emit: Option<Command>,
//     pub final_emit: Option<Command>,
// }

// TODO write test
