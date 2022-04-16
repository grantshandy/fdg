use std::ops::Range;

mod node;
mod simulation;

pub use glam::Vec3;
pub use petgraph;

pub use node::Node;
pub use simulation::{Dimensions, ForceGraph, ForceGraphHelper, Simulation, SimulationParameters};
