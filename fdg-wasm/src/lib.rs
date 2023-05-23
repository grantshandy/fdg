use fdg_sim::{
    dot, force,
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
use serde_json::{Map, Value};
use serde_wasm_bindgen::Serializer;
use wasm_bindgen::prelude::*;

mod subtypes;

pub use subtypes::{ForceGraphEdge, ForceGraphNode};

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn jsongraph_to_dot(json: String) -> Result<JsValue, JsError> {
    let graph = match json::graph_from_json(&json) {
        Ok(graph) => graph,
        Err(err) => return Err(JsError::new(&err.to_string())),
    };

    let dot = match dot::graph_to_dot(&graph) {
        Ok(dot) => dot,
        Err(err) => return Err(JsError::new(&err.to_string())),
    };

    Ok(JsValue::from_str(&dot))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ForceGraphNode[]")]
    pub type GraphNodes;

    #[wasm_bindgen(typescript_type = "ForceGraphEdge[]")]
    pub type GraphEdges;

    #[wasm_bindgen(typescript_type = "string | number")]
    pub type NodeReference;
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

    #[wasm_bindgen(setter, js_name = "graph")]
    pub fn set_graph(&mut self, json: JsValue) -> Result<(), JsError> {
        let inner_graph: Value = serde_wasm_bindgen::from_value(json)?;

        if !inner_graph.is_object() {
            return Err(JsError::new("graph must be an object"));
        }

        let mut outer_graph = Map::new();
        outer_graph.insert("graph".to_string(), inner_graph);
        let outer_graph = Value::Object(outer_graph);

        let serde_graph = match json::graph_from_json(outer_graph.to_string()) {
            Ok(graph) => graph,
            Err(err) => return Err(JsError::new(err.to_string().as_str())),
        };

        let wasm_graph = serde_to_wasm_graph(&serde_graph)?;

        self.sim.set_graph(wasm_graph);

        Ok(())
    }

    #[wasm_bindgen(getter, js_name = "graph")]
    pub fn get_graph(&mut self) -> Result<JsValue, JsError> {
        let new_graph = wasm_to_serde_graph(self.sim.get_graph())?;

        let serde_graph = json::graph_to_json(&new_graph)?;

        let inner_serde_graph = match serde_graph.get("graph") {
            Some(graph) => graph,
            None => return Err(JsError::new("fdg-sim did not return json with graph in it")),
        };

        let json_graph: JsValue = inner_serde_graph.serialize(
            &Serializer::new()
                .serialize_maps_as_objects(true)
                .serialize_missing_as_null(true),
        )?;

        Ok(json_graph)
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

    #[wasm_bindgen(js_name = "addNodeWithCoords")]
    pub fn add_node_with_coords(
        &mut self,
        name: String,
        weight: JsValue,
        x: f32,
        y: f32,
        z: f32,
    ) -> Result<usize, JsError> {
        match node_index_from_name(self.sim.get_graph(), &name) {
            Some(_) => Err(JsError::new(&format!(
                "node with name \"{name}\" already in graph"
            ))),
            None => Ok(self
                .sim
                .get_graph_mut()
                .add_force_node_with_coords(name, weight, Vec3::new(x, y, z))
                .index()),
        }
    }

    #[wasm_bindgen(getter, js_name = "nodes")]
    pub fn get_nodes(&self) -> GraphNodes {
        let array = Array::new();

        for node in self.sim.get_graph().node_weights() {
            let node = ForceGraphNode::new(node);

            array.push(&node.into());
        }

        JsValue::from(array).into()
    }

    #[wasm_bindgen(js_name = "addEdge")]
    pub fn add_edge(
        &mut self,
        source: NodeReference,
        target: NodeReference,
        weight: JsValue,
    ) -> Result<(), JsError> {
        let source: NodeIndex = if let Some(source) = source.as_string() {
            match node_index_from_name(self.sim.get_graph(), &source) {
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
            match node_index_from_name(self.sim.get_graph(), &target) {
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

    #[wasm_bindgen(getter, js_name = "edges")]
    pub fn get_edges(&self) -> GraphEdges {
        let array = Array::new();
        let graph = self.sim.get_graph();

        for edge in self.sim.get_graph().edge_references() {
            let source = ForceGraphNode::new(&graph[edge.source()]);
            let target = ForceGraphNode::new(&graph[edge.target()]);
            let weight = edge.weight().to_owned();

            let edge = ForceGraphEdge::new(source, target, weight);

            array.push(&edge.into());
        }

        JsValue::from(array).into()
    }

    #[wasm_bindgen(js_name = "resetNodePlacement")]
    pub fn reset_node_placement(&mut self) {
        self.sim.reset_node_placement();
    }

    /// Set the simulator to use 2 or 3 dimensions.
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

    #[wasm_bindgen(js_name = "nodeInfo")]
    pub fn node_info(&self, name: NodeReference) -> JsValue {
        let idx: NodeIndex = if let Some(name) = name.as_string() {
            match node_index_from_name(self.sim.get_graph(), &name) {
                Some(idx) => idx,
                None => return JsValue::NULL,
            }
        } else if let Some(name) = name.as_f64() {
            NodeIndex::from(name as u32)
        } else {
            return JsValue::NULL;
        };

        ForceGraphNode::new(&self.sim.get_graph()[idx]).into()
    }

    #[wasm_bindgen]
    pub fn update(&mut self, dt: f32) {
        self.sim.update(dt);
    }
}

impl Default for ForceGraphSimulator {
    fn default() -> Self {
        Self::new()
    }
}

fn node_index_from_name<N, E>(
    graph: &ForceGraph<N, E>,
    name: impl AsRef<str>,
) -> Option<NodeIndex> {
    let name = name.as_ref().to_string();

    graph
        .node_indices()
        .find(|&index| graph[index].name == name)
}

fn serde_to_wasm_graph(
    graph: &ForceGraph<Value, Value>,
) -> Result<ForceGraph<JsValue, JsValue>, JsError> {
    let mut new_graph: ForceGraph<JsValue, JsValue> = ForceGraph::default();

    for node in graph.node_weights() {
        let weight = serde_wasm_bindgen::to_value(&node.data)?;
        new_graph.add_force_node(node.name.clone(), weight);
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
    let mut new_graph: ForceGraph<Value, Value> = ForceGraph::default();

    for node in graph.node_weights() {
        let weight: Value = serde_wasm_bindgen::from_value(node.data.clone())?;
        new_graph.add_force_node(node.name.clone(), weight);
    }

    for edge in graph.edge_references() {
        let weight: Value = serde_wasm_bindgen::from_value(edge.weight().clone())?;
        new_graph.add_edge(edge.source(), edge.target(), weight);
    }

    Ok(new_graph)
}
