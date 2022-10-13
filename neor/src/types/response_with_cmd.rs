use crate::Command;

#[derive(Debug, Clone)]
pub struct ResponseWithCmd<T>(pub T, pub Command);

impl<T> ResponseWithCmd<T> {
    pub fn value(self) -> T {
        self.0
    }

    pub fn cmd(self) -> Command {
        self.1
    }
}
