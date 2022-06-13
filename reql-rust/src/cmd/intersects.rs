pub mod geometry {
    use futures::{Stream, TryStreamExt};
    use ql2::term::TermType;
    use serde::Serialize;

    use crate::cmd::run;
    use crate::{Command, ops::ReqlOpsGeometry};

    #[derive(Debug, Clone)]
    pub struct IntersectsBuilder(pub(crate) Command);

    impl IntersectsBuilder {
        pub(crate) fn new<T: ReqlOpsGeometry + Serialize>(geometry: T) -> Self {
            let arg = Command::from_json(geometry);
            let command = Command::new(TermType::Intersects).with_arg(arg);

            Self(command)
        }

        pub async fn run(self, arg: impl run::Arg) -> crate::Result<Option<bool>> {
            self.make_query(arg).try_next().await
        }
    
        pub fn make_query(self, arg: impl run::Arg) -> impl Stream<Item = crate::Result<bool>> {
            self.0
                .into_arg::<()>()
                .into_cmd()
                .run::<_, bool>(arg)
        }

        pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
            self.0 = self.0.with_parent(parent);
            self
        }
    }
}

pub mod sequence {
    use futures::{Stream, TryStreamExt};
    use ql2::term::TermType;
    use serde::Serialize;
    use serde_json::Value;

    use crate::cmd::run;
    use crate::{Command, ops::ReqlOpsGeometry};

    #[derive(Debug, Clone)]
    pub struct IntersectsBuilder(pub(crate) Command);

    impl IntersectsBuilder {
        pub(crate) fn new<T: ReqlOpsGeometry + Serialize>(sequence: &[T]) -> Self {
            let arg = Command::from_json(sequence);
            let command = Command::new(TermType::Intersects).with_arg(arg);

            Self(command)
        }

        pub async fn run(self, arg: impl run::Arg) -> crate::Result<Option<Value>> {
            self.make_query(arg).try_next().await
        }
    
        pub fn make_query(self, arg: impl run::Arg) -> impl Stream<Item = crate::Result<Value>> {
            self.0
                .into_arg::<()>()
                .into_cmd()
                .run::<_, Value>(arg)
        }

        pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
            self.0 = self.0.with_parent(parent);
            self
        }
    }
}



// pub trait Arg {
//     fn arg(self) -> cmd::Arg<()>;
// }

// impl Arg for Command {
//     fn arg(self) -> cmd::Arg<()> {
//         Self::new(TermType::Intersects).with_arg(self).into_arg()
//     }
// }
