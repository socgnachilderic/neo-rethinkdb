use ql2::term::TermType;

use crate::{arguments::Args, Command, CommandArg};

pub(crate) fn new(args: impl RangeArg) -> Command {
    let (arg1, arg2) = args.into_range_opts();
    let mut command = Command::new(TermType::Range);

    if let Some(arg) = arg1 {
        command = command.with_arg(arg)
    }

    if let Some(arg) = arg2 {
        command = command.with_arg(arg)
    }

    command
}

pub trait RangeArg {
    fn into_range_opts(self) -> (Option<Command>, Option<Command>);
}

impl RangeArg for () {
    fn into_range_opts(self) -> (Option<Command>, Option<Command>) {
        (None, None)
    }
}

impl RangeArg for i8 {
    fn into_range_opts(self) -> (Option<Command>, Option<Command>) {
        (None, Some(Command::from_json(self)))
    }
}

impl RangeArg for u8 {
    fn into_range_opts(self) -> (Option<Command>, Option<Command>) {
        (None, Some(Command::from_json(self)))
    }
}

impl RangeArg for i16 {
    fn into_range_opts(self) -> (Option<Command>, Option<Command>) {
        (None, Some(Command::from_json(self)))
    }
}

impl RangeArg for u16 {
    fn into_range_opts(self) -> (Option<Command>, Option<Command>) {
        (None, Some(Command::from_json(self)))
    }
}

impl RangeArg for i32 {
    fn into_range_opts(self) -> (Option<Command>, Option<Command>) {
        (None, Some(Command::from_json(self)))
    }
}

impl RangeArg for u32 {
    fn into_range_opts(self) -> (Option<Command>, Option<Command>) {
        (None, Some(Command::from_json(self)))
    }
}

impl RangeArg for i64 {
    fn into_range_opts(self) -> (Option<Command>, Option<Command>) {
        (None, Some(Command::from_json(self)))
    }
}

impl RangeArg for u64 {
    fn into_range_opts(self) -> (Option<Command>, Option<Command>) {
        (None, Some(Command::from_json(self)))
    }
}

impl RangeArg for i128 {
    fn into_range_opts(self) -> (Option<Command>, Option<Command>) {
        (None, Some(Command::from_json(self)))
    }
}

impl RangeArg for u128 {
    fn into_range_opts(self) -> (Option<Command>, Option<Command>) {
        (None, Some(Command::from_json(self)))
    }
}

impl RangeArg for isize {
    fn into_range_opts(self) -> (Option<Command>, Option<Command>) {
        (None, Some(Command::from_json(self)))
    }
}

impl RangeArg for usize {
    fn into_range_opts(self) -> (Option<Command>, Option<Command>) {
        (None, Some(Command::from_json(self)))
    }
}

impl RangeArg for Command {
    fn into_range_opts(self) -> (Option<Command>, Option<Command>) {
        (None, Some(self))
    }
}

impl<S, E> RangeArg for Args<(S, E)>
where
    S: Into<CommandArg>,
    E: Into<CommandArg>,
{
    fn into_range_opts(self) -> (Option<Command>, Option<Command>) {
        (
            Some(self.0 .0.into().to_cmd()),
            Some(self.0 .1.into().to_cmd()),
        )
    }
}
