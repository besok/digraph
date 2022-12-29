use crate::analyzer::dijkstra::DijkstraPath;
use crate::DiGraph;
use graphviz_rust::dot_structures::Node;
use std::hash::Hash;

pub mod astar;
pub mod dijkstra;
pub mod dom;
pub mod fs;
pub mod min_weight;
pub mod predecessors;
pub mod visit;

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
}
