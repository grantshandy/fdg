use log::trace;
use petgraph::Graph;

pub use petgraph;

pub type NodeData = dyn Clone + Send;

pub struct Simulation<NodeData> {
    /// Internal data structure
    pub graph: Graph<Node<NodeData>, ()>,
    // Maybe here we'll add other settings?
}

impl<NodeData> Simulation<NodeData> {
    /// Create a new simulation from an undirected graph dataset.
    pub fn new(graph: Graph<Node<NodeData>, ()>) -> Self {
        Self { graph }
    }

    pub fn step(&mut self) {
        for node in self.graph.node_weights_mut() {
            // simple demonstration
            node.x += 0.5;
            trace!("{} x: {}, y: {}, z: {}", node.name, node.x, node.y, node.z);
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Node<NodeData> {
    /// The name of the node
    pub name: String,
    /// data can be some other arbitrary information you want to store.
    pub data: Option<NodeData>,
    /// x coord
    pub x: f32,
    /// y coord
    pub y: f32,
    /// z coord (can be kept at zero while we work in 2D for the first part)
    pub z: f32,
}

impl<NodeData> Node<NodeData> {
    pub fn new<S: AsRef<str>>(name: S, data: Option<NodeData>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            data,
            x: 0.0, // everything starts in the center and expands out at the beginning of the simulation.
            y: 0.0,
            z: 0.0,
        }
    }
}
