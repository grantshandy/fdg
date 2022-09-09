import init, { jsongraph_to_dot } from './pkg/fdg-wasm.js'

await init();

let jsongraph = await fetch('https://raw.githubusercontent.com/jsongraph/json-graph-specification/master/examples/les_miserables.json')
    .then(response => response.text());

let dot = jsongraph_to_dot(jsongraph);

console.log('jsongraph:\n\n');
console.log(jsongraph);
console.log('dot graph:\n\n');
console.log(dot);