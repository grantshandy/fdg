use std::ops::Range;

pub use glam::Vec3;
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
#[derive(Clone)]
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
        }
    }

    pub fn new_with_location(name: impl AsRef<str>, data: D, location: Vec3) -> Self {
        Self {
            name: name.as_ref().to_string(),
            data,
            location,
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
        }
    }
}

/// Contains our graph and runs the layout algorithm.
#[derive(Clone)]
pub struct Simulation<D> {
    /// Internal force graph
    pub graph: ForceGraph<D>,
    /// Gravity coefficient (positive makes nodes attract and negative repels)
    pub gravity: f32,
    /// Number of dimensions, either 2D or 3D
    pub dimensions: Dimensions,
}

impl<D> Simulation<D> {
    /// Create a new simulation from a [`ForceGraph`]
    pub fn from_graph(graph: ForceGraph<D>, dimensions: Dimensions) -> Self {
        let mut myself = Self {
            graph,
            gravity: -20.0,
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
            node.location = Vec3::new(
                rng.gen_range(NODE_START_RANGE),
                rng.gen_range(NODE_START_RANGE),
                rng.gen_range(NODE_START_RANGE),
            );

            node.acceleration = Vec3::ZERO;
            node.velocity = Vec3::ZERO;

            // If we only have 2 dimensions then flaten out the locations to a 2D plane
            // This should let us do our physics the same for all dimensions settings, just ignore z in 2d sims.00
            if self.dimensions == Dimensions::Two {
                node.location.z = 0.0;
            }
        }
    }

    /// This is where the physics happens! we'll probably have to feed it a time delay value or something
    pub fn step(&mut self) {
        // Take a snapshot of all of the nodes before we change them
        // let nodes_snapshot = self.graph.node_weights().clone().collect::<Vec<&Node<D>>>();
        // let nodes =  self.graph.node_weights_mut().collect::<Vec<&mut Node<D>>>();

        // for node in nodes {
        //     for snapshot in &nodes_snapshot {

        //     }
        // }

        // This will be helpful to show coords without a visualizer as we start
        // trace!(
        //     "Node \"{}\" coords: {{ x: {}, y: {}, z: {} }}",
        //     node.name,
        //     node.location.x,
        //     node.location.y,
        //     node.location.z
        // );
    }
}
