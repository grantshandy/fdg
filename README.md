# fdg (Force Directed Graph)
A Force Directed Graph Framework for Rust.

![2D example](./fdg-macroquad/screenshots/screenshot-2D.png)
![3D example](./fdg-macroquad/screenshots/screenshot-3D.png)

The goal of this project is to provide a force-directed graph framework and algorithms for Rust, as well as 2D and 3D visualizers that work on the web and desktop. It sits on top of `petgraph` and manages the positions of your nodes.

[View Examples Online](https://grantshandy.github.io/fdg)

## Contents
- [`/fdg-sim`](./fdg-sim/) The underlying force simulation framework. Handles your dataset's positions based on a physics engine of your choice (or creation).
- [`/fdg-macroquad`](./fdg-macroquad/) A visualizer that uses `macroquad` for rendering.
- [`/fdg-img`](./fdg-img/) A SVG visualizer for your graphs.

## Resources
- [Force-Directed Graphs on Wikipedia](https://en.wikipedia.org/wiki/Force-directed_graph_drawing)
- [Force Directed Drawing Algorithms (Kobourov)](https://cs.brown.edu/people/rtamassi/gdhandbook/chapters/force-directed.pdf)
- Example javascript force directed graph layout engines [d3-force](https://github.com/d3/d3-force) and [ngraph.forcelayout](https://github.com/anvaka/ngraph.forcelayout)
