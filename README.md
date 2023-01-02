# Digraph-rs

Sandbox for playing with digraphs.

## Description

A set of methods to handle and visualize the different approaches and algorithms in an appliance to directed graphs.

## Create

## What is inside

- Di Graph struct: `lib.rs`
- Graph builder: a set of macros to construct or extend graphs: `builder.rs`
- Graph visualization to dot format: `visualizer.rs`
- Dijkstra: `analyzer\dijkstra.rs`
- AStar: `analyzer\astar.rs`
- BFS:
  - search: `analyzer\fs.rs`
  - iterators: `iterator.rs`
- DFS
  - search: `analyzer\fs.rs`
  - iterators: `iterator.rs`
    - post-order
    - normal
- dominators: `analyzer\dom.rs`
  - simple fast
- Random graphs: `generator.rs`
  - Erdős-Rényi model
  - Watts Strogatz model
