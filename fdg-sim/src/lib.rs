use std::ops::Range;

pub use glam::Vec3;
use log::trace;
pub use petgraph;
use petgraph::{stable_graph::StableGraph, Undirected};
use rand::Rng;

// Range in which the nodes will be randomly placed on the first frame.
const NODE_START_RANGE: Range<f32> = 0.0..1.0;

/// A helper type that creates a [`StableGraph`] with our custom [`Node`].
pub type ForceGraph<D> = StableGraph<Node<D>, (), Undirected>;

/// Syntactic sugar to make adding [`Node`]s to a [`ForceGraph`] easier.
pub trait ForceGraphHelper<D> {
    fn add_force_node(&mut self, name: impl AsRef<str>, data: D);
}

impl<D> ForceGraphHelper<D> for ForceGraph<D> {
    fn add_force_node(&mut self, name: impl AsRef<str>, data: D) {
        self.add_node(Node::new(name, data));
    }
}

/// Number of Dimensions to run our simulation in.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Dimensions {
    Two,
    Three,
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
    /// 3D acceleration
    pub acceleration: Vec3,
    /// Mass (defaults to 1)
    pub mass: f32,
}

impl<D> Node<D> {
    /// Create a new node with it's name and associated data
    pub fn new(name: impl AsRef<str>, data: D) -> Self {
        Self {
            name: name.as_ref().to_string(),
            data,
            location: Vec3::ZERO,
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            mass: 1.0,
        }
    }
}

/// Contains our graph and runs the layout algorithm.
#[derive(Clone)]
pub struct Simulation<D: Clone + PartialEq> {
    /// Internal force graph
    pub graph: ForceGraph<D>,
    /// Gravity coefficient (positive makes nodes attract and negative repels)
    pub gravity: f32,
    /// Number of dimensions, either 2D or 3D
    pub dimensions: Dimensions,
}

impl<D: Clone + PartialEq> Simulation<D> {
    /// Create a new simulation from a [`ForceGraph`]
    pub fn from_graph(graph: ForceGraph<D>, dimensions: Dimensions) -> Self {
        let mut myself = Self {
            graph,
            gravity: -10.0,
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
    pub fn step(&mut self) {
        // This is where the physics happens! we'll probably have to feed it a time delay value or something
        let nodes = self.graph.clone();

        for node in self.graph.node_weights_mut() {
            let mut acceleration_vector_list: Vec<Vec3> = Vec::new();

            for other_node in nodes.node_weights() {
                // skip duplicates
                if node == other_node {
                    continue;
                }

                // calculate distance between node and other_node using distance formula

                // calculate force vector for node from law of gravitation
                // convert force vector into acceleration vector
                // add acceleration vector to acceleration vector list

                // for our experiment now we'll add a test vector

                acceleration_vector_list.push(Vec3::X);
            }

            // todo later: do this again but for edge (spring) forces between nodes


            // set new node acceleration vector as the average of the acceleration vector list
            let mut acceleration = node.acceleration;

            

            node.acceleration = acceleration;
            // calculate new velocity vector from acceleration vector

            // calculate new location from velocity vector and time interval


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
}
