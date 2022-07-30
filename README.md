# fdg (Force Directed Graph)
A Force Directed Graph Framework for Rust.

| Name | Version | Docs | License
|------|---------|------|------|
| `fdg-sim` | [![Latest version](https://img.shields.io/crates/v/fdg-sim.svg)](https://crates.io/crates/fdg-sim) | [![Documentation](https://docs.rs/fdg-sim/badge.svg)](https://docs.rs/fdg-sim) | [![GPL-3.0](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/grantshandy/fdg/blob/main/fdg-sim/LICENSE) |
| `fdg-macroquad` | [![Latest version](https://img.shields.io/crates/v/fdg-macroquad.svg)](https://crates.io/crates/fdg-macroquad) | [![Documentation](https://docs.rs/fdg-macroquad/badge.svg)](https://docs.rs/fdg-macroquad) | [![GPL-3.0](https://img.shields.io/badge/license-GPL-blue.svg)](https://github.com/grantshandy/fdg/blob/main/fdg-macroquad/LICENSE) |
| `fdg-img` | [![Latest version](https://img.shields.io/crates/v/fdg-img.svg)](https://crates.io/crates/fdg-img) | [![Documentation](https://docs.rs/fdg-img/badge.svg)](https://docs.rs/fdg-img) | [![GPL-3.0](https://img.shields.io/badge/license-GPL-blue.svg)](https://github.com/grantshandy/fdg/blob/main/fdg-img/LICENSE) |

![2D example](https://raw.githubusercontent.com/grantshandy/fdg/main/fdg-macroquad/screenshots/screenshot-2D.png)
![3D example](https://raw.githubusercontent.com/grantshandy/fdg/main/fdg-macroquad/screenshots/screenshot-3D.png)

The goal of this project is to provide a force-directed graph framework and algorithms for Rust, as well as 2D and 3D visualizers that work on the web and desktop. It sits on top of [`petgraph`](https://crates.io/crates/petgraph) and manages the positions of your nodes.

[View Examples Online](https://grantshandy.github.io/fdg)

## Contents
- [`/fdg-sim`](./fdg-sim/) The underlying force simulation framework. Handles your dataset's positions based on a physics engine of your choice (or creation).
- [`/fdg-macroquad`](./fdg-macroquad/) A visualizer that uses [`macroquad`](https://crates.io/crates/macroquad) for rendering.
- [`/fdg-img`](./fdg-img/) A SVG visualizer for your graphs.

## Resources
- [Force-Directed Graphs on Wikipedia](https://en.wikipedia.org/wiki/Force-directed_graph_drawing)
- [Force Directed Drawing Algorithms (Kobourov)](https://cs.brown.edu/people/rtamassi/gdhandbook/chapters/force-directed.pdf)
- Example javascript force directed graph layout engines [d3-force](https://github.com/d3/d3-force) and [ngraph.forcelayout](https://github.com/anvaka/ngraph.forcelayout)
