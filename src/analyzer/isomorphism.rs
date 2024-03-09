use std::hash::Hash;
use crate::DiGraph;

pub struct IsomorphismAnalyzer<'a, NodeId, NL, EL>
    where
        NodeId: Eq + Hash,
{
    pub(crate) lhs: &'a DiGraph<NodeId, NL, EL>,
    pub(crate) rhs: &'a DiGraph<NodeId, NL, EL>,
}

impl<'a, NodeId, NL, EL> IsomorphismAnalyzer<'a, NodeId, NL, EL> where
    NodeId: Eq + Hash, {
    pub fn new(lhs: &'a DiGraph<NodeId, NL, EL>, rhs: &'a DiGraph<NodeId, NL, EL>) -> Self {
        Self { lhs, rhs }
    }

    pub fn test(&self) -> bool {
        return false;
    }
}
