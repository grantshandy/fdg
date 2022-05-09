#![doc = include_str!("../README.md")]

mod cpu;
#[cfg(feature = "gpu")]
mod gpu;
mod graph;
mod simulation;

pub use cpu::CpuSimulation;
#[cfg(feature = "gpu")]
pub use gpu::GpuSimulation;

pub use {
    glam::Vec3,
    graph::{ForceGraph, ForceGraphHelper},
    petgraph,
    simulation::{Dimensions, Node, Simulation, SimulationParameters, Forces},
};

#[cfg(feature = "json")]
pub use graph::graph_from_json;
