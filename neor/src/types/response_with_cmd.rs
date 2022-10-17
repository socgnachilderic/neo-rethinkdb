use crate::Command;

#[derive(Debug, Clone)]
pub struct ResponseWithCmd<T>(pub T, pub Command);

impl<T: Clone> ResponseWithCmd<T> {
    pub fn value(&self) -> T {
        self.0.to_owned()
    }

    pub fn cmd(&self) -> Command {
        self.1.to_owned()
    }
}
