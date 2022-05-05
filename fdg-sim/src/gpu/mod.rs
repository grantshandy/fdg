use glam::Vec3;
use petgraph::{
    graph::{EdgeIndex, NodeIndex},
    visit::{EdgeRef, IntoEdgeReferences},
};
use rand::Rng;

use crate::{Dimensions, ForceGraph, ForceGraphHelper, Node, Simulation, SimulationParameters};

pub struct GpuSimulation<D: Clone> {
    graph: ForceGraph<D>,
    parameters: SimulationParameters,
}

impl<D: Clone> Simulation<D> for GpuSimulation<D> {
    fn from_graph(graph: &ForceGraph<D>, parameters: SimulationParameters) -> Self {
        let mut myself = Self {
            graph: graph.clone(),
            parameters,
        };

        myself.reset_node_placement();

        myself
    }

    fn reset_node_placement(&mut self) {
        let mut rng = rand::thread_rng();

        for node in self.graph.node_weights_mut() {
            node.location = Vec3::new(
                rng.gen_range(
                    -(self.parameters.node_start_size / 2.0)
                        ..(self.parameters.node_start_size / 2.0),
                ),
                rng.gen_range(
                    -(self.parameters.node_start_size / 2.0)
                        ..(self.parameters.node_start_size / 2.0),
                ),
                match self.parameters.dimensions {
                    Dimensions::Three => rng.gen_range(
                        -(self.parameters.node_start_size / 2.0)
                            ..(self.parameters.node_start_size / 2.0),
                    ),
                    Dimensions::Two => 0.0,
                },
            );

            node.velocity = Vec3::ZERO;
        }
    }

    // go crazy here ethan
    fn update(&mut self, dt: f32) {
        todo!()
    }

    fn visit_nodes(&self, cb: &mut impl Fn(&Node<D>)) {
        for n_idx in self.graph.node_indices() {
            cb(&self.graph[n_idx]);
        }
    }

    fn visit_edges(&self, cb: &mut impl Fn(&Node<D>, &Node<D>)) {
        for edge_ref in self.graph.edge_references() {
            cb(
                &self.graph[edge_ref.source()],
                &self.graph[edge_ref.target()],
            );
        }
    }

    fn add_node(&mut self, name: impl AsRef<str>, data: D) -> NodeIndex {
        self.graph.add_force_node(name, data)
    }

    fn add_edge(&mut self, a: NodeIndex, b: NodeIndex) -> EdgeIndex {
        self.graph.add_edge(a, b, ())
    }

    fn get_graph(&self) -> &ForceGraph<D> {
        &self.graph
    }

    fn remove_node(&mut self, index: NodeIndex) -> Option<Node<D>> {
        self.graph.remove_node(index)
    }

    fn remove_edge(&mut self, index: EdgeIndex) {
        self.graph.remove_edge(index);
    }

    fn clear(&mut self) {
        self.graph.clear();
    }

    fn parameters(&self) -> &SimulationParameters {
        &self.parameters
    }

    fn parameters_mut(&mut self) -> &mut SimulationParameters {
        &mut self.parameters
    }
}
