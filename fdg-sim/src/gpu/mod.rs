// go crazy here ethan

use petgraph::graph::{EdgeIndex, NodeIndex};

use crate::{ForceGraph, Node, Simulation, SimulationParameters};

pub struct GpuSimulation<D: Clone> {
    graph: ForceGraph<D>,
}

impl<D: Clone> Simulation<D> for GpuSimulation<D> {
    fn from_graph(graph: ForceGraph<D>, parameters: SimulationParameters) -> Self {
        todo!()
    }

    fn reset_node_placement(&mut self) {
        todo!()
    }

    fn update(&mut self, dt: f32) {
        todo!()
    }

    fn visit_nodes(&self, cb: &mut impl Fn(&Node<D>)) {
        todo!()
    }

    fn visit_edges(&self, cb: &mut impl Fn(&Node<D>, &Node<D>)) {
        todo!()
    }

    fn add_node(&mut self, name: impl AsRef<str>, data: D) -> NodeIndex {
        todo!()
    }

    fn add_edge(&mut self, a: NodeIndex, b: NodeIndex) -> EdgeIndex {
        todo!()
    }

    fn remove_node(&mut self, index: NodeIndex) -> Option<Node<D>> {
        todo!()
    }

    fn remove_edge(&mut self, index: EdgeIndex) {
        todo!()
    }

    fn get_graph(&self) -> &ForceGraph<D> {
        todo!()
    }

    fn clear(&mut self) {
        todo!()
    }

    fn parameters(&self) -> &SimulationParameters {
        todo!()
    }

    fn parameters_mut(&mut self) -> &mut SimulationParameters {
        todo!()
    }
}
