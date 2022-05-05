use super::ForceGraph;
use glam::Vec3;
use petgraph::graph::{EdgeIndex, NodeIndex};

/// Number of dimensions to run the simulation in.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Dimensions {
    Two,
    Three,
}

/// A general trait for running a simulation.
pub trait Simulation<D: Clone> {
    /// Create a new [`Simulation`] from a [`ForceGraph`].
    fn from_graph(graph: ForceGraph<D>, parameters: SimulationParameters) -> Self;
    /// Reset the location of all the nodes to random positions.
    fn reset_node_placement(&mut self);
    /// Update node locations over a given interval.
    fn update(&mut self, dt: f32);
    /// Run a callback on every node.
    fn visit_nodes(&self, cb: &mut impl Fn(&Node<D>));
    /// Run a callback on every set of edge endpoints.
    fn visit_edges(&self, cb: &mut impl Fn(&Node<D>, &Node<D>));
    /// Add a new node to the internal graph.
    fn add_node(&mut self, name: impl AsRef<str>, data: D) -> NodeIndex;
    /// Add an edge to the internal graph.
    fn add_edge(&mut self, a: NodeIndex, b: NodeIndex) -> EdgeIndex;
    /// Remove a node from the internal graph.
    fn remove_node(&mut self, index: NodeIndex) -> Option<Node<D>>;
    /// Remove an edge to the internal graph.
    fn remove_edge(&mut self, index: EdgeIndex);
    /// Get a reference to the internal [`ForceGraph`].
    fn get_graph(&self) -> &ForceGraph<D>;
    /// Clear all data in the internal graph.
    fn clear(&mut self);
    /// Get a reference to the internal parameters.
    fn parameters(&self) -> &SimulationParameters;
    /// Get a mutable reference to the internal parameters.
    fn parameters_mut(&mut self) -> &mut SimulationParameters;
    // TODO: Add `get_node_from_coordinates` and lock location of certain nodes.
}

/// Parameters for the simulation.
#[derive(Clone)]
pub struct SimulationParameters {
    pub cooloff_factor: f32,
    pub node_start_size: f32,
    pub dimensions: Dimensions,
}

impl Default for SimulationParameters {
    fn default() -> Self {
        Self {
            cooloff_factor: 0.975,
            node_start_size: 500.0,
            dimensions: Dimensions::Two,
        }
    }
}

/// A node on a [`ForceGraph`].
#[derive(Clone, PartialEq)]
pub struct Node<D> {
    /// The name of the node
    pub name: String,
    /// data can be some other arbitrary information you want to store.
    pub data: D,
    /// 3D coordinates
    pub location: Vec3,
    /// 3D velocity
    pub velocity: Vec3,
    /// Mass (defaults to 1)
    pub mass: f32,
    /// Color
    pub color: [u8; 4],
}

impl<D> Node<D> {
    /// Create a new node with it's name and associated data
    pub fn new(name: impl AsRef<str>, data: D) -> Self {
        Self {
            name: name.as_ref().to_string(),
            data,
            location: Vec3::ZERO,
            velocity: Vec3::ZERO,
            mass: 1.0,
            color: [0, 0, 0, 255],
        }
    }

    /// Create a new node with a custom color
    pub fn new_with_color(name: impl AsRef<str>, data: D, color: [u8; 4]) -> Self {
        Self {
            name: name.as_ref().to_string(),
            data,
            location: Vec3::ZERO,
            velocity: Vec3::ZERO,
            mass: 1.0,
            color,
        }
    }
}
