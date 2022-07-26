use std::collections::HashMap;

use anyhow::{anyhow, Result};
use petgraph::{
    graph::NodeIndex,
    visit::{EdgeRef, IntoEdgeReferences},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{ForceGraph, Node};

#[derive(Serialize, Deserialize)]
struct JsonGraph {
    nodes: Vec<JsonNode>,
    edges: Vec<JsonEdge>,
}

#[derive(Serialize, Deserialize)]
struct JsonNode {
    name: String,
    color: Option<[u8; 4]>,
    data: Option<Value>,
}

#[derive(Serialize, Deserialize)]
struct JsonEdge {
    source: String,
    target: String,
    data: Option<Value>,
}

pub fn json_from_graph<N: Serialize, E: Serialize>(graph: &ForceGraph<N, E>) -> Result<String> {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    for node in graph.node_weights() {
        nodes.push(JsonNode {
            name: node.name.clone(),
            color: Some(node.color.clone()),
            data: Some(serde_json::to_value(&node.data)?),
        });
    }

    for edge in graph.edge_references() {
        let source = &graph[edge.source()].name;
        let target = &graph[edge.target()].name;
        let data = serde_json::to_value(edge.weight())?;

        edges.push(JsonEdge {
            source: source.clone(),
            target: target.clone(),
            data: Some(data),
        });
    }

    let json = serde_json::to_string(&JsonGraph { nodes, edges })?;

    Ok(json)
}

/// Generate a graph from json formatted similar to the [json graph specification](https://github.com/jsongraph/json-graph-specification).
/// Not all features are implemented, but basic graphs like should work:
/// ```json
/// {
///   "nodes": [
///     {
///       "name": "A"
///     },
///     {
///       "name": "B"
///     }
///   ],
///   "edges": [
///     {
///       "source": "A",
///       "target": "B"
///     }
///   ]
/// }
/// ```
pub fn graph_from_json(json: impl AsRef<str>) -> Result<ForceGraph<Value, Value>> {
    let json: JsonGraph = serde_json::from_str(json.as_ref())?;

    let mut graph: ForceGraph<Value, Value> = ForceGraph::default();
    let mut indices: HashMap<String, NodeIndex> = HashMap::new();

    for node in json.nodes {
        let data = match &node.data {
            Some(data) => serde_json::to_value(data)?,
            None => Value::default(),
        };

        let (indice, name) = match &node.color {
            Some(color) => (
                graph.add_node(Node::new_with_color(&node.name, data, *color)),
                node.name,
            ),
            None => (graph.add_node(Node::new(&node.name, data)), node.name),
        };

        indices.insert(name, indice);
    }

    for edge in json.edges {
        let source_index = match indices.get(&edge.source) {
            Some(source_index) => source_index.clone(),
            None => return Err(anyhow!("source \"{}\" not found in nodes", &edge.source)),
        };

        let target_index = match indices.get(&edge.target) {
            Some(target_index) => target_index.clone(),
            None => return Err(anyhow!("target \"{}\" not found in nodes", &edge.target)),
        };

        let data = match edge.data {
            Some(data) => data,
            None => Value::default(),
        };

        graph.add_edge(source_index, target_index, data);
    }

    Ok(graph)
}
