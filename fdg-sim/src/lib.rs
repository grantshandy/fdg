#![doc = include_str!("../README.md")]

#[cfg(feature = "gpu")]
mod gpu;
mod cpu;
mod graph;
mod simulation;

pub use glam::Vec3;
pub use petgraph;

#[cfg(feature = "gpu")]
use gpu::GpuSimulation;
pub use cpu::{CpuSimulation, Forces};
pub use graph::{ForceGraph, ForceGraphHelper};
pub use simulation::{Dimensions, Node, Simulation, SimulationParameters};

#[cfg(feature = "json")]
pub use graph::graph_from_json;
