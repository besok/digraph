use crate::{DiGraph, EmptyPayload};
use graphviz_rust::attributes::{EdgeAttributes, NodeAttributes};
use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use graphviz_rust::printer::{DotPrinter};
use std::hash::Hash;

/// The processor to visualize the nodes and edges of the graph to dot format
pub trait DotProcessor<'a, NId, NL, EL> {
    fn node(&self, id: &'a NId, nl: &'a NL) -> Stmt;
    fn edge(&self, from: &'a NId, to: &'a NId, el: &'a EL) -> Stmt;
}

pub trait ToStringOpt {
    fn to_string_opt(&self) -> Option<String>;
}

impl<T: ToString> ToStringOpt for T {
    fn to_string_opt(&self) -> Option<String> {
        match self.to_string().as_str() {
            e if e.is_empty() => None,
            e => Some(e.to_owned())
        }
    }
}

pub struct ToStringProcessor;

impl ToStringProcessor {
    pub fn node_with_attrs<'a, NId, NL>(
        &self,
        id: &'a NId,
        nl: &'a NL,
        attrs: Vec<Attribute>,
    ) -> Stmt
        where
            NId: ToStringOpt,
            NL: ToStringOpt,
    {
        let id = id.to_string_opt().expect("the node id should exist");
        let label = match nl.to_string_opt() {
            Some(label) => format!("\"{} {}\"", id, label),
            None => format!("\"{}\"", id),
        };
        let mut attrs = attrs;
        attrs.push(NodeAttributes::label(label));
        stmt!(node!(id.as_str(), attrs))
    }
    pub fn edge_with_attrs<'a, NId, EL>(
        &self,
        from: &'a NId,
        to: &'a NId,
        el: &'a EL,
        attrs: Vec<Attribute>,
    ) -> Stmt
        where
            NId: ToStringOpt,
            EL: ToStringOpt,
    {
        let from = format!(
            "{}",
            from.to_string_opt().expect("the from point should exist")
        );
        let to = format!("{}", to.to_string_opt().expect("the to point should exist"));
        let mut attrs = attrs;
        if let Some(label) = el.to_string_opt() {
            attrs.push(EdgeAttributes::label(label));
        }

        stmt!(edge!(node_id!(from.as_str()) => node_id!(to.as_str()), attrs))
    }
}

impl<'a, NId, NL, EL> DotProcessor<'a, NId, NL, EL> for ToStringProcessor
    where
        NId: ToStringOpt,
        NL: ToStringOpt,
        EL: ToStringOpt,
{
    fn node(&self, id: &'a NId, nl: &'a NL) -> Stmt {
        self.node_with_attrs(id, nl, vec![])
    }

    fn edge(&self, from: &'a NId, to: &'a NId, el: &'a EL) -> Stmt {
        self.edge_with_attrs(from, to, el, vec![])
    }
}

#[cfg(test)]
mod tests {
    use crate::visualizer::{vis, vis_to_file};
    use crate::DiGraph;
    use crate::EmptyPayload;
    use crate::*;
    use graphviz_rust::dot_structures::Graph;

    #[test]
    fn simple_viz_to_file_test() {
        let dot = digraph!(
            => [1,2,3,4,5,6,7,8,9,10] => {
             1 => [2,3,4];
             [2,3,4] => 5;
             [2,3,4] => 6;
             5 => 6;
             6 => [7,8];
             [7,8] => 9;
             9 => 10
            }
        )
            .visualize()
            .str_to_dot_file("dots/output.svg");
        println!("{:?}", dot)
    }

    #[test]
    fn simple_viz_to_file_payload_edge_test() {
        let dot = digraph!(
           (_,_,i32) => [1,2,3,4,5,6,7,8,9,10] => {
             1 => [2,3,4];
             [2,3,4] => (5,100);
             [2,3,4] => (6,10);
             5 => (6,1);
             6 => [(7,14),(8,14)];
             [7,8] => 9;
             9 => 10
            }
        )
            .visualize()
            .str_to_dot_file("dots/output.svg");
        println!("{:?}", dot)
    }

    #[test]
    fn simple_viz_to_file_str_edge_test() {
        let dot = digraph!(
           (&str,_,_) => ["company","employer","employee"] => {
                "employer" => "company";
                "company" => "employee"
            }
        )
            .visualize()
            .str_to_dot_file("dots/output.svg");

        println!("{:?}", dot)
    }
}
