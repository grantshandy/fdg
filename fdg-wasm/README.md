# fdg-wasm
[![NPM Package](https://img.shields.io/npm/v/fdg-wasm)](https://www.npmjs.com/package/fdg-wasm)

This is the webassembly api for [`fdg`](https://github.com/grantshandy/fdg). I'm really not an expert in anything javascript, so proceed with a bit of caution here.

## Basic Example
```javascript
import init, { ForceGraphSimulator } from 'https://unpkg.com/fdg-wasm@0.1.0/fdg-wasm.js';

await init();

let sim = new ForceGraphSimulator();

sim.graph = await fetch('https://raw.githubusercontent.com/jsongraph/json-graph-specification/master/examples/les_miserables.json')
    .then(response => response.json())
    .then(response => response.graph);

sim.setDimensions(2);
sim.resetNodePlacement();

for (let i = 0; i < 2000; i++) {
    sim.update(0.035);
}

sim.nodes.forEach((node) => {
    console.log(`${node.name}: ${node.location}`);
});
```

It works with deno greate just like this:
```shell
$ deno run --allow-net deno.js
Listolier: 197.79331970214844,134.728515625,0
CountessdeLo: 157.74066162109375,-356.2279052734375,0
Dahlia: 228.4669647216797,134.91195678710938,0
Jondrette: -403.8577880859375,86.82536315917969,0
Napoleon: 175.74081420898438,-324.6880187988281,0
Zephine: 195.84951782226562,157.9574737548828,0
Champtercier: 50.745147705078125,-383.8532409667969,0
Brujon: -95.5260009765625,23.29458236694336,0
Cochepaille: 137.5876007080078,-97.65261840820312,0
Fauchelevent: -81.33965301513672,-95.81346893310547,0
Mme.Hucheloup: -188.01766967773438,79.68028259277344,0
Boulatruelle: -140.707275390625,-19.337364196777344,0
Child1: -261.75665283203125,36.16830062866211,0
...
```


## `ForceGraphSimulation` Documentation

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

## `jsongraph_to_dot(String graph)`
Returns a graphviz dot graph from a jsongraph string.