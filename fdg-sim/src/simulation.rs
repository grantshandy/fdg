use crate::force::{fruchterman_reingold, Force};

use super::ForceGraph;
use glam::Vec3;
use petgraph::{
    graph::{EdgeIndex, NodeIndex},
    visit::{EdgeRef, IntoEdgeReferences},
};
use quad_rand::RandomRange;

/// Number of dimensions to run the simulation in.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Dimensions {
    Two,
    Three,
}

/// Parameters for the simulation.
#[derive(Clone)]
pub struct SimulationParameters<N: Clone, E: Clone> {
    pub node_start_size: f32,
    pub dimensions: Dimensions,
    pub force: Force<N, E>,
}

impl<N: Clone, E: Clone> SimulationParameters<N, E> {
    pub fn new(node_start_size: f32, dimensions: Dimensions, force: Force<N, E>) -> Self {
        Self {
            node_start_size,
            dimensions,
            force,
        }
    }
}

impl<N: Clone, E: Clone> SimulationParameters<N, E> {
    pub fn force_mut(&mut self) -> &Force<N, E> {
        &mut self.force
    }

    pub fn force(&self) -> &Force<N, E> {
        &self.force
    }

    pub fn from_force(force: Force<N, E>) -> Self {
        Self {
            force,
            ..Default::default()
        }
    }

    pub fn set_force(&mut self, force: Force<N, E>) {
        self.force = force.clone();
    }
}

impl<N: Clone, E: Clone> Default for SimulationParameters<N, E> {
    fn default() -> Self {
        Self {
            node_start_size: 200.0,
            dimensions: Dimensions::Two,
            force: fruchterman_reingold(45.0, 0.975),
        }
    }
}

/// A simulation that runs all physics on the CPU.
#[derive(Clone)]
pub struct Simulation<N: Clone, E: Clone> {
    graph: ForceGraph<N, E>,
    parameters: SimulationParameters<N, E>,
}

impl<N: Clone, E: Clone> Simulation<N, E> {
    pub fn from_graph(graph: &ForceGraph<N, E>, parameters: SimulationParameters<N, E>) -> Self {
        let mut myself = Self {
            graph: graph.clone(),
            parameters,
        };

        myself.reset_node_placement();

        myself
    }

    pub fn reset_node_placement(&mut self) {
        for node in self.graph.node_weights_mut() {
            node.location = Vec3::new(
                RandomRange::gen_range(
                    -(self.parameters.node_start_size / 2.0),
                    self.parameters.node_start_size / 2.0,
                ),
                RandomRange::gen_range(
                    -(self.parameters.node_start_size / 2.0),
                    self.parameters.node_start_size / 2.0,
                ),
                match self.parameters.dimensions {
                    Dimensions::Three => RandomRange::gen_range(
                        -(self.parameters.node_start_size / 2.0),
                        self.parameters.node_start_size / 2.0,
                    ),
                    Dimensions::Two => 0.0,
                },
            );

            node.velocity = Vec3::ZERO;
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.parameters.force().update(&mut self.graph, dt);
    }

    pub fn update_custom(&mut self, force: &Force<N, E>, dt: f32) {
        force.update(&mut self.graph, dt)
    }

    pub fn visit_nodes(&self, cb: &mut impl Fn(&Node<N>)) {
        for n_idx in self.graph.node_indices() {
            cb(&self.graph[n_idx]);
        }
    }

    pub fn visit_edges(&self, cb: &mut impl Fn(&Node<N>, &Node<N>)) {
        for edge_ref in self.graph.edge_references() {
            cb(
                &self.graph[edge_ref.source()],
                &self.graph[edge_ref.target()],
            );
        }
    }

    pub fn get_graph(&self) -> &ForceGraph<N, E> {
        &self.graph
    }

    pub fn get_graph_mut(&mut self) -> &mut ForceGraph<N, E> {
        &mut self.graph
    }

    pub fn set_graph(&mut self, graph: &ForceGraph<N, E>) {
        self.graph = graph.clone();
    }

    pub fn remove_node(&mut self, index: NodeIndex) -> Option<Node<N>> {
        self.graph.remove_node(index)
    }

    pub fn remove_edge(&mut self, index: EdgeIndex) {
        self.graph.remove_edge(index);
    }

    pub fn clear(&mut self) {
        self.graph.clear();
    }

    pub fn parameters(&self) -> &SimulationParameters<N, E> {
        &self.parameters
    }

    pub fn parameters_mut(&mut self) -> &mut SimulationParameters<N, E> {
        &mut self.parameters
    }

    // thrown together code, should be revised for performance.
    pub fn find(&self, query: Vec3, radius: f32) -> Option<NodeIndex> {
        let query_x = (query.x - radius)..=(query.x + radius);
        let query_y = (query.y - radius)..=(query.y + radius);
        let query_z = (query.z - radius)..=(query.z + radius);

        for index in self.graph.node_indices() {
            let node = &self.graph[index];

            if query_x.contains(&node.location.x)
                && query_y.contains(&node.location.y)
                && query_z.contains(&node.location.z)
            {
                return Some(index);
            }
        }

        None
    }
}

impl<N: Clone, E: Clone> Default for Simulation<N, E> {
    fn default() -> Self {
        return Self::from_graph(&ForceGraph::default(), SimulationParameters::default());
    }
}

/// A node on a [`ForceGraph`].
#[derive(Clone, PartialEq)]
pub struct Node<N> {
    /// The name of the node
    pub name: String,
    /// data can be some other arbitrary information you want to store.
    pub data: N,
    /// 3D coordinates
    pub location: Vec3,
    /// 3D velocity
    pub velocity: Vec3,
    /// Color
    pub color: [u8; 4],
    /// If the location is locked
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
            locked: false,
        }
    }
}
