use std::collections::HashMap;
use std::hash::Hash;
use std::iter::Map;
use graphviz_rust::dot_generator::graph;
use Color::Init;
use crate::analyzer::bipartite::Color::{Black, White};
use crate::{DiGraph, digraph, extend_edges, extend_nodes};

#[derive(PartialEq, Clone)]
enum Color {
    Init,
    Black,
    White,
}

impl Color {
    fn switch(&self) -> Color {
        match self {
            White | Init => { Black }
            Black => { White }
        }
    }
}


///
/// https://en.wikipedia.org/wiki/Bipartite_graph
///
struct Bipartite<'a, NId, NL, EL>
    where NId: Eq + Hash + Clone,
{
    graph: &'a DiGraph<NId, NL, EL>,
    visited: HashMap<NId, bool>,
    colors: HashMap<NId, Color>,
}


impl<'a, NId, NL, EL> Bipartite<'a, NId, NL, EL>
    where NId: Eq + Hash + Clone,
{
    fn not_visited(&self, id: NId) -> bool {
        self.visited.get(&id).map(|x| !x).unwrap_or(true)
    }
    fn eq_colors(&self, lhs: NId, rhs: NId) -> bool {
        self.colors.get(&lhs) == self.colors.get(&rhs)
    }

    pub fn new(graph: &'a DiGraph<NId, NL, EL>) -> Self {
        let visited: HashMap<_, _> =
            graph.nodes.iter().map(|(k, _)| (k.clone(), false)).collect();
        let colors =
            graph.nodes.iter().map(|(k, _)| (k.clone(), Init)).collect();
        Self { graph, visited, colors }
    }
    fn no_odd_cycles(&mut self, id: NId, color: Color) -> bool {
        self.visited.insert(id.clone(), true);
        self.colors.insert(id.clone(), color.clone());
        for ss in self.graph.successors_ids(&id) {
            if self.not_visited(ss.clone()) {
                if !self.no_odd_cycles(ss.clone(), color.switch()) {
                    return false;
                } else if self.eq_colors(ss.clone(), id.clone()) {
                    return false;
                }
            }
        }
        true
    }
    pub fn no_bipartite(&mut self) -> bool {
        for (nid, _) in &self.graph.nodes {
            if self.not_visited(nid.clone()) {
                if !self.no_odd_cycles(nid.clone(), Black) {
                    return true;
                }
            }
        }

        false
    }
    pub fn is_bipartite(&mut self) -> bool {
        return !self.no_bipartite();
    }
}

#[cfg(test)]
mod tests {
    use crate::DiGraph;
    use crate::EmptyPayload;
    use crate::{digraph, extend_edges, extend_nodes};
    use crate::analyzer::bipartite::Bipartite;

    #[test]
    fn smoke_test() {
        let graph = digraph!((usize,_,_) => [0,1,2,3] => {
           0 => [1,3];
           1 => 2;
           2 => 3
        });

        let mut c = Bipartite::new(&graph);
        assert!(c.no_bipartite());

        let graph = digraph!((usize,_,_) => [0,1,2,3] => {
           0 => [1,3];
           [1,3] => 2;
        });
        let mut c = Bipartite::new(&graph);
        assert!(c.is_bipartite());
    }
}