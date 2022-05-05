#![doc = include_str!("../README.md")]

mod cpu;
#[cfg(feature = "gpu")]
mod gpu;
mod graph;
mod simulation;

pub use glam::Vec3;
pub use petgraph;

pub use cpu::{CpuSimulation, Forces};
#[cfg(feature = "gpu")]
use gpu::GpuSimulation;
pub use graph::{ForceGraph, ForceGraphHelper};
pub use simulation::{Dimensions, Node, Simulation, SimulationParameters};

#[cfg(feature = "json")]
pub use graph::graph_from_json;
