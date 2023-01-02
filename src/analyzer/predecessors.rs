use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::DiGraph;

/// Struct holds a map of predecessors(the opposite struct to successors)
/// and
/// a post order dfs vec.
pub struct Predecessors<'a, NId>
where
    NId: Eq + Hash + Clone,
{
    predecessors: HashMap<&'a NId, HashSet<&'a NId>>,
    post_order: Vec<&'a NId>,
}

impl<'a, NId> Predecessors<'a, NId>
where
    NId: Eq + Hash + Clone,
{
    pub fn new<NL, EL>(graph: &'a DiGraph<NId, NL, EL>) -> Self {
        let mut predecessors: HashMap<&NId, HashSet<&NId>> = HashMap::new();
        let mut post_order = vec![];

        for (from, _) in graph.iter_df_post() {
            post_order.push(from);
            if let Some(ss) = graph.successors(from) {
                for to in ss.keys() {
                    predecessors
                        .entry(to)
                        .or_insert_with(HashSet::new)
                        .insert(from);
                }
            }
        }

        Self {
            predecessors,
            post_order,
        }
    }

    pub fn post_order_line(&self) -> Vec<&NId> {
        self.post_order.clone()
    }
    pub fn predecessors(&self) -> HashMap<&NId, HashSet<&NId>> {
        self.predecessors.clone()
    }

    pub fn by_node(&self, id: &NId) -> Option<&HashSet<&NId>> {
        self.predecessors.get(id)
    }
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashSet;

    use crate::{digraph, extend_edges, extend_nodes, DiGraph, EmptyPayload};

    #[test]
    fn smoke_test() {
        let graph = digraph!((usize,_,_) => [0,1,2,3,4] => {
           0 => 1;
           1 => [2,3];
           [2,3] => 4;
           4 => 1;
        });

        let predecessors = graph.predecessors();

        assert_eq!(
            predecessors.by_node(&1).unwrap(),
            &HashSet::from_iter(vec![&4, &0])
        );
        assert_eq!(
            predecessors.by_node(&2).unwrap(),
            &HashSet::from_iter(vec![&1])
        );
        assert_eq!(
            predecessors.by_node(&3).unwrap(),
            &HashSet::from_iter(vec![&1])
        );
        assert_eq!(
            predecessors.by_node(&4).unwrap(),
            &HashSet::from_iter(vec![&2, &3])
        );
        assert_eq!(predecessors.by_node(&0), None);
    }
}
