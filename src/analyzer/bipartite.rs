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
            NoColor => Black,
            White => Black,
            Black => White
        }
    }
    fn is_opposite(&self, color: &Color) -> bool {
        match (&self, color) {
            (White, Black) | (Black, White) => true,
            _ => false
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
    fn is_opposite(&self, lhs: &NId, rhs: &NId) -> bool {
        match (self.colors.get(lhs), self.colors.get(rhs)) {
            (Some(l), Some(r)) => l.is_opposite(r),
            _ => false
        }
    }
    fn not_visited(&self, nid: &NId) -> bool {
        match self.colors.get(nid) {
            None | Some(NoColor) => true,
            _ => false
        }
    }

    pub fn new(graph: &'a DiGraph<NId, NL, EL>) -> Self {
        let mut colors: HashMap<NId, Color> =
            graph.nodes.iter().map(|(id, _)| (id.clone(), NoColor)).collect();
        Self { graph, colors }
    }
    fn has_odd_cycles(&mut self, id: NId) -> bool {
        !self.has_no_odd_cycles(id)
    }
    fn has_no_odd_cycles(&mut self, id: NId) -> bool {
        let mut q = vec![];
        q.push(id);

        while let Some(id) = q.pop() {
            for ss in self.graph.successor_ids(&id) {
                if self.not_visited(ss) {
                    let color = self.colors.get(&id).map(Color::switch).unwrap_or(Black);
                    self.colors.insert(ss.clone(), color);
                    q.push(ss.clone())
                } else if !self.is_opposite(&id, ss) {
                    return false;
                }
            }
        }

        true
    }
    pub fn bipartite(&mut self) -> bool {
        for (nid, _) in &self.graph.nodes {
            if self.not_visited(nid) {
                self.colors.insert(nid.clone(), Black);
                if self.has_odd_cycles(nid.clone()) {
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
           "B" => "C";
           "C" => "D";
           "D" => "A"

        });

        let res = graph.visualize().str_to_dot_file("dots/gen.svg");

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