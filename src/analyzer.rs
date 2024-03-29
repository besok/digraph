use crate::DiGraph;
use std::hash::Hash;
use graphviz_rust::dot_structures::Graph;
use crate::analyzer::isomorphism::IsomorphismAnalyzer;
use crate::analyzer::mst::MinimumSpanningArborescence;

pub mod astar;
pub mod dijkstra;
pub mod disjoint;
pub mod dom;
pub mod fs;
pub mod min_weight;
pub mod mst;
pub mod predecessors;
pub mod scc;
pub mod visit;
mod bipartite;
mod isomorphism;

enum SearchRes {
    Next,
    Find,
    Stop,
    Skip,
}

#[derive(Debug)]
pub struct GraphAnalyzer<'a, NodeId, NL, EL>
    where
        NodeId: Eq + Hash,
{
    pub(crate) graph: &'a DiGraph<NodeId, NL, EL>,
}

impl<'a, NodeId, NL, EL> GraphAnalyzer<'a, NodeId, NL, EL>
    where
        NodeId: Eq + Hash,
        NL: PartialEq,
{
    pub fn first_node_by_payload(&self, payload: &NL) -> Option<&NL> {
        self.graph.nodes.values().find(|v| *v == payload)
    }
    pub fn node(&self, id: &NodeId, payload: &NL) -> Option<&NL> {
        self.graph.nodes.get(id).filter(|v| *v == payload)
    }

    pub fn min_spanning_arborescence(&self) -> MinimumSpanningArborescence<'a, NodeId, NL, EL>
        where
            NodeId: Clone,
            EL: Ord,

    {
        return MinimumSpanningArborescence::new(self.graph);
    }

    pub fn is_isomorphic(&self, another: &'a DiGraph<NodeId, NL, EL>) -> bool {
        IsomorphismAnalyzer::new(&self.graph, another).test()
    }
}
