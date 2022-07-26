use chrono::prelude::*;
use fdg_sim::{ForceGraph, ForceGraphHelper, Simulation, SimulationParameters};
use petgraph::graph::NodeIndex;
use quad_rand::RandomRange;

const NUM_NODES: u32 = 400;
const NUM_EDGES: u32 = 400;
const TIME_DIFFERENCE: f32 = 0.0032;
const NUM_CALCULATIONS: u32 = 100;

fn main() {
    let mut graph: ForceGraph<(), ()> = ForceGraph::default();
    gen_graph(&mut graph);

    let cpu = cpu(&graph);
    // let gpu = gpu(&graph);
    println!("cpu took {cpu} milliseconds to simulate a graph with {NUM_NODES} nodes and {NUM_EDGES} edges {NUM_CALCULATIONS} times with an interval of {TIME_DIFFERENCE} seconds.");
    // println!("gpu took {gpu} milliseconds to simulate a graph with {NUM_NODES} nodes and {NUM_EDGES} edges {NUM_CALCULATIONS} times with an interval of {TIME_DIFFERENCE} seconds.")
}

fn gen_graph(graph: &mut ForceGraph<(), ()>) {
    let mut indices: Vec<NodeIndex> = Vec::new();

    for _ in 0..NUM_NODES {
        indices.push(graph.add_force_node("", ()));
    }

    for _ in 0..NUM_EDGES {
        let source = indices[RandomRange::gen_range(0, indices.len())];
        let target = indices[RandomRange::gen_range(0, indices.len())];

        graph.add_edge(source, target, ());
    }
}

fn cpu(graph: &ForceGraph<(), ()>) -> i64 {
    let b = Utc::now();
    let mut sim = Simulation::from_graph(&graph, SimulationParameters::default());
    for _ in 0..NUM_CALCULATIONS {
        sim.update(TIME_DIFFERENCE);
    }
    Utc::now().signed_duration_since(b).num_milliseconds()
}

// fn gpu(graph: &ForceGraph<(),  ()>) -> i64 {
//     let b = Utc::now();
//     let mut sim = Simulation::from_graph(
//         &graph,
//         SimulationParameters::from_force(FruchtermanReingoldGpu::default()),
//     );
//     for _ in 0..NUM_CALCULATIONS {
//         sim.update(TIME_DIFFERENCE);
//     }
//     Utc::now().signed_duration_since(b).num_milliseconds()
// }
