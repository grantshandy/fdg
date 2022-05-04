#![doc = include_str!("../README.md")]

mod cpu;
mod force;
mod graph;
mod simulation;

pub use glam::Vec3;
pub use petgraph;

pub use cpu::CpuSimulation;
pub use force::Forces;
pub use graph::{ForceGraph, ForceGraphHelper};
pub use simulation::{Dimensions, Node, Simulation, SimulationParameters};

#[cfg(feature = "json")]
pub use graph::graph_from_json;
