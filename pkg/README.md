# fdg-wasm
This is the webassembly api for [`fdg`](https://github.com/grantshandy/fdg). Currently a WIP.

## `new ForceGraphSimulation()`

### *simulation*.**graph**
Get and set the simulation's internal graph based on the [jsongraph specification](https://github.com/jsongraph/json-graph-specification).

### *simulation*.**addNode**(String name)
Insert a new node into the graph and returns it's index.

### *simulation*.**nodes**
Retrieve all the nodes from the graph in an array.

### *simulation*.**addEdge**(source index or name, target index or name, weight)
Add a new edge into the graph.

### *simulation*.**edges**
Retrieve all the edges from the graph in an array.

### *simulation*.**resetNodePlacement**()
Resets the nodes in the graph to a random position near the origin.

### *simulation*.**setDimensions**()
Sets the number of dimensions the simulation runs in.

### *simulation*.**find**(query: [Number; 3], radius: Number)
Queries the graph for a node in a location with a radius and returns the node.

### *simulation*.**nodeInfo**(String name | Number index)
Returns the node info from the name or index of a node.

### *simulation*.**update**(Number dt)
Updates the internal simulation for an interval.