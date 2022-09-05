pub mod geometry {
    use futures::{Stream, TryStreamExt};
    use ql2::term::TermType;
    use serde::Serialize;

    use crate::cmd::run;
    use crate::ops::{ReqlOps, ReqlOpsGeometry};
    use crate::Command;

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
            self.get_parent().run::<_, bool>(arg)
        }

        pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
            self.0 = self.0.with_parent(parent);
            self
        }
    }

    impl ReqlOps for IntersectsBuilder {
        fn get_parent(&self) -> Command {
            self.0.clone().into_arg::<()>().into_cmd()
        }
    }

    impl Into<Command> for IntersectsBuilder {
        fn into(self) -> Command {
            self.get_parent()
        }
    }
}

pub mod sequence {
    use std::marker::PhantomData;

    use futures::{Stream, TryStreamExt};
    use ql2::term::TermType;
    use serde::de::DeserializeOwned;
    use serde::Serialize;

    use crate::cmd::run;
    use crate::ops::{ReqlOps, ReqlOpsDocManipulation, ReqlOpsGeometry, ReqlOpsSequence};
    use crate::types::Sequence;
    use crate::Command;

    #[derive(Debug, Clone)]
    pub struct IntersectsBuilder<T>(pub(crate) Command, PhantomData<T>);

    impl<T: ReqlOpsGeometry + Serialize + DeserializeOwned + Unpin> IntersectsBuilder<T> {
        pub(crate) fn new(sequence: &[T]) -> Self {
            let arg = Command::from_json(sequence);
            let command = Command::new(TermType::Intersects).with_arg(arg);

            Self(command, PhantomData)
        }

        pub async fn run(self, arg: impl run::Arg) -> crate::Result<Option<Sequence<T>>> {
            self.make_query(arg).try_next().await
        }

        pub fn make_query(
            self,
            arg: impl run::Arg,
        ) -> impl Stream<Item = crate::Result<Sequence<T>>> {
            self.get_parent().run::<_, Sequence<T>>(arg)
        }

        pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
            self.0 = self.0.with_parent(parent);
            self
        }
    }

    impl<T: Unpin + Serialize + DeserializeOwned> ReqlOpsSequence<T> for IntersectsBuilder<T> {}

    impl<T> ReqlOpsDocManipulation for IntersectsBuilder<T> {}

    impl<T> ReqlOps for IntersectsBuilder<T> {
        fn get_parent(&self) -> Command {
            self.0.clone().into_arg::<()>().into_cmd()
        }
    }

    impl<T> Into<Command> for IntersectsBuilder<T> {
        fn into(self) -> Command {
            self.get_parent()
        }
    }
}
