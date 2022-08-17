//!
//! Here's an example graph:
//! ```
//! graph [
//!   node [
//!     id 0
//!     label "8"
//!   ]
//!   node [
//!     id 1
//!     label "3"
//!   ]
//!   node [
//!     id 2
//!     label "5"
//!   ]
//!   node [
//!     id 3
//!     label "6"
//!   ]
//!   node [
//!     id 4
//!     label "2"
//!   ]
//!   node [
//!     id 5
//!     label "4"
//!   ]
//!   node [
//!     id 6
//!     label "7"
//!   ]
//!   node [
//!     id 7
//!     label "1"
//!   ]
//!   edge [
//!     source 7
//!     target 4
//!   ]
//!   edge [
//!     source 4
//!     target 1
//!   ]
//!   edge [
//!     source 1
//!     target 5
//!   ]
//!   edge [
//!     source 5
//!     target 7
//!   ]
//!   edge [
//!     source 2
//!     target 3
//!   ]
//!   edge [
//!     source 3
//!     target 6
//!   ]
//!   edge [
//!     source 6
//!     target 0
//!   ]
//!   edge [
//!     source 0
//!     target 2
//!   ]
//!   edge [
//!     source 7
//!     target 2
//!   ]
//!   edge [
//!     source 4
//!     target 3
//!   ]
//!   edge [
//!     source 1
//!     target 6
//!   ]
//!   edge [
//!     source 5
//!     target 0
//!   ]
//! ]
//! ```

use core::fmt;
use std::{collections::HashMap, error::Error};

use crate::{ForceGraph, ForceGraphHelper};
use petgraph::graph::NodeIndex;
use regex::Regex;

/// Possible errors returned by the functions in the module.
#[derive(Clone, Debug)]
pub enum GmlParseError {
    GraphStructure,
    NoNodes,
    IdNotNumber,
    NoId,
    NoSource,
    NoTarget,
    SourceNotNumber,
    TargetNotNumber,
    InvalidSource(usize),
    InvalidTarget(usize),
    RegexError(String),
}

impl fmt::Display for GmlParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GraphStructure => {
                write!(f, "Graph must be structured as \"graph [ [CONTENT] ]\"")
            }
            Self::NoNodes => write!(f, "Graph include nodes"),
            Self::IdNotNumber => write!(f, "Node ids must be a number"),
            Self::NoId => write!(f, "Nodes must have an id"),
            Self::NoSource => write!(f, "Edges must have a source"),
            Self::NoTarget => write!(f, "Edges must have a target"),
            Self::SourceNotNumber => write!(f, "Edge sources must be numbers"),
            Self::TargetNotNumber => write!(f, "Edge targets must be numbers"),
            Self::InvalidSource(s) => write!(f, "Edge source {s} not found in nodes"),
            Self::InvalidTarget(s) => write!(f, "Edge target {s} not found in nodes"),
            Self::RegexError(err) => write!(f, "Regex Error: {err}"),
        }
    }
}

impl Error for GmlParseError {}

/// Get a [`ForceGraph`] from a gml string.
pub fn graph_from_gml(gml: impl AsRef<str>) -> Result<ForceGraph<(), ()>, GmlParseError> {
    let gml = gml.as_ref();

    let mut graph = ForceGraph::default();
    let mut indices: HashMap<usize, NodeIndex<u32>> = HashMap::new();

    // overall graph structure
    let content = match Regex::new(r"graph\s\[([\d\D]+)\]") {
        Ok(r) => match r.captures(gml) {
            Some(x) => x[1].to_string(),
            None => return Err(GmlParseError::GraphStructure),
        },
        Err(err) => return Err(GmlParseError::RegexError(err.to_string())),
    };

    let nodes: Vec<String> = match Regex::new(r"node\s\[([^]]+)\]") {
        Ok(r) => r
            .captures_iter(&content)
            .map(|x| x[1].to_string())
            .collect(),
        Err(err) => return Err(GmlParseError::RegexError(err.to_string())),
    };

    if nodes.is_empty() {
        return Err(GmlParseError::NoNodes);
    }

    let id_regex = match Regex::new(r"\sid\s(\d)") {
        Ok(r) => r,
        Err(err) => return Err(GmlParseError::RegexError(err.to_string())),
    };

    let label_regex = match Regex::new(r##"\slabel\s"([^]]+)""##) {
        Ok(r) => r,
        Err(err) => return Err(GmlParseError::RegexError(err.to_string())),
    };

    for node in nodes {
        let id = match id_regex.captures(&node).map(|x| x[1].to_string()) {
            Some(id) => id,
            None => return Err(GmlParseError::NoId),
        };

        let id: usize = match id.parse() {
            Ok(id) => id,
            Err(_) => return Err(GmlParseError::IdNotNumber),
        };

        let label: String = label_regex
            .captures(&node)
            .map(|x| x[1].to_string())
            .unwrap_or_default();

        indices.insert(id, graph.add_force_node(label, ()));
    }

    let edges: Vec<String> = match Regex::new(r"edge\s\[([^]]+)\]") {
        Ok(r) => r
            .captures_iter(&content)
            .map(|x| x[1].to_string())
            .collect(),
        Err(err) => return Err(GmlParseError::RegexError(err.to_string())),
    };

    let source_regex = match Regex::new(r"\ssource\s(\d)") {
        Ok(r) => r,
        Err(err) => return Err(GmlParseError::RegexError(err.to_string())),
    };

    let target_regex = match Regex::new(r"\starget\s(\d)") {
        Ok(r) => r,
        Err(err) => return Err(GmlParseError::RegexError(err.to_string())),
    };

    for edge in edges {
        let source_str = match source_regex.captures(&edge).map(|x| x[1].to_string()) {
            Some(source) => source,
            None => return Err(GmlParseError::NoSource),
        };

        let target_str = match target_regex.captures(&edge).map(|x| x[1].to_string()) {
            Some(target) => target,
            None => return Err(GmlParseError::NoTarget),
        };

        let source: usize = match source_str.parse() {
            Ok(source) => source,
            Err(_) => return Err(GmlParseError::SourceNotNumber),
        };

        let target: usize = match target_str.parse() {
            Ok(target) => target,
            Err(_) => return Err(GmlParseError::TargetNotNumber),
        };

        let source_idx = match indices.get(&source) {
            Some(idx) => *idx,
            None => return Err(GmlParseError::InvalidSource(source)),
        };

        let target_idx = match indices.get(&target) {
            Some(idx) => *idx,
            None => return Err(GmlParseError::InvalidSource(target)),
        };

        graph.add_edge(source_idx, target_idx, ());
    }

    Ok(graph)
}

/// Create a gml string from a [`ForceGraph`].
pub fn graph_to_gml<N, E>(graph: &ForceGraph<N, E>) -> String {
    let mut final_str = String::new();

    final_str.push_str("graph [\n");

    for id in graph.node_indices() {
        let label = &graph[id].name;

        final_str.push_str(&format!(
            "  node [\n    id {}\n    label \"{}\"\n  ]\n",
            id.index(),
            label
        ));
    }

    for edge in graph.edge_indices() {
        let (source, target) = match graph.edge_endpoints(edge) {
            Some(x) => x,
            None => continue,
        };

        final_str.push_str(&format!(
            "  edge [\n    source {}\n    target {}\n  ]\n",
            source.index(),
            target.index()
        ));
    }

    final_str.push_str("]");

    final_str
}
