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