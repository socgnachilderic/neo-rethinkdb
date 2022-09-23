use crate::Command;
use ql2::term::TermType;

#[derive(Debug, Clone)]
pub struct Func(pub(crate) Command);

impl Func {
    pub fn new<T>(ids: Vec<u64>, body: T) -> Self
    where
        T: Into<Command>,
    {
        Func(
            Command::new(TermType::Func)
                .with_arg(Command::from_json(ids))
                .with_arg(body),
        )
    }
}
