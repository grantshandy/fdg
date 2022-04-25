use super::{ForceGraph, ForceGraphHelper, Force};
use glam::Vec3;
use log::trace;
use petgraph::{
    graph::{EdgeIndex, NodeIndex},
    visit::{EdgeRef, IntoEdgeReferences},
};
use rand::Rng;

/// Settings for the simulation
#[derive(Clone)]
pub struct SimulationParameters<D> {
    pub cooloff_factor: f32,
    pub node_start_size: f32,
    pub general_force: Force<D>,
    pub neighbor_force: Force<D>,
}

impl<D> Default for SimulationParameters<D> {
    fn default() -> Self {
        Self {
            cooloff_factor: 0.98,
            node_start_size: 20.0,
            general_force: Force::coulomb(),
            neighbor_force: Force::hooke(),
        }
    }
}

/// Contains our graph and runs the layout algorithm.
#[derive(Clone)]
pub struct Simulation<D: Clone> {
    /// Internal force graph
    graph: ForceGraph<D>,
    /// Simulation Parameters
    pub parameters: SimulationParameters<D>,
}

impl<D: Clone> Simulation<D> {
    /// Create a new simulation from a [`ForceGraph`]
    pub fn from_graph(graph: ForceGraph<D>, parameters: SimulationParameters<D>) -> Self {
        let mut myself = Self { graph, parameters };

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
                rng.gen_range(
                    -(self.parameters.node_start_size / 2.0)
                        ..(self.parameters.node_start_size / 2.0),
                ),
                rng.gen_range(
                    -(self.parameters.node_start_size / 2.0)
                        ..(self.parameters.node_start_size / 2.0),
                ),
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
        let graph = self.graph.clone();

        for node_index in graph.node_indices() {
            let mut final_force = Vec3::ZERO;

            for other_node_index in graph.node_indices() {
                // skip duplicates
                if other_node_index == node_index {
                    continue;
                }

                final_force += self
                    .parameters
                    .general_force
                    .apply(&graph[node_index], &graph[other_node_index]);
            }

            for neighbor_index in graph.neighbors(node_index) {
                final_force += self
                    .parameters
                    .neighbor_force
                    .apply(&graph[node_index], &graph[neighbor_index]);
            }

            let node = &mut self.graph[node_index];

            // calculate acceleration vector
            let acceleration = final_force / node.mass;

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