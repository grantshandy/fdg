use crate::{ForceGraphHelper, Force, FruchtermanReingold};

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
pub struct SimulationParameters<D, F> {
    pub node_start_size: f32,
    pub dimensions: Dimensions,
    pub force: Force<D, F>
}

impl<D, F> Default for SimulationParameters<D, F> {
    fn default() -> Self {
        Self {
            node_start_size: 200.0,
            dimensions: Dimensions::Two,
            force: FruchtermanReingold::new(35.0)
        }
    }
}

/// A simulation that runs all physics on the CPU.
#[derive(Clone)]
pub struct Simulation<D: Clone, F> {
    graph: ForceGraph<D>,
    parameters: SimulationParameters<D, F>,
}

impl<D: Clone, F> Simulation<D, F> {
    pub fn from_graph(graph: &ForceGraph<D>, parameters: SimulationParameters<D, F>) -> Self {
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
        todo!()
    }

    pub fn visit_nodes(&self, cb: &mut impl Fn(&Node<D>)) {
        for n_idx in self.graph.node_indices() {
            cb(&self.graph[n_idx]);
        }
    }

    pub fn visit_edges(&self, cb: &mut impl Fn(&Node<D>, &Node<D>)) {
        for edge_ref in self.graph.edge_references() {
            cb(
                &self.graph[edge_ref.source()],
                &self.graph[edge_ref.target()],
            );
        }
    }

    pub fn add_node(&mut self, name: impl AsRef<str>, data: D) -> NodeIndex {
        self.graph.add_force_node(name, data)
    }

    pub fn add_edge(&mut self, a: NodeIndex, b: NodeIndex) -> EdgeIndex {
        self.graph.add_edge(a, b, ())
    }

    pub fn get_graph(&self) -> &ForceGraph<D> {
        &self.graph
    }

    pub fn get_graph_mut(&mut self) -> &mut ForceGraph<D> {
        &mut self.graph
    }

    pub fn remove_node(&mut self, index: NodeIndex) -> Option<Node<D>> {
        self.graph.remove_node(index)
    }

    pub fn remove_edge(&mut self, index: EdgeIndex) {
        self.graph.remove_edge(index);
    }

    pub fn clear(&mut self) {
        self.graph.clear();
    }

    pub fn parameters(&self) -> &SimulationParameters<D, F> {
        &self.parameters
    }

    pub fn parameters_mut(&mut self) -> &mut SimulationParameters<D, F> {
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

    pub fn set_graph(&mut self, graph: &ForceGraph<D>) {
        self.graph = graph.clone();
    }
}

impl<D: Clone, F> Default for Simulation<D, F> {
    fn default() -> Self {
        return Self::from_graph(&ForceGraph::default(), SimulationParameters::default());
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