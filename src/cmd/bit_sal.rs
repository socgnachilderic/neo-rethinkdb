use ql2::term::TermType;

use crate::Command;

use super::CmdOpts;

pub(crate) fn new(args: impl BitSalArg) -> Command {
    args.into_bit_sal_opts()
        .add_to_cmd(Command::new(TermType::BitSal))
}

pub trait BitSalArg {
    fn into_bit_sal_opts(self) -> CmdOpts;
}

impl BitSalArg for f64 {
    fn into_bit_sal_opts(self) -> CmdOpts {
        CmdOpts::Single(Command::from_json(self))
    }
}

impl BitSalArg for Vec<f64> {
    fn into_bit_sal_opts(self) -> CmdOpts {
        CmdOpts::Single(Command::from_json(self))
    }
}

impl BitSalArg for Command {
    fn into_bit_sal_opts(self) -> CmdOpts {
        CmdOpts::Single(self)
    }
}

// TODO write test
