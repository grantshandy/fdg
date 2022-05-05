#![doc = include_str!("../README.md")]

mod cpu;
#[cfg(feature = "gpu")]
mod gpu;
mod graph;
mod simulation;

pub use cpu::{CpuSimulation, Forces};
#[cfg(feature = "gpu")]
pub use gpu::GpuSimulation;

pub use {
    graph::{ForceGraph, ForceGraphHelper},
    simulation::{Dimensions, Node, Simulation, SimulationParameters},
    petgraph,
    glam::Vec3
};

#[cfg(feature = "json")]
pub use graph::graph_from_json;
