use chrono::prelude::*;
use fdg_sim::{CpuSimulation, ForceGraph, ForceGraphHelper, Simulation, SimulationParameters};
use petgraph::graph::NodeIndex;
use rand::Rng;

const NUM_NODES: u32 = 400;
const NUM_EDGES: u32 = 400;
const TIME_DIFFERENCE: f32 = 0.0032;
const NUM_CALCULATIONS: u32 = 100;

fn main() {
    let mut graph: ForceGraph<()> = ForceGraph::default();
    let mut indices: Vec<NodeIndex> = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..NUM_NODES {
        indices.push(graph.add_force_node("", ()));
    }

    for _ in 0..NUM_EDGES {
        let source = indices[rng.gen_range(0..indices.len())];
        let target = indices[rng.gen_range(0..indices.len())];

        graph.add_edge(source, target, ());
    }

    let b = Utc::now();
    let mut cpu = CpuSimulation::from_graph(&graph, SimulationParameters::default());
    for n in 0..NUM_CALCULATIONS {
        println!("Running CPU calculation {n}/{NUM_CALCULATIONS}");
        cpu.update(TIME_DIFFERENCE);
    }
    let d = Utc::now().signed_duration_since(b).num_seconds();

    #[cfg(feature = "gpu")]
    {
        use fdg_sim::GpuSimulation;
        let b = Utc::now();
        let mut gpu = GpuSimulation::from_graph(&graph, SimulationParameters::default());
        for n in 0..NUM_CALCULATIONS {
            println!("Running GPU calculation {n}/{NUM_CALCULATIONS}");
            gpu.update(TIME_DIFFERENCE);
        }
        let d = Utc::now().signed_duration_since(b).num_seconds();
        println!("GpuSimulation took {d} seconds to simulate a graph with {NUM_NODES} nodes and {NUM_EDGES} edges {NUM_CALCULATIONS} times with an interval of {TIME_DIFFERENCE} seconds.");
    }

    println!("CpuSimulation took {d} seconds to simulate a graph with {NUM_NODES} nodes and {NUM_EDGES} edges {NUM_CALCULATIONS} times with an interval of {TIME_DIFFERENCE} seconds.");
}
