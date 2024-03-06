# Digraph-rs

A set of algorithms on directed graphs. 
It can be used as in the educational purposes so in the applications also. 

## Description

A set of methods to handle and visualize the different approaches and algorithms in an appliance to directed graphs.

## Base structures

- Di Graph struct: `lib.rs`
- Graph builder: a set of macros to construct or extend graphs: `builder.rs`
- Graph visualization to dot format: `visualizer.rs`

## Iterators

- BFS:
  - search: `analyzer\fs.rs`
  - iterators: `iterator.rs`
- DFS
  - search: `analyzer\fs.rs`
  - iterators: `iterator.rs`
    - post-order
    - normal

## Algorithms 

- Dijkstra: `analyzer\dijkstra.rs`
- AStar: `analyzer\astar.rs`
- dominators: `analyzer\dom.rs`
  - simple fast
- strongly connected components(Tarjan)
- disjoint set
- Bipartite graph
- minimum spanning Arborescence (Kruskal's algorithm)

## Random graph generators

- Random graphs: `generator.rs`
  - Erdős-Rényi model
  - Watts Strogatz model
