use ql2::term::TermType;

use crate::arguments::{Args, RandomOption};
use crate::{Command, CommandArg};

pub(crate) fn new(args: impl RandomArg) -> Command {
    let (arg1, arg2, opts) = args.into_random_opts();
    let mut command = Command::new(TermType::Random);

    if let Some(arg) = arg1 {
        command = command.with_arg(arg)
    }

    if let Some(arg) = arg2 {
        command = command.with_arg(arg)
    }

    command.with_opts(opts)
}

pub trait RandomArg {
    fn into_random_opts(self) -> (Option<Command>, Option<Command>, RandomOption);
}

impl RandomArg for () {
    fn into_random_opts(self) -> (Option<Command>, Option<Command>, RandomOption) {
        (None, None, Default::default())
    }
}

impl RandomArg for i8 {
    fn into_random_opts(self) -> (Option<Command>, Option<Command>, RandomOption) {
        (Some(Command::from_json(self)), None, Default::default())
    }
}

impl RandomArg for u8 {
    fn into_random_opts(self) -> (Option<Command>, Option<Command>, RandomOption) {
        (Some(Command::from_json(self)), None, Default::default())
    }
}

impl RandomArg for i16 {
    fn into_random_opts(self) -> (Option<Command>, Option<Command>, RandomOption) {
        (Some(Command::from_json(self)), None, Default::default())
    }
}

impl RandomArg for u16 {
    fn into_random_opts(self) -> (Option<Command>, Option<Command>, RandomOption) {
        (Some(Command::from_json(self)), None, Default::default())
    }
}

impl RandomArg for i32 {
    fn into_random_opts(self) -> (Option<Command>, Option<Command>, RandomOption) {
        (Some(Command::from_json(self)), None, Default::default())
    }
}

impl RandomArg for u32 {
    fn into_random_opts(self) -> (Option<Command>, Option<Command>, RandomOption) {
        (Some(Command::from_json(self)), None, Default::default())
    }
}

impl RandomArg for i64 {
    fn into_random_opts(self) -> (Option<Command>, Option<Command>, RandomOption) {
        (Some(Command::from_json(self)), None, Default::default())
    }
}

impl RandomArg for u64 {
    fn into_random_opts(self) -> (Option<Command>, Option<Command>, RandomOption) {
        (Some(Command::from_json(self)), None, Default::default())
    }
}

impl RandomArg for i128 {
    fn into_random_opts(self) -> (Option<Command>, Option<Command>, RandomOption) {
        (Some(Command::from_json(self)), None, Default::default())
    }
}

impl RandomArg for u128 {
    fn into_random_opts(self) -> (Option<Command>, Option<Command>, RandomOption) {
        (Some(Command::from_json(self)), None, Default::default())
    }
}

impl RandomArg for isize {
    fn into_random_opts(self) -> (Option<Command>, Option<Command>, RandomOption) {
        (Some(Command::from_json(self)), None, Default::default())
    }
}

impl RandomArg for usize {
    fn into_random_opts(self) -> (Option<Command>, Option<Command>, RandomOption) {
        (Some(Command::from_json(self)), None, Default::default())
    }
}

impl RandomArg for f32 {
    fn into_random_opts(self) -> (Option<Command>, Option<Command>, RandomOption) {
        (Some(Command::from_json(self)), None, Default::default())
    }
}

impl RandomArg for f64 {
    fn into_random_opts(self) -> (Option<Command>, Option<Command>, RandomOption) {
        (Some(Command::from_json(self)), None, Default::default())
    }
}

impl RandomArg for Command {
    fn into_random_opts(self) -> (Option<Command>, Option<Command>, RandomOption) {
        (Some(self), None, Default::default())
    }
}

impl<S, T> RandomArg for Args<(S, T)>
where
    S: Into<CommandArg>,
    T: Into<CommandArg>,
{
    fn into_random_opts(self) -> (Option<Command>, Option<Command>, RandomOption) {
        (
            Some(self.0 .0.into().to_cmd()),
            Some(self.0 .1.into().to_cmd()),
            Default::default(),
        )
    }
}

impl<S, T> RandomArg for Args<(S, T, RandomOption)>
where
    S: Into<CommandArg>,
    T: Into<CommandArg>,
{
    fn into_random_opts(self) -> (Option<Command>, Option<Command>, RandomOption) {
        (
            Some(self.0 .0.into().to_cmd()),
            Some(self.0 .1.into().to_cmd()),
            self.0 .2,
        )
    }
}
