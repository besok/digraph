//! The library allows creating and manipulating with directed graphs.
//!
//! # Description:
//! The main structure is `DiGraph` that is defined in terms of three types:
//!  - NId - id of the node. Should be unique and implement `Eq + Hash`
//!  - NL - node payload (`EmptyPayload`) by default
//!  - EL - edge payload (`EmptyPayload`) by default
//!
//! # Example of the struct:
//! ```rust
//! use digraph_rs::{DiGraph,EmptyPayload};
//! use std::collections::HashMap;
//! fn simple_creation_graph(){
//!   let mut graph:DiGraph<usize, EmptyPayload,EmptyPayload> = DiGraph::empty();
//!   graph.add_bare_node(1);  
//!   graph.add_bare_node(2);  
//!   graph.add_bare_node(3);
//!   graph.add_bare_node(4);
//!   
//!   graph.add_bare_edge(1, 2);
//!   graph.add_bare_edge(2, 3);
//!   graph.add_bare_edge(3, 4);
//!   graph.add_bare_edge(4, 1);
//!   assert_eq!(graph.start(), &Some(1));
//!        assert_eq!(
//!            graph.successors(&1),
//!            Some(&HashMap::from_iter(
//!                vec![(2usize, EmptyPayload)].into_iter()
//!           ))
//!       );
//!   
//!     
//! }
//! ```
//! # Modules
//!  - builder: the module allows creating graph using defined templates(macroses)
//!  - analyzer: the module allows performing a set of default algorithms  
//!  - visualizer: the module allows visualizing the graph and some extra information in graphviz format
//!  - generator: the module allows generating random graphs according to the different modules
//!  - iterator: a set of iterators over the graph
//! # Example with modules:
//! ```rust
//!  
//!    use digraph_rs::{DiGraph,EmptyPayload,digraph, extend_edges, extend_nodes,};
//!    use digraph_rs::generator::{RandomGraphGenerator,RGGenCfg,WSCfg};
//!    use digraph_rs::analyzer::dijkstra::{DijkstraPath, MinPathProcessor};
//!     #[test]
//!      fn complex_example() {
//!          let mut graph = digraph!((usize,_,usize) => [1,2,3,4,5,6,7,8] => {
//!            1 => [(2,3),(3,1),(4,2)];
//!            [2,3,4] => (5,2);
//!            5 => (6,1);
//!            6 => [(7,2),(8,3)];
//!          });
//!  
//!          let v_res = graph.visualize().str_to_dot_file("dots/graph.svg");
//!          assert!(v_res.is_ok());
//!  
//!          assert!(graph.analyze().edge(&1, &2).is_some());
//!          assert!(graph.analyze().edge(&1, &6).is_none());
//!  
//!          let mut path_finder = DijkstraPath::new(&graph);
//!          let paths = path_finder.on_edge(1);
//!          let trail = paths.trail(&8).unwrap();
//!          assert_eq!(trail, vec![1, 3, 5, 6, 8]);
//!          let r = graph
//!              .visualize()
//!              .to_dot_file("dots/graph_path_1_8.svg", MinPathProcessor::new(trail));
//!          assert!(r.is_ok());
//!       }
//!         
//!        fn simple_gen_test() {
//!           let mut ws_gen = RandomGraphGenerator::new(RGGenCfg::WS(WSCfg {
//!            node_len: 20,
//!            nearest_k: 4,
//!            rewire_prob: 0.5,
//!           }));
//!           let di = ws_gen.generate_usize(|_| 0, |lhs, rhs| 0);
//!           let r = di.visualize().str_to_dot_file("dots/gen_ws.svg");
//!           assert!(r.is_ok());
//!       }  
//! ```
//!

pub mod analyzer;
pub mod builder;
pub mod generator;
pub mod iterator;
pub mod visualizer;

use crate::analyzer::GraphAnalyzer;
use crate::visualizer::dot::*;
use crate::visualizer::{vis, vis_to_file};

use self::visualizer::DotGraphVisualizer;
use analyzer::dom::Dominators;
use analyzer::predecessors::Predecessors;
use analyzer::scc::TarjanSCC;
use graphviz_rust::dot_generator::{graph, id, node};
use graphviz_rust::dot_structures::{Graph, Id, Stmt};
use iterator::{NodeIteratorBF, NodeIteratorDF, NodeIteratorDFPostOrder, NodeIteratorPlain};
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Error, Formatter};
use std::hash::Hash;

#[derive(PartialEq,Eq)]
pub struct Edge<'a, NId, EL> where
    NId: Eq + Hash, {
    src: &'a NId,
    trg: &'a NId,
    payload: &'a EL,
}

impl<'a, NId, EL> Edge<'a, NId, EL> where
    NId: Eq + Hash, {
    pub fn new(src: &'a NId, trg: &'a NId, payload: &'a EL) -> Self {
        Self { src, trg, payload }
    }
}

/// The base structure denoting a directed graph with a start in the first added node.
///  - NId: id of the node. should be unique and implement `Eq + Hash`
///  - NL: payload for node
///  - EL: payload for edge
#[derive(Debug)]
pub struct DiGraph<NId, NL, EL>
    where
        NId: Eq + Hash,
{
    nodes: HashMap<NId, NL>,
    edges: HashMap<NId, HashMap<NId, EL>>,
    start: Option<NId>,
}

impl DiGraph<usize, EmptyPayload, EmptyPayload> {
    /// Default empty payload graph
    pub fn empty() -> Self {
        Self::new()
    }
}

impl<NId, NL, EL> DiGraph<NId, NL, EL>
    where
        NId: Clone + Eq + Hash,
{
    fn insert_new_node(&mut self, payload: NL, id: NId) -> NId {
        self.nodes.insert(id.clone(), payload);
        if self.start.is_none() {
            self.start = Some(id.clone())
        }

        id
    }

    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            start: None,
        }
    }
    /// Adds new node. If the given id is presented then it will be replaced with a new payload.
    /// Returns this id  
    fn add_node(&mut self, id: NId, payload: NL) -> Option<NId> {
        Some(self.insert_new_node(payload, id))
    }
    /// Removes a node.
    /// Returns a payload if the node is presented.
    ///
    /// **Note**: this operation should be accompanied by the remove_edge operations.  
    pub fn remove_node(&mut self, id: &NId) -> Option<NL> {
        self.nodes.remove(id)
    }

    /// Adds new edge. Returns prev.
    pub fn add_edge(&mut self, from: NId, to: NId, payload: EL) -> Option<EL> {
        self.edges.entry(from).or_default().insert(to, payload)
    }
    /// Removes edge.
    /// Returns a payload on the edge if it exists.
    pub fn remove_edge(&mut self, from: &NId, to: &NId) -> Option<EL> {
        self.edges.entry(from.clone()).or_default().remove(to)
    }

    /// Returns a reference to the successors.
    pub fn successors(&self, from: &NId) -> Option<&HashMap<NId, EL>> {
        self.edges.get(from)
    }

    /// Returns a reference to the successors ids.
    pub fn successor_ids(&self, from: &NId) -> Vec<&NId> {
        self.edges
            .get(from)
            .map(|m| m.keys().collect())
            .unwrap_or(vec![])
    }

    /// Returns a reference to a start node.
    pub fn start(&self) -> &Option<NId> {
        &self.start
    }

    /// Invokes a graph analyzer `GraphAnalyzer`
    pub fn analyze(&self) -> GraphAnalyzer<NId, NL, EL> {
        GraphAnalyzer { graph: &self }
    }

    /// Invokes a graph visualizer `DotGraphVisualizer`
    pub fn visualize(&self) -> DotGraphVisualizer<NId, NL, EL> {
        DotGraphVisualizer::new(self)
    }

    /// Returns an edge payload if exists
    pub fn edge(&self, from: &NId, to: &NId) -> Option<&EL> {
        self.edges.get(from).and_then(|tos| tos.get(to))
    }
    /// Returns a pair of id of node and node payload if exists
    pub fn node_by_id(&self, id: &NId) -> Option<(&NId, &NL)> {
        self.nodes.get_key_value(id)
    }

    pub fn iter(&self) -> NodeIteratorPlain<NId, NL> {
        NodeIteratorPlain::new(&self)
    }

    pub fn iter_df(&self) -> NodeIteratorDF<NId, NL, EL> {
        NodeIteratorDF::new(&self)
    }
    pub fn iter_df_post(&self) -> NodeIteratorDFPostOrder<NId, NL, EL> {
        NodeIteratorDFPostOrder::new(&self)
    }

    pub fn iter_bf(&self) -> NodeIteratorBF<NId, NL, EL> {
        NodeIteratorBF::new(&self)
    }

    pub fn predecessors(&self) -> Predecessors<NId> {
        Predecessors::new(&self)
    }
    pub fn dominators(&self) -> Dominators<NId> {
        Dominators::simple_fast(&self)
    }
    pub fn scc(&self) -> Vec<Vec<&NId>> {
        TarjanSCC::new(&self).process_graph()
    }

    /// Returns a list of edge references as a plain structure.
    pub fn edges(&self) -> Vec<Edge<NId, EL>> {
        let mut edges = vec![];

        for (nid, _) in &self.nodes {
            if let Some(ss) = self.successors(nid) {
                for (trg, el) in ss {
                    edges.push(Edge::new(nid, trg, el))
                }
            }
        }

        edges
    }
}

impl<NId, NL, EL> DiGraph<NId, NL, EL>
    where
        NId: Clone + Eq + Hash,
        NL: Default,
{
    /// Adds a node with an `EmptyPayload`
    pub fn add_bare_node(&mut self, id: NId) -> Option<NId> {
        self.add_node(id, Default::default())
    }
}

impl<NId, NL, EL> DiGraph<NId, NL, EL>
    where
        NId: Clone + Eq + Hash,
        EL: Default,
{
    /// Adds an edge with an `EmptyPayload`
    pub fn add_bare_edge(&mut self, from: NId, to: NId) -> Option<EL> {
        self.add_edge(from, to, Default::default())
    }
}

#[derive(Copy, Clone, PartialEq, Default)]
pub struct EmptyPayload;

impl ToString for EmptyPayload {
    fn to_string(&self) -> String {
        "".to_string()
    }
}

impl Debug for EmptyPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("")
    }
}
