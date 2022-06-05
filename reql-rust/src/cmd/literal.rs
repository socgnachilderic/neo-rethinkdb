use crate::Command;
use ql2::term::TermType;
use serde::Serialize;

pub struct LiteralBuilder(Command);

impl LiteralBuilder {
    pub(crate) fn _new(document: impl Serialize) -> String {
        let arg = Command::from_json(document);
        
        let command = Command::new(TermType::Literal)
            .with_arg(arg)
            .into_arg::<()>()
            .into_cmd();

        // cmd::serialise(&command)
        serde_json::to_string(&crate::proto::Query(&command)).unwrap()
    }
}
