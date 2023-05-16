# Amandas-Labyrinth

To escape from Mr. Puzzle, Amanda has to find a way out of the binary labyrinth.

To solve this puzzle, Amanda will need to use her logical and analytical skills to explore the maze, make informed decisions, and navigate her way towards the exit. Depending on the complexity of the maze and the level of difficulty, Amanda may need to apply more advanced techniques or algorithms, such as graph theory, search algorithms, or optimization techniques, to solve the puzzle efficiently.

This repository contains an implementation of the Breadth-First Search (BFS) algorithm in Rust, leveraging parallel programming techniques for enhanced performance. The BFS algorithm is a fundamental graph traversal algorithm used to explore and discover all the nodes in a graph, starting from a given source node.

## Features

+ **Rust**: The implementation is written in Rust, a powerful and modern systems programming language known for its safety, performance, and expressive syntax.
+ **Parallel Programming**: The BFS algorithm is parallelized to leverage the full potential of modern multi-core processors, enabling faster exploration of large graphs.
+ **Efficient Graph Traversal**: The algorithm efficiently traverses the graph in a breadth-first manner, ensuring that nodes at the same level are explored before moving to deeper levels.
+ **Multi-threading**: The parallel implementation utilizes multi-threading to distribute the workload across multiple threads, maximizing computational resources and reducing execution time.
+ **Performance Analysis**: Benchmarking tools and techniques are included to evaluate the performance of the parallel BFS implementation and compare it to sequential versions.

## Setup

How to run:

```bash
cd amandas-labyrinth
cargo build
cargo run main.rs
```