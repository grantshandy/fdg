# fdg-img

A visualizer for [`fdg-sim`](https://github.com/grantshandy/fdg/) that generates simple SVG images.

![screenshot](https://raw.githubusercontent.com/grantshandy/fdg/main/fdg-img/screenshots/json.svg)

## Basic Example
```rust
use fdg_sim::{ForceGraph, ForceGraphHelper, Simulation, SimulationParameters};

fn main() {
    // initialize a graph
    let mut graph: ForceGraph<(), ()> = ForceGraph::default();

    let one = graph.add_force_node("one", ());
    let two = graph.add_force_node("two", ());
    let _three = graph.add_force_node("three", ());
    graph.add_edge(one, two, ());

    // generate svg text for your graph
    let svg = fdg_img::gen_image(&graph, &force::handy(45.0, 0.975, true, true), None).unwrap();

    // save the svg on disk
    fs::write("basic.svg", svg.as_bytes()).unwrap();
}
```

## Related Crates
- `/fdg-macroquad` A visualizer that uses `macroquad` for real-time rendering ([View Demo Online](https://grantshandy.github.io/fdg)).
