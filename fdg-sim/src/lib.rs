use std::{ops::Range};

pub use glam::Vec3;
use log::trace;
pub use petgraph;
use petgraph::{stable_graph::StableGraph, Undirected};
use rand::Rng;

const NODE_START_RANGE: Range<f32> = 0.0..1.0;

pub type ForceGraph<D> = StableGraph<Node<D>, (), Undirected>;

/// Syntactic sugar to make adding Nodes to a ForceGraph easier.
pub trait ForceGraphHelper<D> {
    fn add_force_node(&mut self, name: impl AsRef<str>, data: D);
}

impl<D> ForceGraphHelper<D> for ForceGraph<D> {
    fn add_force_node(&mut self, name: impl AsRef<str>, data: D) {
        self.add_node(Node::new(name, data));
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Dimensions {
    Two,
    Three,
}

/// A single Node in the graph
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

/// Contains graph and runs physics simulations
#[derive(Clone)]
pub struct Simulation<D> {
    /// Internal data structure
    pub graph: ForceGraph<D>,
    // Maybe here we'll add other settings?
    /// Positive attracts and negative gravity repels nodes
    pub gravity: f32,
    /// Number of dimensions
    pub dimensions: Dimensions,
}

impl<D> Simulation<D> {
    /// Create a new simulation from an undirected graph dataset.
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

    pub fn reset_node_placement(&mut self) {
        let node_start_range = 0.0..1.0;
        let mut rng = rand::thread_rng();

        for node in  self.graph.node_weights_mut() {
            node.location = Vec3::new(
                rng.gen_range(node_start_range.clone()),
                rng.gen_range(node_start_range.clone()),
                rng.gen_range(node_start_range.clone())
            );
            
            // If we only have 2 dimensions then flaten out the locations to a 2D plane
            // This should let us do our physics as normal 3d but still remain applicable.
            if self.dimensions == Dimensions::Two {
                node.location.z = 0.0;
            }
        }
    }

    /// This is where the physics happens! we'll probably have to feed it a time delay value or something
    pub fn step(&mut self) {
        // Take a snapshot of the nodes before we change it
        let nodes_snapshot = self.graph.node_weights().clone().collect::<Vec<&Node<D>>>();
        let nodes =  self.graph.node_weights_mut().collect::<Vec<&mut Node<D>>>();

        for node in nodes {
            for snapshot in &nodes_snapshot {

            }
        }

            // trace!(
            //     "Node \"{}\" coords: {{ x: {}, y: {}, z: {} }}",
            //     node.name,
            //     node.location.x,
            //     node.location.y,
            //     node.location.z
            // );
    }
}