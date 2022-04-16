use std::ops::Range;

use glam::Vec3;
use log::trace;
use petgraph::{
    graph::{EdgeIndex, NodeIndex},
    stable_graph::StableGraph,
    visit::{EdgeRef, IntoEdgeReferences},
    Undirected,
};
use rand::Rng;
//static GRAVITY: f32 = 0.000000000066743;

use super::node::Node;

/// Number of Dimensions to run our simulation in.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Dimensions {
    Two,
    Three,
}

/// Settings for the simulation
#[derive(Clone, Debug, PartialEq)]
pub struct SimulationParameters {
    pub gravity: f32,
    pub node_start_range: Range<f32>,
    pub cooloff_factor: f32,
    pub ideal_spring_length: f32,
}

impl Default for SimulationParameters {
    fn default() -> Self {
        Self {
            gravity: 150.0,
            node_start_range: -10.0..10.0,
            cooloff_factor: 0.99,
            ideal_spring_length: 7.0,
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
        let node_start_range = &self.parameters.node_start_range;

        for node in self.graph.node_weights_mut() {
            // put nodes in random locations
            node.location = Vec3::new(
                rng.gen_range(node_start_range.clone()),
                rng.gen_range(node_start_range.clone()),
                // if we are in 2D set z to 0, this should let us calculate physics in 3d like normal but keep 2d relevant
                match self.dimensions {
                    Dimensions::Two => 0.0,
                    Dimensions::Three => rng.gen_range(node_start_range.clone()),
                },
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

            for other_index in self.graph.node_indices() {
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
                let force =
                    (self.parameters.gravity * 10.0) * node.mass * other_node.mass / distance_squared;

                //calculate force vector
                let fvector = Vec3::new(force * angle.cos(), force * angle.sin(), 0.0);

                force_final += fvector;
            }

            // The attractive force here is from Fruchterman & Reingold (1991)
            for other_index in self.graph.neighbors(node_index) {
                let node = &self.graph[node_index];
                let other_node = &self.graph[other_index];

                let distance = node.location.distance(other_node.location);

                let distance_factor = distance / self.parameters.ideal_spring_length;

                let fvector = (node.location - other_node.location);

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
