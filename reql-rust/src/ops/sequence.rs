use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::{cmd, Func};
use crate::cmd::table::TableBuilder;

use crate::types::Document;
use crate::types::Sequence;
use crate::types::WritingResponseType;

use super::{ReqlOpsDocManipulation, ReqlOps, ReqlOpsGeometry};

pub trait ReqlOpsSequence<T: Unpin + Serialize + DeserializeOwned>: ReqlOpsDocManipulation {
    fn changes(&self) -> cmd::changes::ChangesBuilder<Document<T>> {
        cmd::changes::ChangesBuilder::new()._with_parent(self.get_parent())
    }
    
    fn values(&self) -> cmd::values::ValuesBuilder {
        cmd::values::ValuesBuilder::new()._with_parent(self.get_parent())
    }

    fn includes<A>(&self, geometry: A) -> cmd::includes::IncludesBuilder<Sequence<A>>
    where
        A: ReqlOpsGeometry + Serialize + DeserializeOwned + Unpin,
    {
        cmd::includes::IncludesBuilder::new(geometry)._with_parent(self.get_parent())
    }
    
    fn intersects<A>(&self, sequence: &[A]) -> cmd::intersects::sequence::IntersectsBuilder<A>
    where
        A: ReqlOpsGeometry + Serialize + DeserializeOwned + Unpin,
    {
        cmd::intersects::sequence::IntersectsBuilder::new(sequence)
    }
}
