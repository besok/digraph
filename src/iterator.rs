use crate::{
    analyzer::visit::{Visited, VisitedSet},
    DiGraph,
};
use std::{
    collections::{hash_map::Iter, HashSet, VecDeque},
    hash::Hash,
};

pub struct NodeIteratorPlain<'a, NId, NL>
where
    NId: Eq + Hash,
{
    delegate: Iter<'a, NId, NL>,
}

impl<'a, NId, NL> Iterator for NodeIteratorPlain<'a, NId, NL>
where
    NId: Eq + Hash,
{
    type Item = (&'a NId, &'a NL);

    fn next(&mut self) -> Option<Self::Item> {
        self.delegate.next()
    }
}

impl<'a, NId, NL> NodeIteratorPlain<'a, NId, NL>
where
    NId: Eq + Hash,
{
    pub fn new<EL>(graph: &'a DiGraph<NId, NL, EL>) -> Self {
        Self {
            delegate: graph.nodes.iter(),
        }
    }
}

pub struct NodeIteratorDF<'a, NId, NL, EL>
where
    NId: Eq + Hash,
{
    graph: &'a DiGraph<NId, NL, EL>,
    visited: VisitedSet<'a, NId>,
    line: Vec<&'a NId>,
}

impl<'a, NId, NL, EL> NodeIteratorDF<'a, NId, NL, EL>
where
    NId: Eq + Hash + Clone,
{
    pub fn new(graph: &'a DiGraph<NId, NL, EL>) -> Self {
        let mut visited = VisitedSet::default();
        let line = graph
            .start()
            .as_ref()
            .map(|s| {
                visited.visit(s);
                vec![s]
            })
            .unwrap_or_else(|| vec![]);

        Self {
            graph,
            line,
            visited: Default::default(),
        }
    }
}

pub struct NodeIteratorBF<'a, NId, NL, EL>
where
    NId: Eq + Hash,
{
    graph: &'a DiGraph<NId, NL, EL>,
    visited: VisitedSet<'a, NId>,
    line: VecDeque<&'a NId>,
}

impl<'a, NId, NL, EL> NodeIteratorBF<'a, NId, NL, EL>
where
    NId: Eq + Hash + Clone,
{
    pub fn new(graph: &'a DiGraph<NId, NL, EL>) -> Self {
        let mut visited = VisitedSet::default();
        let line = graph
            .start()
            .as_ref()
            .map(|s| {
                visited.visit(s);
                VecDeque::from_iter(vec![s])
            })
            .unwrap_or_else(|| VecDeque::new());

        Self {
            graph,
            line,
            visited: Default::default(),
        }
    }
}

impl<'a, NId, NL, EL> Iterator for NodeIteratorBF<'a, NId, NL, EL>
where
    NId: Eq + Hash + Clone,
{
    type Item = (&'a NId, &'a NL);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.line.pop_front() {
            for nexts in self.graph.successors(node.clone()) {
                for s in nexts.keys() {
                    if !self.visited.already_visited(s) {
                        self.line.push_back(s);
                        self.visited.visit(s);
                    }
                }
            }
            self.graph.node_by_id(node)
        } else {
            None
        }
    }
}

impl<'a, NId, NL, EL> Iterator for NodeIteratorDF<'a, NId, NL, EL>
where
    NId: Eq + Hash + Clone,
{
    type Item = (&'a NId, &'a NL);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.line.pop() {
            for nexts in self.graph.successors(node.clone()) {
                for s in nexts.keys() {
                    if !self.visited.already_visited(s) {
                        self.line.push(s);
                        self.visited.visit(s);
                    }
                }
            }
            self.graph.node_by_id(node)
        } else {
            None
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{digraph, extend_edges, extend_nodes, DiGraph, EmptyPayload};

    use super::NodeIteratorBF;
    use super::NodeIteratorDF;

    #[test]
    fn simple_test() {
        let graph = digraph!((usize,_,_) => [1,2,3,4] => {
           1 => [2,3];
           [2,3] => 4;
        });

        let r = graph.visualize().str_to_dot_file("dots/a.svg");
        assert!(r.is_ok());

        let iter_d = NodeIteratorDF::new(&graph);
        let iter_b = NodeIteratorBF::new(&graph);

        let mut res_iter: Vec<usize> = graph.iter().map(|(id, _)| id.clone()).collect();
        let mut res_iter_df: Vec<usize> = graph.iter_df().map(|(id, _)| id.clone()).collect();
        let mut res_iter_bf: Vec<usize> = graph.iter_bf().map(|(id, _)| id.clone()).collect();

        res_iter.sort();
        assert_eq!(res_iter, vec![1, 2, 3, 4]);

        assert!(res_iter_bf
            .get(2)
            .filter(|s| s.clone() == &2 || s.clone() == &3)
            .is_some());

        assert_eq!(res_iter_df.get(2), Some(&4));

        assert_ne!(res_iter_bf, res_iter_df);
        res_iter_bf.sort();
        res_iter_df.sort();

        assert_eq!(res_iter_df, res_iter_bf);
    }
}
