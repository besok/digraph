use graphviz_rust::attributes::{EdgeAttributes, NodeAttributes};
use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;

use crate::visualizer::dot::{DotProcessor, ToStringProcessor};
use crate::DiGraph;
use std::fmt::Debug;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    hash::Hash,
};

const UNDEFINED: usize = std::usize::MAX;

///
/// https://www.cs.rice.edu/~keith/EMBED/dom.pdf
pub struct Dominators<NId>
where
    NId: Eq + Hash + Clone,
{
    internal: HashMap<NId, NId>,
}

pub struct DominatorsHighlighter<NId>
where
    NId: Eq + Hash + Clone,
{
    dominators: Dominators<NId>,
    delegate: ToStringProcessor,
}

impl<'a, NId, NL, EL> DotProcessor<'a, NId, NL, EL> for DominatorsHighlighter<NId>
where
    NId: Eq + Hash + Clone + ToString,
    EL: ToString,
    NL: ToString,
{
    fn node(&self, id: &'a NId, nl: &'a NL) -> Stmt {
        let dom = self.dominators.idom(id);
        let id = id.to_string();
        let label = match dom {
            Some(d) => format!("\"{} {}, dom = {}\"", id, nl.to_string(), d.to_string()),
            None => format!("\"{} {}\"", id, nl.to_string()),
        };
        let mut attrs = vec![];
        attrs.push(NodeAttributes::label(label));
        stmt!(node!(id.as_str(), attrs))
    }

    fn edge(&self, from: &'a NId, to: &'a NId, el: &'a EL) -> Stmt {
        (&self.delegate as &dyn DotProcessor<NId, NL, EL>).edge(from, to, el)
    }
}

impl<NId> DominatorsHighlighter<NId>
where
    NId: Eq + Hash + Clone,
{
    pub fn new(dominators: Dominators<NId>) -> Self {
        Self {
            dominators,
            delegate: ToStringProcessor {},
        }
    }
}

impl<NId> Dominators<NId>
where
    NId: Eq + Hash + Clone,
{
    fn idom(&self, node: &NId) -> Option<&NId> {
        self.internal
            .get(node)
            .and_then(|x| if x == node { None } else { Some(x) })
    }
}

impl<'a, NId> Dominators<NId>
where
    NId: Eq + Hash + Clone,
{
    pub fn simple_fast<NL, EL>(graph: &'a DiGraph<NId, NL, EL>) -> Self {
        let predecessors = graph.predecessors();
        let post_order_line = predecessors.post_order_line();
        let predecessors = predecessors.predecessors();

        let post_order_idx_vec = to_post_order_indexes(&post_order_line, &predecessors);
        let len = post_order_idx_vec.len();
        let mut dominators = vec![UNDEFINED; len];
        dominators[len - 1] = len - 1;

        let mut changed = true;
        while changed {
            changed = false;
            // reverse post order except start node => (0 .. len - 1).rev()
            for idx in (0..len - 1).rev() {
                let predecessors = post_order_idx_vec[idx].clone();
                if !predecessors.is_empty() {
                    let mut new_idom = predecessors[0];
                    for p in 1..predecessors.len() {
                        if dominators[p] != UNDEFINED {
                            new_idom = intersect(&dominators, dominators[p], new_idom);
                        }
                    }
                    if dominators[idx] != new_idom {
                        dominators[idx] = new_idom;
                        changed = true;
                    }
                }
            }
        }

        let internal = dominators
            .into_iter()
            .enumerate()
            .map(|(idx, dom)| (post_order_line[idx].clone(), post_order_line[dom].clone()))
            .collect();

        Self { internal }
    }
}

fn intersect(dominators: &Vec<usize>, mut finger1: usize, mut finger2: usize) -> usize {
    while finger1 != finger2 {
        while finger1 < finger2 {
            finger1 = dominators[finger1];
        }
        while finger2 < finger1 {
            finger2 = dominators[finger2];
        }
    }

    finger1
}

/// creates a post order vec where the index represents an element in post order vec
/// and value it is a vec of indexes of predecessors of the node.    
fn to_post_order_indexes<'a, NId>(
    post_order_line: &Vec<&NId>,
    predecessors: &HashMap<&'a NId, HashSet<&'a NId>>,
) -> Vec<Vec<usize>>
where
    NId: Eq + Hash + Clone,
{
    let mut indexes = vec![];

    let post_order_enumerate: HashMap<&NId, usize> = post_order_line
        .iter()
        .enumerate()
        .map(|(idx, &id)| (id, idx))
        .collect();

    for &node in post_order_line.iter() {
        let predecessors_idx = predecessors
            .get(node)
            .map(|ps| {
                let mut idxs: Vec<usize> = vec![];
                for &p in ps.iter() {
                    if let Some(idx) = post_order_enumerate.get(p) {
                        idxs.push(idx.clone())
                    }
                }
                idxs
            })
            .unwrap_or(vec![]);
        indexes.push(predecessors_idx)
    }

    indexes
}

#[cfg(test)]
mod tests {
    use crate::{
        analyzer::{
            dom::{to_post_order_indexes, Dominators},
            predecessors,
        },
        digraph, extend_edges, extend_nodes, DiGraph, EmptyPayload,
    };
    use std::collections::{HashMap, HashSet};

    use super::DominatorsHighlighter;

    #[test]
    fn smoke_test() {
        let graph = digraph!((usize,_,_) => [0,1,2,3,4] => {
           0 => 1;
           1 => [2,3];
           [2,3] => 4;
        });

        let r = graph.visualize().str_to_dot_file("dots/dom.svg");
        assert!(r.is_ok());

        let doms = graph.dominators();

        assert_eq!(doms.idom(&1), Some(&0));
        assert_eq!(doms.idom(&2), Some(&1));
        assert_eq!(doms.idom(&3), Some(&1));
        assert_eq!(doms.idom(&0), None);
        assert_eq!(doms.idom(&4), Some(&1));
    }
    #[test]
    fn smoke_to_post_order_indexes_test() {
        let graph = digraph!((usize,_,_) => [0,1,2,3,4] => {
           0 => 1;
           1 => [2,3];
           [2,3] => 4;

        });

        let predecessors = graph.predecessors();
        let post_order_line = &predecessors.post_order_line();
        let predecessors = &predecessors.predecessors();
        let post_order_indexes = to_post_order_indexes(post_order_line, predecessors);

        assert_eq!(post_order_line.len(), 5);
        assert_eq!(post_order_line[0], &4);
        assert!(vec![&2, &3].contains(&post_order_line[1]));
        assert!(vec![&2, &3].contains(&post_order_line[2]));
        assert_eq!(post_order_line[3], &1);
        assert_eq!(post_order_line[4], &0);

        assert_eq!(predecessors.len(), 4);
        assert_eq!(predecessors[&1], HashSet::from_iter(vec![&0]));
        assert_eq!(predecessors[&3], HashSet::from_iter(vec![&1]));
        assert_eq!(predecessors[&2], HashSet::from_iter(vec![&1]));
        assert_eq!(predecessors[&4], HashSet::from_iter(vec![&2, &3]));

        assert_eq!(post_order_indexes.len(), 5);
        assert_eq!(post_order_indexes[4], vec![]);
        assert_eq!(post_order_indexes[3], vec![4]);
        assert_eq!(post_order_indexes[2], vec![3]);
        assert_eq!(post_order_indexes[1], vec![3]);
        assert!(post_order_indexes[0] == vec![1, 2] || post_order_indexes[0] == vec![2, 1]);
    }

    #[test]
    fn viz_test() {
        let graph = digraph!((usize,_,_) => [0,1,2,3,4] => {
           0 => 1;
           1 => [2,3];
           [2,3] => 4;

        });
        let dominators = graph.dominators();
        let r = graph
            .visualize()
            .to_dot_file("dots/dom_viz.svg", DominatorsHighlighter::new(dominators));
        assert!(r.is_ok())
    }
}
