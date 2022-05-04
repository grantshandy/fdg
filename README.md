# Force Directed Graph
Force directed graph simulation

[![Latest version](https://img.shields.io/crates/v/fdg_sim.svg)](https://crates.io/crates/fdg_sim)
[![Documentation](https://docs.rs/fdg_sim/badge.svg)](https://docs.rs/fdg_sim)
[![GPL-3.0](https://img.shields.io/badge/license-GPL-blue.svg)](https://github.com/skylinecc/fdg/blob/main/LICENSE)

![example graph](https://d3-wiki.readthedocs.io/zh_CN/master/force.png)

*An [example](https://vasturiano.github.io/force-graph/example/load-json/) of a force directed graph visualization.*

The goal of this project is to provide a force-directed graph algorithm for Rust, as well as 2D and 3D visualizers for this algorithm that work on the web and on desktop.

## Contents
- `/fdg-sim` The underlying force simulation. Handles your dataset and node's positions based on a simple physics engine.
- `/fdg-macroquad` A visualizer for `fdg-sim` using `macroquad` to render. This is slower, but it'll be much easier to use while we polish `fdg-sim`.

## Structure
```
-----------------------
|     Application     |
-----------------------
|     Visualizer      |
-----------------------
|       fdg-sim       |
-----------------------
```

## Resources
- [Force-Directed Graphs on Wikipedia](https://en.wikipedia.org/wiki/Force-directed_graph_drawing)
- [Force Directed Drawing Algorithms (Kobourov)](https://cs.brown.edu/people/rtamassi/gdhandbook/chapters/force-directed.pdf)
- Example javascript force directed graph layout engines [d3-force](https://github.com/d3/d3-force) and [ngraph.forcelayout](https://github.com/anvaka/ngraph.forcelayout)