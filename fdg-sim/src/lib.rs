use std::ops::Range;

mod node;
mod simulation;

pub use glam::Vec3;
pub use petgraph;

pub use node::Node;
pub use simulation::{Dimensions, ForceGraph, ForceGraphHelper, Simulation, SimulationParameters};

// Range in which the nodes will be randomly placed on the first frame.
const NODE_START_RANGE: Range<f32> = 0.0..1.0;
