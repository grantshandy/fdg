use std::collections::HashMap;

use super::Node;
use petgraph::{graph::NodeIndex, stable_graph::StableGraph, Undirected};
use serde_json::Value;

/// A helper type that creates a [`StableGraph`] with our custom [`Node`].
pub type ForceGraph<D> = StableGraph<Node<D>, (), Undirected>;

/// Syntactic sugar to make adding [`Node`]s to a [`ForceGraph`] easier.
pub trait ForceGraphHelper<D> {
    fn add_force_node(&mut self, name: impl AsRef<str>, data: D) -> NodeIndex;
}

impl<D> ForceGraphHelper<D> for ForceGraph<D> {
    fn add_force_node(&mut self, name: impl AsRef<str>, data: D) -> NodeIndex {
        self.add_node(Node::new(name, data))
    }
}

/// https://github.com/jsongraph/json-graph-specification
pub fn graph_from_json(json: impl AsRef<str>) -> Option<ForceGraph<String>> {
    let mut final_graph: ForceGraph<String> = ForceGraph::default();
    let mut indices: HashMap<String, NodeIndex> = HashMap::new();

    let json: Value = match serde_json::from_str(json.as_ref()) {
        Ok(json) => json,
        Err(_) => return None,
    };

    let graph = match json.get("graph") {
        Some(g) => g,
        None => return None,
    };

    if let Some(nodes) = graph.get("nodes") {
        if let Some(nodes) = nodes.as_object() {
            for (name, value) in nodes {
                let index = final_graph.add_force_node(name, value.to_string());
                indices.insert(name.clone(), index);
            }
        }

        if let Some(edges) = graph.get("edges") {
            let edges = edges.as_array()?;

            for edge in edges {
                let source = *indices
                    .get(&edge.get("source")?.to_string().replace("\"", ""))
                    .unwrap();
                let target = *indices
                    .get(&edge.get("target")?.to_string().replace("\"", ""))
                    .unwrap();

                final_graph.add_edge(source, target, ());
            }
        }
    };

    return Some(final_graph);
}
