use std::ops::Range;

use log::trace;
use petgraph::{
    graph::{EdgeIndex, NodeIndex},
    stable_graph::StableGraph,
    visit::{EdgeRef, IntoEdgeReferences},
    Undirected,
};
use rand::Rng;

pub use glam::Vec3;
pub use petgraph;

/// Settings for the simulation
#[derive(Clone, Debug, PartialEq)]
pub struct SimulationParameters {
    pub charge_constant: f32,
    pub node_start_range: Range<f32>,
    pub cooloff_factor: f32,
    pub ideal_spring_length: f32,
    pub spring_constant: f32
}

impl Default for SimulationParameters {
    fn default() -> Self {
        Self {
            charge_constant: 40.0,
            node_start_range: -10.0..10.0,
            cooloff_factor: 0.98,
            ideal_spring_length: 100.0,
            spring_constant: 1.0
        }
    }
}

/// Contains our graph and runs the layout algorithm.
#[derive(Clone)]
pub struct Simulation<D: Clone + PartialEq> {
    /// Internal force graph
    graph: ForceGraph<D>,
    /// Simulation Parameters
    pub parameters: SimulationParameters,
}

impl<D: Clone + PartialEq> Simulation<D> {
    /// Create a new simulation from a [`ForceGraph`]
    pub fn from_graph(
        graph: ForceGraph<D>,
        parameters: SimulationParameters,
    ) -> Self {
        let mut myself = Self {
            graph,
            parameters,
        };

        // place nodes in starting position
        myself.reset_node_placement();

        myself
    }

    /// Reset locations for every node back to the beginning
    pub fn reset_node_placement(&mut self) {
        let mut rng = rand::thread_rng();
        let node_start_range = &self.parameters.node_start_range;

        for node in self.graph.node_weights_mut() {
            // put nodes in random locations
            node.location = Vec3::new(
                rng.gen_range(node_start_range.clone()),
                rng.gen_range(node_start_range.clone()),
                // if we are in 2D set z to 0, this should let us calculate physics in 3d like normal but keep 2d relevant
                0.0,
            );

            // reset velocity
            node.velocity = Vec3::ZERO;
        }
    }

    /// step through the simulation
    /// dt is the time since the last step
    pub fn step(&mut self, dt: f32) {
        let graph_clone = self.graph.clone();

        for node_index in graph_clone.node_indices() {
            let mut force_final: Vec3 = Vec3::new(0.0, 0.0, 0.0);

            for other_index in graph_clone.node_indices() {
                // skip duplicates
                if other_index == node_index {
                    continue;
                }

                let other_node = &self.graph[other_index];
                let node = &self.graph[node_index];

                // The repulsive force here is Coulomb's law.

                //there is probably a better way to do this without using angles -- note for later
                //calculates distance (r^2 in coulomb's equation) to save a few cpu cycles
                let distance_squared = node.location.distance_squared(other_node.location);
                let displacement = node.location - other_node.location;

                //computes angle between the two nodes in question
                let angle = (displacement.y).atan2(displacement.x);

                //calculate force according to coulomb's equation
                let force = (self.parameters.charge_constant * 100.0) * node.mass * other_node.mass
                    / distance_squared;

                //calculate force vector
                let fvector = Vec3::new(force * angle.cos(), force * angle.sin(), 0.0);

                force_final += fvector;
            }

            for neighbor in self.graph.neighbors(node_index) {
                let neighbor = &self.graph[neighbor];
                let node = &self.graph[node_index];
                let distance= node.location.distance(neighbor.location);
                let displacement = node.location - neighbor.location;

                //computes angle between the two nodes in question
                let angle = (displacement.y).atan2(displacement.x);

                //calculate force according to coulomb's equation
                let force = (self.parameters.spring_constant * 10.0) * -(distance - self.parameters.ideal_spring_length);
                 //calculate force vector
                let fvector = Vec3::new(force * angle.cos(), force * angle.sin(), 0.0);

                force_final += fvector;

            }

            let node = &mut self.graph[node_index];

            // calculate acceleration vector
            let acceleration = force_final / node.mass;

            // calculate new velocity vector from acceleration vector
            node.velocity += acceleration * dt;

            // multiply velocity by cooloff factor
            node.velocity *= self.parameters.cooloff_factor;

            // calculate new location from velocity vector and time interval
            node.location += node.velocity * dt;

            // log out new node status
            trace!(
                "Node \"{}\" coords: {{ x: {}, y: {}, z: {} }}",
                node.name,
                node.location.x,
                node.location.y,
                node.location.z,
            );
        }
    }

    /// Run callback with access to every node
    pub fn visit_nodes<F: FnMut(&Node<D>)>(&self, mut cb: F) {
        for n_idx in self.graph.node_indices() {
            cb(&self.graph[n_idx]);
        }
    }

    /// Run callback with access to source and target of every edge
    pub fn visit_edges<F: FnMut(&Node<D>, &Node<D>)>(&self, mut cb: F) {
        for edge_ref in self.graph.edge_references() {
            cb(
                &self.graph[edge_ref.source()],
                &self.graph[edge_ref.target()],
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
        }
    }
}
