#![doc = include_str!("../../README.md")]

pub mod force;
mod graph;
mod simulation;

#[cfg(feature = "json")]
mod json;

pub use {
    glam::Vec3,
    graph::{ForceGraph, ForceGraphHelper},
    petgraph,
    simulation::{Dimensions, Node, Simulation, SimulationParameters},
};

#[cfg(feature = "json")]
pub use json::{graph_from_json, json_from_graph};
