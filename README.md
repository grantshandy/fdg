# fdg
A Force-Directed Graph Framework for Rust.

![screenshot](https://raw.githubusercontent.com/grantshandy/fdg/main/fdg-macroquad/screenshots/screenshot.png)

[View Examples Online](https://grantshandy.github.io/fdg/)

The goal of this project is to provide a [force-directed graph drawing](https://en.wikipedia.org/wiki/Force-directed_graph_drawing) framework for Rust, as well as 2D and 3D visualizers/renderers that work on the web and desktop. It's built on top of the popular [`petgraph`](https://crates.io/crates/petgraph) Rust library for interaction with already existing datasets in Rust.

**In the simplest terms**, the crates in this project allow you to take a [graph](https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)) and turn it into a really pretty picture.

## Crates
| Name            | Version                                                                                                        | Docs                                                                                       | License                                                                                                                             |
|-----------------|----------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------|
| `fdg-sim`       | [![Latest version](https://img.shields.io/crates/v/fdg-sim.svg)](https://crates.io/crates/fdg-sim)             | [![Documentation](https://docs.rs/fdg-sim/badge.svg)](https://docs.rs/fdg-sim)             | [![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/grantshandy/fdg/blob/main/fdg-sim/LICENSE)       |
| `fdg-macroquad` | [![Latest version](https://img.shields.io/crates/v/fdg-macroquad.svg)](https://crates.io/crates/fdg-macroquad) | [![Documentation](https://docs.rs/fdg-macroquad/badge.svg)](https://docs.rs/fdg-macroquad) | [![GPL-3.0](https://img.shields.io/badge/license-GPL-blue.svg)](https://github.com/grantshandy/fdg/blob/main/fdg-macroquad/LICENSE) |
| `fdg-img`       | [![Latest version](https://img.shields.io/crates/v/fdg-img.svg)](https://crates.io/crates/fdg-img)             | [![Documentation](https://docs.rs/fdg-img/badge.svg)](https://docs.rs/fdg-img)             | [![GPL-3.0](https://img.shields.io/badge/license-GPL-blue.svg)](https://github.com/grantshandy/fdg/blob/main/fdg-img/LICENSE)       |
| `fdg-wasm`      | [![NPM Package](https://img.shields.io/npm/v/fdg-wasm)](https://www.npmjs.com/package/fdg-wasm)                | [![View Readme](https://docs.rs/fdg-sim/badge.svg)](https://github.com/grantshandy/fdg/tree/main/fdg-wasm#forcegraphsimulation-documentation) | [![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/grantshandy/fdg/blob/main/fdg-sim/LICENSE)       |

## Contents
- [`/fdg-sim`](./fdg-sim/README.md) The underlying force simulation. It handles your dataset's positions based on a layout engine of your choice (or creation).
- [`/fdg-macroquad`](./fdg-macroquad/README.md) A demo visualizer that uses [`macroquad`](https://crates.io/crates/macroquad) for its rendering ([view online](https://grantshandy.github.io/fdg/)).
- [`/fdg-img`](./fdg-img/README.md) A simple SVG renderer for your graphs.
- [`/fdg-wasm`](./fdg-wasm/README.md) A simple Webassembly wrapper of `fdg-sim` for use in Javascript.

## Resources
- [Force-Directed Graphs on Wikipedia](https://en.wikipedia.org/wiki/Force-directed_graph_drawing)
- [Force Directed Drawing Algorithms (Kobourov)](https://cs.brown.edu/people/rtamassi/gdhandbook/chapters/force-directed.pdf)
- Example Javascript force directed graph layout engines [d3-force](https://github.com/d3/d3-force) and [ngraph.forcelayout](https://github.com/anvaka/ngraph.forcelayout)