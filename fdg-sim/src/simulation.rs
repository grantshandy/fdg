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
    fn from_graph(graph: &ForceGraph<D>, parameters: SimulationParameters) -> Self;
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
    fn get_graph_mut(&mut self) -> &mut ForceGraph<D>;
    fn set_graph(&mut self, graph: &ForceGraph<D>);
    /// Clear all data in the internal graph.
    fn clear(&mut self);
    /// Get a reference to the internal parameters.
    fn parameters(&self) -> &SimulationParameters;
    /// Get a mutable reference to the internal parameters.
    fn parameters_mut(&mut self) -> &mut SimulationParameters;
    /// Get a node index from X,Y,Z coordinates and a range.
    fn find(&self, query: Vec3, radius: f32) -> Option<NodeIndex>;
    /// Return a reference for the force parameters.
    fn forces(&self) -> &Forces<D>;
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
            node_start_size: 200.0,
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
    /// Color
    pub color: [u8; 4],
    /// Mass
    pub mass: f32,
    pub locked: bool,
}

impl<D> Node<D> {
    /// Create a new node with it's name and associated data
    pub fn new(name: impl AsRef<str>, data: D) -> Self {
        Self {
            name: name.as_ref().to_string(),
            data,
            location: Vec3::ZERO,
            velocity: Vec3::ZERO,
            color: [0, 0, 0, 255],
            mass: 1.0,
            locked: false,
        }
    }

    /// Create a new node with a custom color
    pub fn new_with_color(name: impl AsRef<str>, data: D, color: [u8; 4]) -> Self {
        Self {
            name: name.as_ref().to_string(),
            data,
            location: Vec3::ZERO,
            velocity: Vec3::ZERO,
            color,
            mass: 1.0,
            locked: false,
        }
    }

    pub fn new_with_coords(name: impl AsRef<str>, data: D, location: Vec3) -> Self {
        Self {
            name: name.as_ref().to_string(),
            data,
            location,
            velocity: Vec3::ZERO,
            color: [0, 0, 0, 255],
            mass: 1.0,
            locked: false,
        }
    }
}

/// Forces that dictate how your nodes move.
#[derive(Clone)]
pub struct Forces<D> {
    general_force: fn(&Vec<f32>, &Node<D>, &Node<D>) -> Vec3,
    neighbor_force: fn(&Vec<f32>, &Node<D>, &Node<D>) -> Vec3,
    dict: Vec<f32>,
}

impl<D> Forces<D> {
    pub fn apply_general_force(&self, node_one: &Node<D>, node_two: &Node<D>) -> Vec3 {
        (self.general_force)(&self.dict, node_one, node_two)
    }

    pub fn apply_neighbor_force(&self, node_one: &Node<D>, node_two: &Node<D>) -> Vec3 {
        (self.neighbor_force)(&self.dict, node_one, node_two)
    }

    pub fn dict(&self) -> &Vec<f32> {
        &self.dict
    }
}

/// The default implementation of [`Forces`] uses Fruchterman & Reingold (1991).
impl<D> Default for Forces<D> {
    fn default() -> Self {
        Forces::fruchterman_reingold(45.0)
    }
}

impl<D> Forces<D> {
    pub fn fruchterman_reingold(ideal_distance: f32) -> Self {
        let dict = vec![ideal_distance];

        fn general_force<D>(dict: &Vec<f32>, node_one: &Node<D>, node_two: &Node<D>) -> Vec3 {
            -((dict[0] * dict[0]) / node_one.location.distance(node_two.location))
                * ((node_two.location - node_one.location)
                    / node_one.location.distance(node_two.location))
        }

        fn neighbor_force<D>(dict: &Vec<f32>, node_one: &Node<D>, node_two: &Node<D>) -> Vec3 {
            (node_one.location.distance_squared(node_two.location) / dict[0])
                * ((node_two.location - node_one.location)
                    / node_one.location.distance(node_two.location))
        }

        Self {
            general_force,
            neighbor_force,
            dict,
        }
    }
}
