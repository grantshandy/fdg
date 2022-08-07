#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

/// Forces that define how your graph moves.
pub mod force;

mod graph;
mod simulation;

#[cfg(feature = "json")]
/// Import and export graphs with json.
pub mod json;

#[cfg(feature = "gml")]
/// Import and export graphs with gml.
pub mod gml;

pub use glam;
pub use petgraph;

pub use {
    graph::{ForceGraph, ForceGraphHelper},
    simulation::{Dimensions, Node, Simulation, SimulationParameters},
};
