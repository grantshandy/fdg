use glam::Vec3;
use log::trace;
use petgraph::{
    graph::{EdgeIndex, NodeIndex},
    visit::{EdgeRef, IntoEdgeReferences},
};
use rand::Rng;

use crate::{Dimensions, ForceGraph, ForceGraphHelper, Node, Simulation, SimulationParameters};

/// A simulation that runs all physics on the CPU.
#[derive(Clone)]
pub struct CpuSimulation<D: Clone> {
    graph: ForceGraph<D>,
    pub parameters: SimulationParameters<D>,
}

impl<D: Clone> Simulation<D> for CpuSimulation<D> {
    fn from_graph(graph: ForceGraph<D>, parameters: SimulationParameters<D>) -> Self {
        let mut myself = Self { graph, parameters };

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

    fn update(&mut self, dt: f32) {
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
                    .forces
                    .apply_general_force(&graph[node_index], &graph[other_node_index]);
            }

            for neighbor_index in graph.neighbors(node_index) {
                final_force += self
                    .parameters
                    .forces
                    .apply_neighbor_force(&graph[node_index], &graph[neighbor_index]);
            }

            let node = &mut self.graph[node_index];

            let acceleration = final_force / node.mass;
            node.velocity += acceleration * dt;
            node.velocity *= self.parameters.cooloff_factor;

            node.location += node.velocity * dt;

            trace!(
                "Node \"{}\" coords: {{ x: {}, y: {}, z: {} }}",
                node.name,
                node.location.x,
                node.location.y,
                node.location.z,
            );
        }
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

    fn parameters(&self) -> &SimulationParameters<D> {
        &self.parameters
    }

    fn parameters_mut(&mut self) -> &mut SimulationParameters<D> {
        &mut self.parameters
    }
}

impl<D: Clone> Default for CpuSimulation<D> {
    fn default() -> Self {
        return Self::from_graph(ForceGraph::default(), SimulationParameters::default());
    }
}
