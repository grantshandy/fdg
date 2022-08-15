use fdg_sim::{
    force,
    glam::Vec3,
    json,
    petgraph::{
        graph::NodeIndex,
        visit::{EdgeRef, IntoEdgeReferences},
    },
    Dimensions, ForceGraph, ForceGraphHelper, Simulation,
};
use js_sys::Array;
use serde::Serialize;
use serde_json::Value;
use serde_wasm_bindgen::Serializer;
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
        let json: Value = match serde_wasm_bindgen::from_value(json) {
            Ok(json) => json,
            Err(err) => return Err(JsError::new(err.to_string().as_str())),
        };

        if !json.is_object() {
            return Err(JsError::new("graph must be an object"));
        }

        let old_graph = match json::graph_from_json(json.to_string()) {
            Ok(graph) => graph,
            Err(err) => return Err(JsError::new(err.to_string().as_str())),
        };

        let new_graph = serde_to_wasm_graph(&old_graph)?;

        self.sim.set_graph(&new_graph);

        Ok(())
    }

    #[wasm_bindgen(method, getter, js_name = "graph")]
    pub fn get_graph(&mut self) -> Result<JsValue, JsError> {
        let new_graph = wasm_to_serde_graph(self.sim.get_graph())?;

        let json_str = match json::graph_to_json(&new_graph) {
            Ok(json) => json,
            Err(err) => return Err(JsError::new(err.to_string().as_str())),
        };

        // log(&json_str);

        let serde_value: Value = match serde_json::from_str(&json_str) {
            Ok(json) => json,
            Err(err) => {
                return Err(JsError::new(&format!(
                    "fdg-sim did not return valid json: {err}"
                )))
            }
        };

        // log(&serde_value.to_string());

        let json_value: JsValue = serde_value.serialize(
            &Serializer::new()
                .serialize_maps_as_objects(true)
                .serialize_missing_as_null(true),
        )?;

        Ok(json_value)
    }

    #[wasm_bindgen(js_name = "addNode")]
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

    #[wasm_bindgen(js_name = "addEdge")]
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

    #[wasm_bindgen(js_name = "resetNodePlacement")]
    pub fn reset_node_placement(&mut self) {
        self.sim.reset_node_placement();
    }

    #[wasm_bindgen(js_name = "setDimensions")]
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
    pub fn location_from_name(&self, name: String) -> Option<Vec<f32>> {
        let idx = match node_index_from_name(self.sim.get_graph(), name) {
            Some(idx) => idx,
            None => return None,
        };

        return Some(self.sim.get_graph()[idx].location.to_array().to_vec());
    }

    #[wasm_bindgen]
    pub fn update(&mut self, dt: f32) {
        self.sim.update(dt);
    }

    #[wasm_bindgen(js_name = "setNodeLocationFromName")]
    pub fn set_node_location_from_name(&mut self, name: String, location: Vec<f32>) {
        let idx = match node_index_from_name(self.sim.get_graph(), &name) {
            Some(idx) => idx,
            None => return,
        };

        if let Some(node) = self.sim.get_graph_mut().node_weight_mut(idx) {
            node.location = Vec3::new(
                location.get(0).unwrap_or(&0.0).clone(),
                location.get(1).unwrap_or(&0.0).clone(),
                location.get(2).unwrap_or(&0.0).clone(),
            );
        }
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

fn serde_to_wasm_graph(
    graph: &ForceGraph<Value, Value>,
) -> Result<ForceGraph<JsValue, JsValue>, JsError> {
    let mut new_graph = ForceGraph::default();

    for node in graph.node_weights() {
        new_graph.add_force_node(node.name.clone(), serde_wasm_bindgen::to_value(&node.data)?);
    }

    for edge in graph.edge_references() {
        new_graph.add_edge(
            edge.source(),
            edge.target(),
            serde_wasm_bindgen::to_value(&edge.weight())?,
        );
    }

    Ok(new_graph)
}

fn wasm_to_serde_graph(
    graph: &ForceGraph<JsValue, JsValue>,
) -> Result<ForceGraph<Value, Value>, JsError> {
    let mut new_graph = ForceGraph::default();

    for node in graph.node_weights() {
        let weight: Value = node.data.into_serde()?;
        new_graph.add_force_node(node.name.clone(), weight);
    }

    for edge in graph.edge_references() {
        let weight: Value = edge.weight().into_serde()?;

        new_graph.add_edge(edge.source(), edge.target(), weight);
    }

    Ok(new_graph)
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
