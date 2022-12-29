use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::DiGraph;

pub struct Predecessors<'a, NId>
where
    NId: Eq + Hash,
{
    predecessors: HashMap<&'a NId, HashSet<&'a NId>>,
}

impl<'a, NId> Predecessors<'a, NId>
where
    NId: Eq + Hash,
{
    pub fn new<NL, EL>(graph: &'a DiGraph<NId, NL, EL>) -> Self {
        let mut predecessors: HashMap<&NId, HashSet<&NId>> = HashMap::new();

        for (from, ss) in graph.edges.iter() {
            for to in ss.keys() {
                predecessors
                    .entry(to)
                    .and_modify(|s| {
                        s.insert(from);
                    })
                    .or_insert(HashSet::from_iter(vec![from]));
            }
        }

        Self { predecessors }
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
