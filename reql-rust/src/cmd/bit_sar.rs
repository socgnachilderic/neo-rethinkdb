use ql2::term::TermType;

use crate::Command;

use super::CmdOpts;

pub(crate) fn new(args: impl BitSarArg) -> Command {
    args.into_bit_sar_opts()
        .add_to_cmd(Command::new(TermType::BitSar))
}

pub trait BitSarArg {
    fn into_bit_sar_opts(self) -> CmdOpts;
}

impl BitSarArg for f64 {
    fn into_bit_sar_opts(self) -> CmdOpts {
        CmdOpts::Single(Command::from_json(self))
    }
}

impl BitSarArg for Vec<f64> {
    fn into_bit_sar_opts(self) -> CmdOpts {
        CmdOpts::Single(Command::from_json(self))
    }
}

impl BitSarArg for Command {
    fn into_bit_sar_opts(self) -> CmdOpts {
        CmdOpts::Single(self)
    }
}

// TODO write test
