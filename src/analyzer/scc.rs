/// https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm
use std::{
    cmp,
    collections::{HashMap, HashSet},
    hash::Hash,
};

use graphviz_rust::{
    attributes::{color_name, NodeAttributes},
    dot_structures::{Attribute, Stmt},
};

use crate::{
    visualizer::dot::{DotProcessor, ToStringProcessor},
    DiGraph,
};

pub struct TarjanSCC<'a, NId, NL, EL>
where
    NId: Eq + Hash,
{
    graph: &'a DiGraph<NId, NL, EL>,
    idx: usize,
    state: HashMap<&'a NId, Idx>,
    stack: Vec<&'a NId>,
    result: Vec<Vec<&'a NId>>,
}

impl<'a, NId, NL, EL> TarjanSCC<'a, NId, NL, EL>
where
    NId: Eq + Hash + Clone,
{
    pub fn new(graph: &'a DiGraph<NId, NL, EL>) -> Self {
        Self {
            graph,
            idx: Default::default(),
            state: Default::default(),
            stack: Default::default(),
            result: Default::default(),
        }
    }

    pub fn process_graph(&mut self) -> Vec<Vec<&'a NId>> {
        for id in self.graph.nodes.keys() {
            if !self.state.contains_key(id) {
                self.process_node(id)
            }
        }
        self.result.clone()
    }

    fn process_node(&mut self, id: &'a NId) {
        let idx = Idx {
            low_link: self.idx,
            index: self.idx,
            on_stack: true,
        };
        self.stack.push(id);
        self.state.insert(id, idx);
        self.idx += 1;

        for next in self.graph.successors_ids(id) {
            match self.state.get(next) {
                Some(idx) => {
                    if idx.on_stack {
                        self.set_low_link_or_idx(id, next);
                    }
                }
                None => {
                    self.process_node(next);
                    self.set_low_link(id, next);
                }
            }
        }

        if self.eq_idx_link(id) {
            let mut scc: Vec<_> = vec![];
            while let Some(curr) = self.stack.pop() {
                scc.push(curr);
                self.set_on_stack_false(curr);
                if curr == id {
                    self.result.push(scc);
                    return;
                }
            }
        }
    }

    fn set_on_stack_false(&mut self, id: &'a NId) {
        self.state.entry(id).and_modify(|idx| {
            *idx = Idx {
                on_stack: false,
                ..*idx
            };
        });
    }
    fn eq_idx_link(&self, src: &'a NId) -> bool {
        self.state
            .get(src)
            .map(|s| s.low_link == s.index)
            .expect("the src should be processed")
    }

    fn set_low_link_or_idx(&mut self, src: &'a NId, trg: &'a NId) {
        let src_link = self
            .state
            .get(src)
            .map(|s| s.low_link)
            .expect("the src should be processed");
        let trg_idx = self
            .state
            .get(trg)
            .map(|s| s.index)
            .expect("the trg should be processed");
        self.state.entry(src).and_modify(|idx| {
            *idx = Idx {
                low_link: cmp::min(src_link, trg_idx),
                ..*idx
            };
        });
    }

    fn set_low_link(&mut self, src: &'a NId, trg: &'a NId) {
        let src_link = self
            .state
            .get(src)
            .map(|s| s.low_link)
            .expect("the src should be processed");
        let trg_link = self
            .state
            .get(trg)
            .map(|s| s.low_link)
            .expect("the trg should be processed");
        self.state.entry(src).and_modify(|idx| {
            *idx = Idx {
                low_link: cmp::min(src_link, trg_link),
                ..*idx
            };
        });
    }
}

struct Idx {
    low_link: usize,
    index: usize,
    on_stack: bool,
}

struct TarjanSCCVizProcessor<'a, NId> {
    delegate: ToStringProcessor,
    groups: Vec<Vec<&'a NId>>,

    map: HashMap<&'a NId, usize>,
}

fn idx_to_color(idx: usize) -> Attribute {
    let mut idx = idx;

    if idx > 10 {
        idx = idx % 10
    }

    match idx {
        0 => NodeAttributes::color(color_name::aqua),
        1 => NodeAttributes::color(color_name::red),
        2 => NodeAttributes::color(color_name::blue),
        3 => NodeAttributes::color(color_name::green),
        4 => NodeAttributes::color(color_name::chocolate),
        5 => NodeAttributes::color(color_name::yellow),
        6 => NodeAttributes::color(color_name::palegreen),
        7 => NodeAttributes::color(color_name::purple),
        8 => NodeAttributes::color(color_name::aquamarine1),
        9 => NodeAttributes::color(color_name::bisque),
        10 => NodeAttributes::color(color_name::yellowgreen),
        _ => NodeAttributes::color(color_name::white),
    }
}

impl<'a, NId> TarjanSCCVizProcessor<'a, NId>
where
    NId: Eq + Hash,
{
    fn new(groups: Vec<Vec<&'a NId>>) -> Self {
        let mut map: HashMap<&NId, usize> = HashMap::new();
        for (i, elems) in groups.iter().enumerate() {
            for e in elems.iter() {
                map.insert(e, i);
            }
        }

        Self {
            delegate: ToStringProcessor,
            groups,
            map,
        }
    }
}

impl<'a, NId, EL, NL> DotProcessor<'a, NId, NL, EL> for TarjanSCCVizProcessor<'a, NId>
where
    NId: Eq + Hash + ToString,
    NL: ToString,
    EL: ToString,
{
    fn node(&self, id: &'a NId, nl: &'a NL) -> Stmt {
        let idx = self.map[id];
        let color = idx_to_color(self.map[id]);
        let label = NodeAttributes::xlabel(format!("{}", idx));
        let node = self.delegate.node_with_attrs(id, nl, vec![color, label]);
        node
    }

    fn edge(&self, from: &'a NId, to: &'a NId, el: &'a EL) -> Stmt {
        (&self.delegate as &dyn DotProcessor<NId, NL, EL>).edge(from, to, el)
    }
}

#[cfg(test)]
mod tests {
    use crate::generator;
    use crate::generator::ERCfg;
    use crate::generator::RGGenCfg;
    use crate::generator::RandomGraphGenerator;
    use crate::DiGraph;
    use crate::EmptyPayload;
    use crate::{digraph, extend_edges, extend_nodes};

    use super::TarjanSCC;
    use super::TarjanSCCVizProcessor;

    #[test]
    fn simple_test() {
        let graph = digraph!((usize,_,_) => [1,2,3,4,5,6,7,8] => {
           1 => [2];
           2 => 3;
           3 => [1,4];
           4 => [5,6];
           5 => [7,8];
           6 => [4,7];
        });
        let r = graph.visualize().str_to_dot_file("dots/graph.svg");
        let sccs = graph.scc();
        let r = graph
            .visualize()
            .to_dot_file("dots/graph_scc.svg", TarjanSCCVizProcessor::new(sccs));
    }

    #[test]
    fn gen_test() {
        let mut g = RandomGraphGenerator::new(RGGenCfg::ER(ERCfg {
            node_len: 20,
            edge_prob: 0.1,
            self_conn: false,
            back_strict: true,
            max_from: 0,
            max_to: 0,
        }));
        let graph = g.generate_usize(|_| 0, |lhs, rhs| lhs + rhs);
        let r = graph.visualize().str_to_dot_file("dots/graph.svg");
        let sccs = graph.scc();
        let r = graph
            .visualize()
            .to_dot_file("dots/graph_scc.svg", TarjanSCCVizProcessor::new(sccs));
    }
}
