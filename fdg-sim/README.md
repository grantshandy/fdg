# fdg-sim
A [force-directed graph](https://en.wikipedia.org/wiki/Force-directed_graph_drawing) simulation for Rust.

[**Visit the project page for more information.**](https://github.com/grantshandy/fdg)

![screenshot](https://raw.githubusercontent.com/grantshandy/fdg/main/fdg-macroquad/screenshots/screenshot.png)

## Crates
| Name                                          | Version                                                                                                        | Docs                                                                                                                                          | License                                                                                                                             | Description                                                                                                               |
|-----------------------------------------------|----------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------|
| [`fdg-sim`](./fdg-sim/README.md)             | [![Latest version](https://img.shields.io/crates/v/fdg-sim.svg)](https://crates.io/crates/fdg-sim)             | [![Documentation](https://docs.rs/fdg-sim/badge.svg)](https://docs.rs/fdg-sim)                                                                | [![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/grantshandy/fdg/blob/main/fdg-sim/LICENSE)           | Runs the layout engine (simulation) and manages the position of nodes.                                                    |
| [`fdg-macroquad`](./fdg-macroquad/README.md) | [![Latest version](https://img.shields.io/crates/v/fdg-macroquad.svg)](https://crates.io/crates/fdg-macroquad) | [![Documentation](https://docs.rs/fdg-macroquad/badge.svg)](https://docs.rs/fdg-macroquad)                                                    | [![GPL-3.0](https://img.shields.io/badge/license-GPL-blue.svg)](https://github.com/grantshandy/fdg/blob/main/fdg-macroquad/LICENSE) | A demo visualizer that lets you interact with the graph in real time. ([View Online](https://grantshandy.github.io/fdg/)) |
| [`fdg-img`](./fdg-img/README.md)             | [![Latest version](https://img.shields.io/crates/v/fdg-img.svg)](https://crates.io/crates/fdg-img)             | [![Documentation](https://docs.rs/fdg-img/badge.svg)](https://docs.rs/fdg-img)                                                                | [![GPL-3.0](https://img.shields.io/badge/license-GPL-blue.svg)](https://github.com/grantshandy/fdg/blob/main/fdg-img/LICENSE)       | A simple SVG renderer for your graphs.                                                                                    |
| [`fdg-wasm`](./fdg-wasm/README.md)           | [![NPM Package](https://img.shields.io/npm/v/fdg-wasm)](https://www.npmjs.com/package/fdg-wasm)                | [![View Readme](https://docs.rs/fdg-sim/badge.svg)](https://github.com/grantshandy/fdg/tree/main/fdg-wasm#forcegraphsimulation-documentation) | [![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/grantshandy/fdg/blob/main/fdg-sim/LICENSE)           | A simple Webassembly wrapper of `fdg-sim` for use in Javascript.                                                          |

## Basic Example
```rust
use fdg_sim::{ForceGraph, ForceGraphHelper, Simulation, SimulationParameters};

fn main() {
    // initialize a graph
    let mut graph: ForceGraph<(), ()> = ForceGraph::default();

    // add nodes to it
    let one = graph.add_force_node("one", ());
    let two = graph.add_force_node("two", ());
    let _three = graph.add_force_node("three", ());
    graph.add_edge(one, two, ());

    // create a simulation from the graph
    let mut simulation = Simulation::from_graph(graph, SimulationParameters::default());

    // your event/render loop
    for frame in 0..50 {
        // update the nodes positions based on force algorithm
        simulation.update(0.035);

        // render (print) your nodes new locations.
        println!("---- frame {frame} ----");
        for node in simulation.get_graph().node_weights() {
            println!("\"{}\" - {:?}", node.name, node.location);
        }
        println!("-----------------------")
    }
}
```

## What are `N`, `E`, and `Ty`?
You may notice that structs and types like `Simulation`, `ForceGraph`, and `Force` have generic type parameters `<N, E, Ty>`.
 - `N`: The node weight (data stored in the `Node`'s `data`).
 - `E`: The edge weight (data stored directly in the graph's edges).
 - `Ty`: The edge type, `Directed` or `Undirected` (set by default).

These type names from the petgraph documentation [here](https://docs.rs/petgraph/0.6.2/petgraph/#generic-parameters). Because `Ty` is set by default, you won't have to mess with it most of the time.