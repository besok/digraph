use std::collections::HashSet;
use std::convert::identity;
use std::hash::Hash;
use std::ops::Add;
use crate::analyzer::dijkstra::MinPath;
use crate::analyzer::disjoint::DisjointSet;
use crate::{DiGraph, Edge};

// Kruskal's Algorithm:
//
// function findMinimumSpanningTree(graph):
// edges = sortEdges(graph.edges)  // Sort edges in non-decreasing order of weight
// result = []  // Initialize an empty list to store the minimum spanning tree
//
// // Create a disjoint set for each vertex
// sets = createDisjointSets(graph.vertices)
//
// for edge in edges:
// // Find the sets of the two vertices of the current edge
// set1 = findSet(sets, edge.vertex1)
// set2 = findSet(sets, edge.vertex2)
//
// // If the two vertices are in different sets, add the edge to the result
// // and merge the two sets
// if set1 != set2:
// result.append(edge)
// unionSets(sets, set1, set2)
//
// return result
//
// // Helper function to create disjoint sets for each vertex
// function createDisjointSets(vertices):
// sets = []
// for vertex in vertices:
// sets.append(makeSet(vertex))
// return sets
//
// // Helper function to create a singleton set for a vertex
// function makeSet(vertex):
// return [vertex]
//
// // Helper function to find the set to which a vertex belongs
// function findSet(sets, vertex):
// for set in sets:
// if vertex in set:
// return set
//
// // Helper function to merge two sets
// function unionSets(sets, set1, set2):
// sets.remove(set1)
// sets.remove(set2)
// newSet = set1 + set2
// sets.append(newSet)
//
// // Helper function to sort edges in non-decreasing order of weight
// function sortEdges(edges):
// return sort(edges)  // Implement a so

#[derive(Debug)]
pub struct MinimumSpanningTree<'a, NId, NL, EL>
    where
        NId: Eq + Hash + Clone,
        EL: Ord
{
    graph: &'a DiGraph<NId, NL, EL>,
}

impl<'a, NId, NL, EL> MinimumSpanningTree<'a, NId, NL, EL> where
    NId: Eq + Hash + Clone,
    EL: Ord
{
    fn find(&self) -> Vec<Edge<'a, NId, EL>> {
        let mut mst = vec![];
        let mut edges = self.graph.edges();
        let mut sets:HashSet<_> = self.graph.nodes.keys().map(|nid| {
            let mut set = HashSet::new();
            set.insert(nid);
            set
        }).collect();


        edges.sort_by_key(|e| e.payload);

        for Edge { src, trg, payload } in edges {

        }


        return mst;
    }

    pub fn new(graph: &'a DiGraph<NId, NL, EL>) -> Self {
        Self { graph }
    }
}

#[cfg(test)]
mod tests {
    use crate::analyzer::mst::MinimumSpanningTree;
    use crate::{digraph, extend_edges, extend_nodes};
    use crate::DiGraph;
    use crate::EmptyPayload;

    #[test]
    fn simple_smoke_test() {
        let graph = digraph!((usize,_,usize) => [1,2,3,4,5,6,7,8,9,10,11,] => {
           1 => [(2,1),(3,1)];
           2 => (4,2);
           3 => (5,3);
           [4,5] => (6,1);
           6 => (7,1);
           7 => [(8,1),(9,2),(10,3)];
           [8,9,10] => (11,1)

        });
        let mut d = MinimumSpanningTree::new(&graph);
        let d = d.find();
    }
}