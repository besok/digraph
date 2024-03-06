use std::cmp::{max, min};
use std::hash::Hash;
use graphviz_rust::attributes::{color_name, EdgeAttributes, NodeAttributes};
use graphviz_rust::dot_structures::Stmt;
use crate::{DiGraph, Edge};
use crate::analyzer::dijkstra::MinPathProcessor;
use crate::visualizer::dot::{DotProcessor, ToStringProcessor};

/// The Minimum Spanning Arborescence (MSA) is a concept in graph theory
/// that is particularly relevant for directed graphs (digraphs).
/// It is a spanning tree of a directed graph that minimizes the sum of the edge weights,
/// considering the directed nature of the edges.
///
/// Note: Should be DAG
#[derive(Debug)]
pub struct MinimumSpanningArborescence<'a, NId, NL, EL>
    where
        NId: Eq + Hash + Clone,
        EL: Ord
{
    graph: &'a DiGraph<NId, NL, EL>,
    forest: Vec<Vec<&'a NId>>,
}

impl<'a, NId, NL, EL> MinimumSpanningArborescence<'a, NId, NL, EL> where
    NId: Eq + Hash + Clone,
    EL: Ord
{
    pub fn find(&'a mut self) -> Vec<Edge<'a, NId, EL>> {
        let mut msa = vec![];
        let mut edges: Vec<Edge<NId, EL>> = self.graph.edges();
        self.fill_forest();
        edges.sort_by_key(|e| e.payload);

        for e @ Edge { src, trg, .. } in edges {
            let src_stump = self.find_set(src);
            let trg_stump = self.find_set(trg);


            if let (Some((idx1, s1)), Some((idx2, s2))) = (src_stump, trg_stump) {
                if s1 != s2 {
                    msa.push(e);
                    let min_idx = min(idx1, idx2);
                    let max_idx = max(idx1, idx2);

                    let mut new_medow = vec![];
                    new_medow.extend(s2);
                    new_medow.extend(s1);

                    self.forest.remove(max_idx);
                    self.forest.remove(min_idx);

                    self.forest.push(new_medow);
                }
            }
        }


        return msa;
    }

    fn find_set(&self, id: &'a NId) -> Option<(usize, &Vec<&'a NId>)> {
        for p @ (_, stump) in self.forest.iter().enumerate() {
            if stump.contains(&id) {
                return Some(p);
            }
        }
        return None;
    }
    fn fill_forest(&mut self) {
        self.forest = self.graph.nodes.keys().map(|nid| vec![nid]).collect();
    }


    fn merge(&mut self, idx1: usize, s1: &'a Vec<&NId>, idx2: usize, s2: &'a Vec<&NId>) {
        let min_idx = min(idx1, idx2);
        let max_idx = max(idx1, idx2);

        let mut new_medow = vec![];
        new_medow.extend(s2);
        new_medow.extend(s1);

        self.forest.remove(max_idx);
        self.forest.remove(min_idx);

        self.forest.push(new_medow);
    }

    pub fn new(graph: &'a DiGraph<NId, NL, EL>) -> Self {
        Self { graph, forest: vec![] }
    }
}

pub struct MSAHighlighter<'a, NId, EL>
    where NId: Eq + Hash,
{
    edges: Vec<Edge<'a, NId, EL>>,
    delegate: ToStringProcessor,
}

impl<'a, NId, EL> MSAHighlighter<'a, NId, EL>
    where NId: Eq + Hash,
{
    pub fn new(edges: Vec<Edge<'a, NId, EL>>) -> Self {
        Self {
            edges,
            delegate: ToStringProcessor {},
        }
    }
}

impl<'a, NId, NL, EL> DotProcessor<'a, NId, NL, EL> for MSAHighlighter<'a, NId, EL>
    where
        NId: ToString + Eq + Hash,
        NL: ToString,
        EL: ToString + PartialEq,
{
    fn node(&self, id: &'a NId, nl: &'a NL) -> Stmt {
        (&self.delegate as &dyn DotProcessor<NId, NL, EL>).node(id, nl)
    }

    fn edge(&self, from: &'a NId, to: &'a NId, el: &'a EL) -> Stmt {
        let edge = Edge::new(from, to, el);
        let green = EdgeAttributes::color(color_name::green);
        let bold = EdgeAttributes::penwidth(2.0);
        if self.edges.contains(&edge) {
            self.delegate.edge_with_attrs(from, to, el, vec![green, bold])
        } else {
            (&self.delegate as &dyn DotProcessor<NId, NL, EL>).edge(from, to, el)
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::analyzer::mst::{MinimumSpanningArborescence, MSAHighlighter};
    use crate::{digraph, extend_edges, extend_nodes};
    use crate::DiGraph;
    use crate::EmptyPayload;

    #[test]
    fn simple_smoke_test() {
        let graph = digraph!((&str,_,usize) => ["A", "B","C","D","E"] => {
            "A" => ("B", 2);
            "A" => ("C", 4);
            "B" => ("C", 1);
            "B" => ("D", 7);
            "C" => ("D", 3);
            "C" => ("E", 5);
            "D" => ("E", 6);
        });
        let mut d = MinimumSpanningArborescence::new(&graph);
        let edges = d.find();
        let _ = graph.visualize().to_dot_file("dots/msa.svg", MSAHighlighter::new(edges));
    }
}