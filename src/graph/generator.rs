use rand::{rngs::ThreadRng, Rng};

use crate::{digraph, extend_nodes};

use super::{DiGraph, EmptyPayload};

trait GenRestriction<NId, NL, EL> {
    fn node(nid: &NId, nl: &NL) -> bool;
    fn edge(from: &NId, to: &NId, el: &EL) -> bool;
}

pub struct GenCfg {
    len: usize,
    max_conn: usize,
    allow_cycles: bool,
}

impl Default for GenCfg {
    fn default() -> Self {
        Self {
            len: 50,
            max_conn: 2,
            allow_cycles: true,
        }
    }
}

pub struct GraphGenerator {
    cfg: GenCfg,
    random: ThreadRng,
}
impl Default for GraphGenerator {
    fn default() -> Self {
        Self {
            cfg: Default::default(),
            random: rand::thread_rng(),
        }
    }
}

impl GraphGenerator {
    pub fn new(cfg: GenCfg) -> Self {
        Self {
            cfg,
            random: rand::thread_rng(),
        }
    }

    pub fn generate(&mut self) -> DiGraph<usize, EmptyPayload, EmptyPayload> {
        let mut g = digraph!();

        let len = self.cfg.len;
        let mut ids = vec![];

        for id in 0..len.clone() {
            g.add_bare_node(id);
            ids.push(id);
        }
        let max_conn = self.cfg.max_conn;
        for from in ids.into_iter() {
            let current_size = self.random.gen_range(1..max_conn);
            for _ in 0..current_size {
                let to = self.random.gen_range(0..len.clone());
                g.add_bare_edge(from.clone(), to);
            }
        }
        g
    }
}

#[cfg(test)]
pub mod tests {
    use super::GraphGenerator;

    #[test]
    fn simple_gen_test() {
        let mut g = GraphGenerator::default();
        let di = g.generate();

        let r = di.visualize().str_to_dot_file("dots/gen.svg");
        assert!(r.is_ok());
    }
}
