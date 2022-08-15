import init, {ForceGraphSimulator} from '../pkg/fdg-web.js';

const canvas = document.getElementById('canvas');
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

const context = canvas.getContext('2d');
const circleRadius = 5;
const lineWidth = 2;
const zoom = 0.75;
const centerX = canvas.width / 2;
const centerY = canvas.height / 2;
const stepTime = 0.035;

await init();

let sim = new ForceGraphSimulator();

sim.graph = await fetch('https://raw.githubusercontent.com/jsongraph/json-graph-specification/master/examples/les_miserables.json')
    .then(response => response.json());

sim.setDimensions(2);
sim.resetNodePlacement();

function step() {
    context.clearRect(0, 0, canvas.width, canvas.height);

    sim.edges.forEach((edge) => {
        let source = sim.location_from_name(edge.source);
        let target = sim.location_from_name(edge.target);

        context.beginPath();
        context.moveTo((source[0] * zoom) + centerX, (source[1] * zoom) + centerY);
        context.lineTo((target[0] * zoom) + centerX, (target[1] * zoom) + centerY);
        context.lineWidth = lineWidth;
        context.strokeStyle = '#000000';
        context.stroke(); 
    });

    sim.nodes.forEach((node) => {
        context.beginPath();
        context.arc((node.location[0] * zoom) + centerX, (node.location[1] * zoom) + centerY, circleRadius, 0, 2 * Math.PI, false);
        context.fillStyle = 'red';
        context.fill();
        context.stroke();
    });

    sim.update(stepTime);
    window.requestAnimationFrame(step);
}

window.requestAnimationFrame(step);
