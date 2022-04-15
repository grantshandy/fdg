# fdg (Force Directed Graph)
Force directed graph simulation

![example graph](https://d3-wiki.readthedocs.io/zh_CN/master/force.png)

*An [example](https://vasturiano.github.io/force-graph/example/load-json/) of a force directed graph visualization*

The goal of this project is to provide a force-directed graph algorithm for Rust, as well as 2D and 3D visualizers for this algorithm that work on the web and on desktop.

<!-- ## Contents
- `/fdg-sim` The underlying force simulation. Handles your dataset and node's positions based on a simple physics engine.
- `/fdg-3d` A 3D visualizer for `fdg-sim` using `wgpu` to render on the web and on desktop.
- `/fdg-2d-wgpu` A 2D visualizer for `fdg-sim` using `wgpu` to render on the web and on desktop.
- `/fdg-2d-macroquad` A 2D visualizer for `fdg-sim` using `macroquad` to render. This is slower but it'll be much easier to use while we polish `fdg-sim`. -->

<!-- ## Structure
```
-----------------------
|     Application     |
-----------------------
|  fdg-3d  |  fdg-2d  |
-----------------------
|       fdg-sim       |
-----------------------
``` -->

## Resources
- [Force-Directed Graphs on Wikipedia](https://en.wikipedia.org/wiki/Force-directed_graph_drawing)
- [Force Directed Drawing Algorithms (Kobourov)](https://cs.brown.edu/people/rtamassi/gdhandbook/chapters/force-directed.pdf)
- Example javascript force directed graph layout engines [d3-force](https://github.com/d3/d3-force) and [ngraph.forcelayout](https://github.com/anvaka/ngraph.forcelayout)