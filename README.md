# fdg (Force Directed Graph)
Force directed graph simulation

## Contents
- `/fdg-sim` The underlying force simulation. Handles your dataset and node's positions based on a simple physics engine.
- `/fdg-3d` A 3D visualizer for `fdg-sim` using `wgpu` to render on the web and on desktop.
- `/fdg-2d` A 2D visualizer for `fdg-sim` using `wgpu` to render on the web and on desktop.

## Structure
```
-----------------------
|     Application     |
-----------------------
|  fdg-3d  |  fdg-2d  |
-----------------------
|       fdg-sim       |
-----------------------
```

## Resources
- [Force-Directed Graphs on Wikipedia](https://en.wikipedia.org/wiki/Force-directed_graph_drawing)
- [Force Directed Drawing Algorithms](https://cs.brown.edu/people/rtamassi/gdhandbook/chapters/force-directed.pdf) (Stephen G. Kobourov)