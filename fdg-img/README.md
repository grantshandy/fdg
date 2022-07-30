# fdg-img

A simple SVG renderer for [`fdg-sim`](https://crates.io/crates/fdg-sim).

[![Latest version](https://img.shields.io/crates/v/fdg-img.svg)](https://crates.io/crates/fdg-img)
[![Documentation](https://docs.rs/fdg-img/badge.svg)](https://docs.rs/fdg-img)
[![GPL-3.0](https://img.shields.io/badge/license-GPL-blue.svg)](https://github.com/grantshandy/fdg/blob/main/fdg-img/LICENSE)

## Basic Example
```rust
use std::fs;

use fdg_sim::{ForceGraph, ForceGraphHelper, force};

fn main() {
    // initialize a graph
    let mut graph: ForceGraph<(), ()> = ForceGraph::default();

    // create a circle
    let nodes = 10;

    graph.add_force_node("0", ());
    for x in 1..nodes {
        graph.add_force_node(x.to_string(), ());
        graph.add_edge(x.into(), (x - 1).into(), ());
    }
    graph.add_edge(0.into(), (nodes - 1).into(), ());

    // generate svg text for your graph
    let svg = fdg_img::gen_image(&graph, &force::handy(45.0, 0.975, true, true), None).unwrap();

    // save the svg on disk
    fs::write("ring.svg", svg.as_bytes()).unwrap();
}
```

![screenshot](https://raw.githubusercontent.com/grantshandy/fdg/main/fdg-img/screenshots/ring.svg)

## Related Crates
- [`fdg-macroquad`](https://crates.io/crates/fdg-macroquad) A visualizer that uses [`macroquad`](https://crates.io/crates/macroquad) for real-time rendering ([view demo online](https://grantshandy.github.io/fdg)).
