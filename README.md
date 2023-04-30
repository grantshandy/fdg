# fdg
A Force-Directed Graph Library for Rust.

![screenshot](https://raw.githubusercontent.com/grantshandy/fdg/main/fdg-macroquad/screenshots/screenshot.png)

[View Demo Online](https://grantshandy.github.io/fdg/)

The goal of this project is to provide a [force-directed graph drawing](https://en.wikipedia.org/wiki/Force-directed_graph_drawing) framework for Rust, as well as 2D and 3D visualizers/renderers that work on the web and desktop. It's built on top of the popular [`petgraph`](https://crates.io/crates/petgraph) Rust library for interaction with already existing datasets in Rust.

**In the simplest terms**, the crates in this project allow you to take a [graph](https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)) and turn it into a pretty picture.

## Crates
| Name                                          | Version                                                                                                        | Docs                                                                                                                                          | License                                                                                                                             | Description                                                                                                               |
|-----------------------------------------------|----------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------|
| [`fdg-sim`](./fdg-sim/README.md)             | [![Latest version](https://img.shields.io/crates/v/fdg-sim.svg)](https://crates.io/crates/fdg-sim)             | [![Documentation](https://docs.rs/fdg-sim/badge.svg)](https://docs.rs/fdg-sim)                                                                | [![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/grantshandy/fdg/blob/main/fdg-sim/LICENSE)           | Runs the layout engine (simulation) and manages the position of nodes.                                                    |
| [`fdg-macroquad`](./fdg-macroquad/README.md) | [![Latest version](https://img.shields.io/crates/v/fdg-macroquad.svg)](https://crates.io/crates/fdg-macroquad) | [![Documentation](https://docs.rs/fdg-macroquad/badge.svg)](https://docs.rs/fdg-macroquad)                                                    | [![GPL-3.0](https://img.shields.io/badge/license-GPL-blue.svg)](https://github.com/grantshandy/fdg/blob/main/fdg-macroquad/LICENSE) | A demo visualizer that lets you interact with the graph in real time. ([View Online](https://grantshandy.github.io/fdg/)) |
| [`fdg-img`](./fdg-img/README.md)             | [![Latest version](https://img.shields.io/crates/v/fdg-img.svg)](https://crates.io/crates/fdg-img)             | [![Documentation](https://docs.rs/fdg-img/badge.svg)](https://docs.rs/fdg-img)                                                                | [![GPL-3.0](https://img.shields.io/badge/license-GPL-blue.svg)](https://github.com/grantshandy/fdg/blob/main/fdg-img/LICENSE)       | A simple SVG renderer for your graphs.                                                                                    |
| [`fdg-wasm`](./fdg-wasm/README.md)           | [![NPM Package](https://img.shields.io/npm/v/fdg-wasm)](https://www.npmjs.com/package/fdg-wasm)                | [![View Readme](https://docs.rs/fdg-sim/badge.svg)](https://github.com/grantshandy/fdg/tree/main/fdg-wasm#forcegraphsimulation-documentation) | [![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/grantshandy/fdg/blob/main/fdg-sim/LICENSE)           | A simple Webassembly wrapper of `fdg-sim` for use in Javascript.                                                          |

## Stability Note/Future Plans:

 > **Note**: There are some parts of this library that are certainly not as polished as I'd like them to be. My Rust API (and calculus) abilities have improved quite a bit since I first wrote this library, and there is a lot of room for improvement.
 > 
 > While the library is certainly usable, there are a few things that I'd like to address in an upcoming 1.0+:
 >
 > - [ ] Lack of "state of the art" (post 1996) force algorithms such as [ForceAtlas2](https://journals.plos.org/plosone/article?id=10.1371/journal.pone.0098679).
 > - [ ] Only simple Euler method instead of more stable [RK4](https://en.wikipedia.org/wiki/Runge%E2%80%93Kutta_methods) or [Leapfrog](https://en.wikipedia.org/wiki/Leapfrog_integration) integration.
 > - [ ] Unnecessary `Clone` requirement for `Node` data (and associated "hot-clones" 😬).
 > - [ ] Opaque `Force` *struct* containing a function pointer instead of an idiomatic `Force` *trait*.
 > - [ ] Buggy node dragging in fdg-macroquad.
 > - [ ] Awkwardly worded documentation.
 > - [ ] Unpolished WASM API.
 > - [ ] Sometimes buggy/inflexible DOT/GML/jsongraph parsers.
 > - [ ] [Broken self-connected nodes](https://github.com/grantshandy/fdg/issues/10).

## Resources
- [Force-Directed Graphs on Wikipedia](https://en.wikipedia.org/wiki/Force-directed_graph_drawing)
- [Force Directed Drawing Algorithms (Kobourov)](https://cs.brown.edu/people/rtamassi/gdhandbook/chapters/force-directed.pdf)
- Example Javascript force directed graph layout engines [d3-force](https://github.com/d3/d3-force) and [ngraph.forcelayout](https://github.com/anvaka/ngraph.forcelayout)
