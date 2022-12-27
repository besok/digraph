use std::{collections::HashMap, vec};

use graphviz_rust::attributes::len;
use rand::{rngs::ThreadRng, Rng};

use super::{DiGraph, EmptyPayload};
use crate::digraph;
use std::hash::Hash;

/// Erdős-Rényi model
pub struct ERCfg {
    pub node_len: usize,
    pub edge_prob: f64,
    pub self_conn: bool,
    pub strict: bool,
    pub max_from: usize,
    pub max_to: usize,
}

pub enum RGGenCfg {
    ER(ERCfg),
}

impl Default for RGGenCfg {
    fn default() -> Self {
        RGGenCfg::ER(ERCfg {
            node_len: 30,
            edge_prob: 0.1,
            self_conn: false,
            strict: true,
            max_from: 0,
            max_to: 0,
        })
    }
}

pub struct RandomGraphGenerator {
    cfg: RGGenCfg,
    random: ThreadRng,
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
        FEL: Fn(&usize, &usize) -> EL,
    {
        let len = match self.cfg {
            RGGenCfg::ER(ERCfg { node_len, .. }) => node_len,
        };
        let mut r = 0..len;
        self.generate(move || r.next().unwrap(), f_nl, f_el)
    }
}

impl Default for RandomGraphGenerator {
    fn default() -> Self {
        Self {
            cfg: Default::default(),
            random: rand::thread_rng(),
        }
    }
}

impl RandomGraphGenerator {
    pub fn new(cfg: RGGenCfg) -> Self {
        Self {
            cfg,
            random: rand::thread_rng(),
        }
    }

    fn generate<NId, NL, EL, FNId, FNL, FEL>(
        &mut self,
        mut f_id: FNId,
        f_nl: FNL,
        f_el: FEL,
    ) -> DiGraph<NId, NL, EL>
    where
        NId: Clone + Eq + Hash,
        FNId: FnMut() -> NId,
        FNL: Fn(&NId) -> NL,
        FEL: Fn(&NId, &NId) -> EL,
    {
        let mut g = digraph!(NId, NL, EL);

        match self.cfg {
            RGGenCfg::ER(ERCfg {
                node_len,
                edge_prob,
                self_conn,
                strict,
                max_from,
                max_to,
            }) => {
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
                                self.random.gen_bool(edge_prob)
                            };
                            if should_gen {
                                let back_link = g
                                    .successors(to.clone())
                                    .map(|ss| ss.contains_key(from))
                                    .unwrap_or(false);

                                if strict && back_link {
                                } else {
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
            }
        }

        g
    }
}

#[cfg(test)]
pub mod tests {
    use crate::graph::generator::{ERCfg, RGGenCfg};

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
            strict: true,
            max_from: 0,
            max_to: 0,
        }));
        let di = g.generate_usize(|_| 0, |lhs, rhs| lhs + rhs);

        let r = di.visualize().str_to_dot_file("dots/gen_load.svg");
        assert!(r.is_ok());
    }
}
