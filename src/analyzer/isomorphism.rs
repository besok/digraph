use std::hash::Hash;
use crate::DiGraph;


/// VF2++
pub struct IsomorphismAnalyzer<'a, NidLhs, NidRhs, NLlhs, NLrhs, ELlhs, ELrhs>
    where
        NidLhs: Eq + Hash,
        NidRhs: Eq + Hash,
{
    pub(crate) lhs: &'a DiGraph<NidLhs, NLlhs, ELlhs>,
    pub(crate) rhs: &'a DiGraph<NidRhs, NLrhs, ELrhs>,
}

impl<'a, NidLhs, NidRhs, NLlhs, NLrhs, ELlhs, ELrhs>
IsomorphismAnalyzer<'a, NidLhs, NidRhs, NLlhs, NLrhs, ELlhs, ELrhs> where
    NidLhs: Eq + Hash,
    NidRhs: Eq + Hash,
{
    pub fn new(lhs: &'a DiGraph<NidLhs, NLlhs, ELlhs>, rhs: &'a DiGraph<NidRhs, NLrhs, ELrhs>) -> Self {
        Self { lhs, rhs }
    }

    pub fn test(&self) -> bool {
        return false;
    }


    /// check degrees
    fn could_be_iso(&self) -> bool {
        if self.lhs.nodes.len() != self.rhs.nodes.len() {
            false
        } else {
            let mut rhs_degree: Vec<usize> =
                self.rhs.edges.values().into_iter().map(|b| b.len()).collect();
            rhs_degree.sort_by(|a, b| b.cmp(a));

            let mut lhs_degree: Vec<usize> =
                self.lhs.edges.values().into_iter().map(|b| b.len()).collect();
            lhs_degree.sort_by(|a, b| b.cmp(a));

            lhs_degree == rhs_degree
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::DiGraph;
    use crate::EmptyPayload;
    use crate::{digraph, extend_edges, extend_nodes};
    use crate::analyzer::isomorphism::IsomorphismAnalyzer;

    #[test]
    fn smoke() {
        let lhs = digraph!((&str,_,_) => ["A","B","C","D"] => {
           "A" => ["B","C","D"];
           "B" => ["C","D"];
           "C" => "D";
        });
        let rhs = digraph!((usize,_,_) => [1,2,3,4] => {
           1 => [4,2,3];
           2 => [3,4];
           3 => 4;
        });
        assert!(IsomorphismAnalyzer::new(&lhs, &rhs).could_be_iso());

        let rhs = digraph!((usize,_,_) => [1,2,3,4] => {
           1 => [4,2];
           2 => [3,4];
           3 => 4;
        });
        assert!(!IsomorphismAnalyzer::new(&lhs, &rhs).could_be_iso());
    }
}
