use std::{collections::HashMap, vec};

use graphviz_rust::attributes::len;
use rand::{rngs::ThreadRng, Rng};

use super::{DiGraph, EmptyPayload};
use crate::digraph;
use std::hash::Hash;

/// Erdős-Rényi model
#[derive(Clone, Copy)]
pub struct ERCfg {
    pub node_len: usize,
    pub edge_prob: f64,
    pub self_conn: bool,
    pub back_strict: bool,
    pub max_from: usize,
    pub max_to: usize,
}

/// Watts Strogatz model
#[derive(Clone, Copy)]
pub struct WSCfg {
    pub node_len: usize,
    pub nearest_k: usize,
    pub rewire_prob: f64,
}

#[derive(Clone, Copy)]
pub enum RGGenCfg {
    ER(ERCfg),
    WS(WSCfg),
}

impl Default for RGGenCfg {
    fn default() -> Self {
        RGGenCfg::ER(ERCfg {
            node_len: 30,
            edge_prob: 0.1,
            self_conn: false,
            back_strict: true,
            max_from: 0,
            max_to: 0,
        })
    }
}
fn has_back_link<NId, NL, EL>(g: &DiGraph<NId, NL, EL>, from: &NId, to: &NId) -> bool
where
    NId: Clone + Eq + Hash,
{
    g.successors(to.clone())
        .map(|ss| ss.contains_key(from))
        .unwrap_or(false)
}

fn ws_generate<NId, NL, EL, FNId, FNL, FEL>(
    cfg: WSCfg,
    mut f_id: FNId,
    f_nl: FNL,
    f_el: FEL,
) -> DiGraph<NId, NL, EL>
where
    NId: Clone + Eq + Hash,
    EL: Clone,
    FNId: FnMut() -> NId,
    FNL: Fn(&NId) -> NL,
    FEL: Fn(&NId, &NId) -> EL,
{
    let mut g = digraph!(NId, NL, EL);
    let WSCfg {
        node_len,
        nearest_k,
        rewire_prob,
    } = cfg;
    let mut rand = rand::thread_rng();
    let nsize = nearest_k / 2;
    assert!(
        node_len > nsize,
        "the node len {} should be greater then nearest_k / 2: {}",
        node_len,
        nsize
    );

    let mut ids = vec![];
    let mut ring: HashMap<NId, Vec<(NId, EL)>> = HashMap::new();
    for _ in 0..node_len {
        let id = f_id();
        let nl = f_nl(&id);
        g.add_node(id.clone(), nl);
        ids.push(id.clone());
    }

    let l = ids.len();
    for (idx, from) in ids.iter().enumerate() {
        let mut ring_edges = vec![];
        for r in 1..=nsize {
            let lhs_idx = if idx < r { l - r } else { idx - r };
            let rhs_idx = if idx + r > l { r } else { idx + r };

            if let Some(to) = ids.get(lhs_idx) {
                if !has_back_link(&g, from, to) {
                    let payload = f_el(from, to);
                    ring_edges.push((to.clone(), payload));
                }
            }

            if let Some(to) = ids.get(rhs_idx) {
                if !has_back_link(&g, from, to) {
                    let payload = f_el(from, to);
                    ring_edges.push((to.clone(), payload));
                }
            }
        }
        ring.insert(from.clone(), ring_edges);
    }

    for from in ids.iter() {
        if let Some(edges) = ring.remove(from) {
            let edges_nodes: Vec<NId> = edges.iter().map(|(id, _)| id.clone()).collect();
            for (to, pl) in edges.into_iter() {
                let should_replace = rand.gen_bool(rewire_prob);
                if !should_replace {
                    g.add_edge(from.clone(), to, pl);
                } else {
                    let mut rand_id = rand.gen_range(0..l);
                    let mut rand_node = ids.get(rand_id).unwrap();
                    while rand_node == from || edges_nodes.contains(rand_node) {
                        rand_id = rand.gen_range(0..l);
                        rand_node = ids.get(rand_id).unwrap();
                    }
                    g.add_edge(from.clone(), rand_node.clone(), pl);
                }
            }
        }
    }
    g
}

fn er_generate<NId, NL, EL, FNId, FNL, FEL>(
    cfg: ERCfg,
    mut f_id: FNId,
    f_nl: FNL,
    f_el: FEL,
) -> DiGraph<NId, NL, EL>
where
    NId: Clone + Eq + Hash,
    EL: Clone,
    FNId: FnMut() -> NId,
    FNL: Fn(&NId) -> NL,
    FEL: Fn(&NId, &NId) -> EL,
{
    let mut g = digraph!(NId, NL, EL);
    let mut rand = rand::thread_rng();
    let ERCfg {
        node_len,
        edge_prob,
        self_conn,
        back_strict,
        max_from,
        max_to,
    } = cfg;

    let mut ids_counters = HashMap::new();
    let mut ids = vec![];
    for _ in 0..node_len {
        let id = f_id();
        let nl = f_nl(&id);
        g.add_node(id.clone(), nl);
        ids.push(id.clone());
        ids_counters.insert(id.clone(), (0usize, 0usize));
    }
    for from in ids.iter() {
        for to in ids.iter() {
            let max_bounds = max_from != 0
                && ids_counters
                    .get(from)
                    .map(|(v, _)| v >= &max_from)
                    .unwrap_or(false)
                || max_to != 0
                    && ids_counters
                        .get(to)
                        .map(|(_, v)| v >= &max_to)
                        .unwrap_or(false);

            if !max_bounds {
                let should_gen = if !self_conn && from == to {
                    false
                } else {
                    rand.gen_bool(edge_prob)
                };
                if should_gen {
                    if !back_strict || !has_back_link(&g, from, to) {
                        ids_counters.entry(from.clone()).and_modify(|v| {
                            *v = (v.0 + 1, v.1);
                        });
                        ids_counters.entry(to.clone()).and_modify(|v| {
                            *v = (v.0, v.1 + 1);
                        });
                        let el = f_el(from, to);
                        g.add_edge(from.clone(), to.clone(), el);
                    }
                }
            }
        }
    }
    g
}

pub struct RandomGraphGenerator {
    cfg: RGGenCfg,
}

impl RandomGraphGenerator {
    pub fn generate_empty(&mut self) -> DiGraph<usize, EmptyPayload, EmptyPayload> {
        self.generate_usize(|_| EmptyPayload {}, |_, _| EmptyPayload {})
    }

    pub fn generate_usize<NL, EL, FNL, FEL>(
        &mut self,
        f_nl: FNL,
        f_el: FEL,
    ) -> DiGraph<usize, NL, EL>
    where
        FNL: Fn(&usize) -> NL,
        EL: Clone,
        FEL: Fn(&usize, &usize) -> EL,
    {
        let len = match self.cfg {
            RGGenCfg::ER(ERCfg { node_len, .. }) | RGGenCfg::WS(WSCfg { node_len, .. }) => node_len,
        };
        let mut r = 0..len;
        self.generate(move || r.next().unwrap(), f_nl, f_el)
    }
}

impl Default for RandomGraphGenerator {
    fn default() -> Self {
        Self {
            cfg: Default::default(),
        }
    }
}

impl RandomGraphGenerator {
    pub fn new(cfg: RGGenCfg) -> Self {
        Self { cfg }
    }

    pub fn generate<NId, NL, EL, FNId, FNL, FEL>(
        &mut self,
        mut f_id: FNId,
        f_nl: FNL,
        f_el: FEL,
    ) -> DiGraph<NId, NL, EL>
    where
        NId: Clone + Eq + Hash,
        EL: Clone,
        FNId: FnMut() -> NId,
        FNL: Fn(&NId) -> NL,
        FEL: Fn(&NId, &NId) -> EL,
    {
        match self.cfg {
            RGGenCfg::WS(cfg) => ws_generate(cfg, f_id, f_nl, f_el),
            RGGenCfg::ER(cfg) => er_generate(cfg, f_id, f_nl, f_el),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::generator::{ERCfg, RGGenCfg, WSCfg};

    use super::RandomGraphGenerator;

    #[test]
    fn simple_gen_test() {
        let mut g = RandomGraphGenerator::default();
        let di = g.generate_empty();

        let r = di.visualize().str_to_dot_file("dots/gen.svg");
        assert!(r.is_ok());
    }
    #[test]
    fn simple_gen_load_test() {
        let mut g = RandomGraphGenerator::new(RGGenCfg::ER(ERCfg {
            node_len: 30,
            edge_prob: 0.1,
            self_conn: false,
            back_strict: true,
            max_from: 0,
            max_to: 0,
        }));
        let di = g.generate_usize(|_| 0, |lhs, rhs| lhs + rhs);

        let r = di.visualize().str_to_dot_file("dots/gen_load.svg");
        assert!(r.is_ok());
    }
    #[test]
    fn simple_gen_sw_test() {
        let mut g = RandomGraphGenerator::new(RGGenCfg::WS(WSCfg {
            node_len: 20,
            nearest_k: 4,
            rewire_prob: 0.5,
        }));
        let di = g.generate_usize(|_| 0, |lhs, rhs| lhs + rhs);

        let r = di.visualize().str_to_dot_file("dots/gen_load.svg");
        assert!(r.is_ok());
    }

    #[test]
    fn simple_gen_both_test() {
        let mut ws_gen = RandomGraphGenerator::new(RGGenCfg::WS(WSCfg {
            node_len: 20,
            nearest_k: 4,
            rewire_prob: 0.5,
        }));
        let di = ws_gen.generate_usize(|_| 0, |lhs, rhs| lhs + rhs);
        let r = di.visualize().str_to_dot_file("dots/gen_ws.svg");
        assert!(r.is_ok());

        let mut er_gen = RandomGraphGenerator::new(RGGenCfg::ER(ERCfg {
            node_len: 20,
            edge_prob: 0.1,
            self_conn: false,
            back_strict: true,
            max_from: 0,
            max_to: 0,
        }));
        let di = er_gen.generate_usize(|_| 0, |lhs, rhs| lhs + rhs);
        let r = di.visualize().str_to_dot_file("dots/gen_er.svg");
        assert!(r.is_ok());
    }
}
