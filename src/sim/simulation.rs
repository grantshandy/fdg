use glam::Vec3;
use log::trace;
use petgraph::{
    graph::{EdgeIndex, NodeIndex},
    stable_graph::StableGraph,
    Undirected,
};
use rand::Rng;
static GRAVITY: f32 = 0.000000000066743;

use crate::sim::{Node, NODE_START_RANGE};

/// Number of Dimensions to run our simulation in.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Dimensions {
    Two,
    Three,
}

/// Settings for the simulation
#[derive(Clone, Debug, PartialEq)]
pub struct SimulationParameters {
    force_charge: f32,
    force_spring: f32,
}

impl Default for SimulationParameters {
    fn default() -> Self {
        Self {
            force_charge: 10.0,
            force_spring: 10.0,
        }
    }
}

/// Contains our graph and runs the layout algorithm.
#[derive(Clone)]
pub struct Simulation<D: Clone + PartialEq> {
    /// Internal force graph
    graph: ForceGraph<D>,
    /// Simulation Parameters
    parameters: SimulationParameters,
    /// Number of dimensions to run the simulation in
    dimensions: Dimensions,
}

impl<D: Clone + PartialEq> Simulation<D> {
    /// Create a new simulation from a [`ForceGraph`]
    pub fn from_graph(
        graph: ForceGraph<D>,
        dimensions: Dimensions,
        parameters: SimulationParameters,
    ) -> Self {
        let mut myself = Self {
            graph,
            parameters,
            dimensions,
        };

        // place nodes in starting position
        myself.reset_node_placement();

        myself
    }

    /// Reset locations for every node back to the beginning
    pub fn reset_node_placement(&mut self) {
        let mut rng = rand::thread_rng();

        for node in self.graph.node_weights_mut() {
            // put nodes in random locations
            node.location = Vec3::new(
                rng.gen_range(NODE_START_RANGE),
                rng.gen_range(NODE_START_RANGE),
                // if we are in 2D set z to 0, this should let us calculate physics in 3d like normal but keep 2d relevant
                match self.dimensions {
                    Dimensions::Two => 0.0,
                    Dimensions::Three => rng.gen_range(NODE_START_RANGE),
                },
            );

            // reset acceleration and velocity
            node.acceleration = Vec3::ZERO;
            node.velocity = Vec3::ZERO;
        }
    }

    /// step through the simulation
    /// dt is the time since the last step
    pub fn step(&mut self, dt: f32) {
        // This is where the physics happens! we'll probably have to feed it a time delay value or something
        let nodes = self.graph.clone();

        for node in self.graph.node_weights_mut() {
            let mut force_acc: Vec3 = Vec3::new(0.0, 0.0, 0.0);

            for other_node in nodes.node_weights() {
                // skip duplicates
                if node == other_node {
                    continue;
                }

                let loc: Vec3 = node.location;
                //tjere is probably a better way to do this without using angles -- note for later
                //calculates distance (r^2 in the gravitational equation) to save a few cpu cycles
                let distance_squared = loc.distance_squared(other_node.location);

                //computes angle between the two nodes in question
                let angle = loc.angle_between(other_node.location);

                //calcualtes force according to gravitational equation
                let force = GRAVITY * node.mass * other_node.mass / distance_squared; 

                //claculates force vector
                let fvector = Vec3::new(force * angle.cos(), force * angle.sin(), 0.0);

                force_acc += fvector; 

            }

            // todo later: do this again but for edge (spring) forces between nodes

            // we actually don't need an acceleration varibale stored in each node but i will do it this way nonetheless, we can remove it later
            node.acceleration = force_acc / node.mass;

            // calculate new velocity vector from acceleration vector
            node.velocity += node.acceleration * dt; 

            // calculate new location from velocity vector and time interval
            node.location += node.velocity * dt;
            // log out new node status
            trace!(
                "Node \"{}\" coords: {{ x: {}, y: {}, z: {} }}",
                node.name,
                node.location.x,
                node.location.y,
                node.location.z
            );
        }
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, name: impl AsRef<str>, data: D) -> NodeIndex {
        self.graph.add_force_node(name, data)
    }

    /// Add an edge to the graph
    pub fn add_edge(&mut self, a: NodeIndex, b: NodeIndex) -> EdgeIndex {
        self.graph.add_edge(a, b, ())
    }

    /// Get the internal force graph from the simulation
    pub fn get_graph(&self) -> &ForceGraph<D> {
        &self.graph
    }

    /// Remove a node from the graph
    pub fn remove_node(&mut self, index: NodeIndex) -> Option<Node<D>> {
        self.graph.remove_node(index)
    }

    /// Remove an edge from the graph
    pub fn remove_edge(&mut self, index: EdgeIndex) {
        self.graph.remove_edge(index);
    }

    /// Clear all edges and nodes from the graph
    pub fn clear(&mut self) {
        self.graph.clear();
    }
}

/// A helper type that creates a [`StableGraph`] with our custom [`Node`].
pub type ForceGraph<D> = StableGraph<Node<D>, (), Undirected>;

/// Syntactic sugar to make adding [`Node`]s to a [`ForceGraph`] easier.
pub trait ForceGraphHelper<D> {
    fn add_force_node(&mut self, name: impl AsRef<str>, data: D) -> NodeIndex;
}

impl<D> ForceGraphHelper<D> for ForceGraph<D> {
    fn add_force_node(&mut self, name: impl AsRef<str>, data: D) -> NodeIndex {
        self.add_node(Node::new(name, data))
    }
}
