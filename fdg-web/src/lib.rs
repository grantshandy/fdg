use fdg_sim::{
    force,
    glam::Vec3,
    petgraph::{
        graph::NodeIndex,
        visit::{EdgeRef, IntoEdgeReferences},
    },
    Dimensions, ForceGraph, ForceGraphHelper, Simulation,
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
        let mut sim: Simulation<JsValue, JsValue> = Simulation::default();

        sim.parameters_mut()
            .set_force(force::handy(45.0, 0.975, true, true));

        Self { sim }
    }

    #[wasm_bindgen(method, setter, js_name = "graph")]
    pub fn set_graph(&mut self, json: JsValue) -> Result<(), JsError> {
        Ok(())
    }

    #[wasm_bindgen(method, getter, js_name = "graph")]
    pub fn get_graph(&mut self) -> Result<(), JsError> {
        Ok(())
    }

    #[wasm_bindgen]
    pub fn add_node(&mut self, name: String, weight: JsValue) -> Result<usize, JsError> {
        match node_index_from_name(self.sim.get_graph(), &name) {
            Some(_) => Err(JsError::new(&format!(
                "node with name \"{name}\" already in graph"
            ))),
            None => Ok(self
                .sim
                .get_graph_mut()
                .add_force_node(name, weight)
                .index()),
        }
    }

    #[wasm_bindgen(method, getter, js_name = "nodes")]
    pub fn get_nodes(&self) -> Array {
        let array = Array::new();

        for node in self.sim.get_graph().node_weights() {
            let node = ForceGraphNode::new(node);

            array.push(&node.into());
        }

        array
    }

    #[wasm_bindgen]
    pub fn add_edge(
        &mut self,
        source: JsValue,
        target: JsValue,
        weight: JsValue,
    ) -> Result<(), JsError> {
        let source: NodeIndex = if let Some(source) = source.as_string() {
            match node_index_from_name(&self.sim.get_graph(), &source) {
                Some(idx) => idx,
                None => {
                    return Err(JsError::new(&format!(
                        "source \"{source}\" does not exist in graph"
                    )))
                }
            }
        } else if let Some(source) = source.as_f64() {
            NodeIndex::from(source as u32)
        } else {
            return Err(JsError::new("source must be a number or string"));
        };

        let target: NodeIndex = if let Some(target) = target.as_string() {
            match node_index_from_name(&self.sim.get_graph(), &target) {
                Some(idx) => idx,
                None => {
                    return Err(JsError::new(&format!(
                        "target \"{target}\" does not exist in graph"
                    )))
                }
            }
        } else if let Some(target) = target.as_f64() {
            NodeIndex::from(target as u32)
        } else {
            return Err(JsError::new("target must be a number or string"));
        };

        self.sim.get_graph_mut().add_edge(source, target, weight);

        Ok(())
    }

    #[wasm_bindgen(method, getter, js_name = "edges")]
    pub fn get_edges(&self) -> Array {
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

    #[wasm_bindgen]
    pub fn find(&self, query: Vec<f32>, radius: f32) -> JsValue {
        let query = Vec3::new(query[0], query[1], query[2]);

        match self.sim.find(query, radius) {
            Some(idx) => match self.sim.get_graph().node_weight(idx) {
                Some(node) => ForceGraphNode::new(node).into(),
                None => JsValue::NULL,
            },
            None => JsValue::NULL,
        }
    }

    #[wasm_bindgen]
    pub fn update(&mut self, dt: f32) {
        self.sim.update(dt);
    }
}

fn node_index_from_name<N, E>(
    graph: &ForceGraph<N, E>,
    name: impl AsRef<str>,
) -> Option<NodeIndex> {
    let name = name.as_ref().to_string();

    for index in graph.node_indices() {
        if &graph[index].name == &name {
            return Some(index);
        }
    }

    return None;
}
