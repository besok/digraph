use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use Color::NoColor;
use crate::analyzer::bipartite::Color::{Black, White};
use crate::DiGraph;

#[derive(PartialEq, Clone, Debug)]
enum Color {
    NoColor,
    Black,
    White,
}

impl Color {
    fn switch(&self) -> Color {
        match self {
            White | NoColor => { Black }
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
    colors: HashMap<NId, Color>,
}


impl<'a, NId, NL, EL> Bipartite<'a, NId, NL, EL>
    where NId: Eq + Hash + Clone + Debug,
{
    fn not_visited(&self, id: NId) -> bool {
        match self.colors.get(&id) {
            None | Some(NoColor) => { true }
            _ => { false }
        }
    }
    fn eq_colors(&self, lhs: NId, rhs: NId) -> bool {
        self.colors.get(&lhs) == self.colors.get(&rhs)
    }
    fn opposite_colors(&self, lhs: NId, rhs: NId) -> bool {
        match (self.colors.get(&lhs), self.colors.get(&rhs)) {
            (Some(White), Some(Black)) | (Some(Black), Some(White)) => { true }
            _ => false
        }
    }

    pub fn new(graph: &'a DiGraph<NId, NL, EL>) -> Self {
        let colors =
            graph.nodes.iter().map(|(k, _)| (k.clone(), NoColor)).collect();
        Self { graph, colors }
    }
    fn has_odd_cycles(&mut self, id: NId, color: Color) -> bool {
        !self.has_no_odd_cycles(id, color)
    }
    fn has_no_odd_cycles(&mut self, id: NId, color: Color) -> bool {
        self.colors.insert(id.clone(), color.clone());
        for ss in self.graph.successor_ids(&id) {
            if self.eq_colors(ss.clone(), id.clone()) {
                return false;
            } else if self.opposite_colors(ss.clone(),id.clone()) {
                continue
            }
            else if self.has_odd_cycles(ss.clone(), color.switch()) {
                return false;
            }
        }
        true
    }
    pub fn bipartite(&mut self) -> bool {
        for (nid, _) in &self.graph.nodes {
            if self.not_visited(nid.clone()) {
                if self.has_odd_cycles(nid.clone(), Black) {
                    return false;
                }
            }
        }
        true
    }
    pub fn no_bipartite(&mut self) -> bool {
        !self.bipartite()
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
        let graph = digraph!((&str,_,_) => ["A","B","C","D"] => {
           "A" => "B";
           "B" => ["C"];
           "C" => "D";
           "D" => "A"

        });

        let res = graph.visualize().str_to_dot_file("dots/gen.svg");

        let mut c = Bipartite::new(&graph);
        assert!(c.bipartite());

        let graph = digraph!((usize,_,_) => [0,1,2,3] => {
           0 => [1,3];
           [1,3] => 2;
        });
        let mut c = Bipartite::new(&graph);
        assert!(c.bipartite());

        let graph = digraph!((&str,_,_) => ["A","B","C","D","E"] => {
           "A" => "B";
           "B" => "C";
           "C" => "D";
           "D" => "E";
           "E" => "A"

        });

        let res = graph.visualize().str_to_dot_file("dots/gen.svg");

        let mut c = Bipartite::new(&graph);
        assert!(c.no_bipartite());
    }
}