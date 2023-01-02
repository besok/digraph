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
impl<'a, NId, NL, EL> Iterator for NodeIteratorDF<'a, NId, NL, EL>
where
    NId: Eq + Hash + Clone,
{
    type Item = (&'a NId, &'a NL);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.line.pop() {
            for nexts in self.graph.successors(node) {
                for s in nexts.keys() {
                    if self.visited.visit(s) {
                        self.line.push(s);
                    }
                }
            }
            self.graph.node_by_id(node)
        } else {
            None
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
            visited,
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
            for nexts in self.graph.successors(node) {
                for s in nexts.keys() {
                    if self.visited.visit(s) {
                        self.line.push_back(s);
                    }
                }
            }
            self.graph.node_by_id(node)
        } else {
            None
        }
    }
}

pub struct NodeIteratorDFPostOrder<'a, NId, NL, EL>
where
    NId: Eq + Hash,
{
    graph: &'a DiGraph<NId, NL, EL>,
    visited: VisitedSet<'a, NId>,
    processed: VisitedSet<'a, NId>,
    buffer: Vec<&'a NId>,
}

impl<'a, NId, NL, EL> NodeIteratorDFPostOrder<'a, NId, NL, EL>
where
    NId: Eq + Hash + Clone,
{
    pub fn new(graph: &'a DiGraph<NId, NL, EL>) -> Self {
        let line = graph
            .start()
            .as_ref()
            .map(|s| vec![s])
            .unwrap_or_else(|| vec![]);

        Self {
            graph,
            buffer: line,
            visited: Default::default(),
            processed: Default::default(),
        }
    }
}

impl<'a, NId, NL, EL> Iterator for NodeIteratorDFPostOrder<'a, NId, NL, EL>
where
    NId: Eq + Hash + Clone,
{
    type Item = (&'a NId, &'a NL);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(&id) = self.buffer.last() {
            if self.visited.visit(id) {
                for ss in self.graph.successors(id) {
                    for s in ss.keys() {
                        if !self.visited.is_visited(s) {
                            self.buffer.push(s);
                        }
                    }
                }
            } else {
                let node = self.buffer.pop().expect("unreachable!");
                if self.processed.visit(node) {
                    return self.graph.node_by_id(node);
                }
            }
        }
        None
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
            .filter(|s| vec![2, 3].contains(s.clone()))
            .is_some());

        assert_eq!(res_iter_df.get(2), Some(&4));

        assert_ne!(res_iter_bf, res_iter_df);
        res_iter_bf.sort();
        res_iter_df.sort();

        assert_eq!(res_iter_df, res_iter_bf);
    }

    #[test]
    fn post_order_dfs_test() {
        let graph = digraph!((usize,_,_) => [1,2,3,4] => {
           1 => [2,3];
           [2,3] => 4;
           4 => 1;
        });

        let r = graph.visualize().str_to_dot_file("dots/a.svg");
        assert!(r.is_ok());

        let mut res: Vec<usize> = graph.iter_df_post().map(|(id, _)| id.clone()).collect();

        assert_eq!(res[0], 4);
        assert!(vec![2usize, 3].contains(&res[1]));
        assert!(vec![2usize, 3].contains(&res[2]));
        assert_eq!(res[3], 1);
    }
}
