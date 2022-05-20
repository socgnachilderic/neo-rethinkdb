use ql2::term::TermType;

use crate::Command;

pub(crate) struct DbList;

impl DbList {
    pub fn new() -> Command {        
        Command::new(TermType::DbList)
    }
}
