#![doc = include_str!("../README.md")]

mod graph;
mod simulation;
mod force;

pub use {
    glam::Vec3,
    graph::{ForceGraph, ForceGraphHelper},
    petgraph,
    simulation::{Dimensions, Node, Simulation, SimulationParameters},
    force::Forces,
};

#[cfg(feature = "json")]
pub use graph::graph_from_json;
