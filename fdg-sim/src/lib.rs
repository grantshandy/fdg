#![doc = include_str!("../README.md")]

mod force;
mod graph;
mod simulation;

pub use {
    force::{Force, FruchtermanReingold},
    glam::Vec3,
    graph::{ForceGraph, ForceGraphHelper},
    petgraph,
    simulation::{Dimensions, Node, Simulation, SimulationParameters},
};

#[cfg(feature = "json")]
pub use graph::graph_from_json;
