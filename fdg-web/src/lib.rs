use fdg_sim::{
    petgraph::{
        graph::NodeIndex,
        visit::{EdgeRef, IntoEdgeReferences},
    },
    ForceGraph, ForceGraphHelper, Simulation, SimulationParameters, Dimensions,
};
use js_sys::Array;
use wasm_bindgen::prelude::*;

mod subtypes;

pub use subtypes::{ForceGraphEdge, ForceGraphNode};

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct ForceGraphSimulator {
    sim: Simulation<JsValue, JsValue>,
}

#[wasm_bindgen]
impl ForceGraphSimulator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let sim: Simulation<JsValue, JsValue> = Simulation::default();

        Self { sim }
    }

    #[wasm_bindgen]
    pub fn add_node(&mut self, name: Option<String>, weight: JsValue) {
        self.sim
            .get_graph_mut()
            .add_force_node(name.unwrap_or_default(), weight);
    }

    #[wasm_bindgen]
    pub fn add_edge(&mut self, a: Option<String>, b: Option<String>, weight: JsValue) {
        if let Some(a) = a {
            if let Some(b) = b {
                let a_idx = match node_index_from_name(&self.sim.get_graph(), a) {
                    Some(idx) => idx,
                    None => return,
                };

                let b_idx = match node_index_from_name(&self.sim.get_graph(), b) {
                    Some(idx) => idx,
                    None => return,
                };

                self.sim.get_graph_mut().add_edge(a_idx, b_idx, weight);
            }
        }
    }

    #[wasm_bindgen]
    pub fn reset_node_placement(&mut self) {
        self.sim.reset_node_placement();
    }

    #[wasm_bindgen]
    pub fn set_dimensions(&mut self, dimensions: u8) {
        let dimensions = match dimensions {
            2 => Dimensions::Two,
            3 => Dimensions::Three,
            _ => Dimensions::Two,
        };

        self.sim.parameters_mut().dimensions = dimensions;
    }

    #[wasm_bindgen(method, getter)]
    pub fn nodes(&self) -> Array {
        let array = Array::new();

        for node in self.sim.get_graph().node_weights() {
            let node = ForceGraphNode::new(node);

            array.push(&node.into());
        }

        array
    }

    #[wasm_bindgen(method, getter)]
    pub fn edges(&self) -> Array {
        let array = Array::new();
        let graph = self.sim.get_graph();

        for edge in self.sim.get_graph().edge_references() {
            let source = graph[edge.source()].name.to_owned();
            let target = graph[edge.target()].name.to_owned();
            let weight = edge.weight().to_owned();

            let edge = ForceGraphEdge::new(source, target, weight);

            array.push(&edge.into());
        }

        array
    }

    #[wasm_bindgen]
    pub fn update(&mut self, dt: f32) {
        self.sim.update(dt);
    }
}

fn node_index_from_name<N, E>(graph: &ForceGraph<N, E>, name: String) -> Option<NodeIndex> {
    for index in graph.node_indices() {
        if &graph[index].name == &name {
            return Some(index);
        }
    }

    return None;
}
