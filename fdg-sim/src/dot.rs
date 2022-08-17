//!
//! Here's an example graph:
//!
//! ```
//! graph {
//!     0 [ label = "8" ]
//!     1 [ label = "3" ]
//!     2 [ label = "4" ]
//!     3 [ label = "5" ]
//!     4 [ label = "1" ]
//!     5 [ label = "7" ]
//!     6 [ label = "6" ]
//!     7 [ label = "2" ]
//!     4 -- 7 [ ]
//!     7 -- 1 [ ]
//!     1 -- 2 [ ]
//!     2 -- 4 [ ]
//!     3 -- 6 [ ]
//!     6 -- 5 [ ]
//!     5 -- 0 [ ]
//!     0 -- 3 [ ]
//!     4 -- 3 [ ]
//!     7 -- 6 [ ]
//!     1 -- 5 [ ]
//!     2 -- 0 [ ]
//! }
//! ```

use std::{collections::HashMap, error::Error, fmt};

use petgraph::{
    dot::{Config, Dot},
    graph::NodeIndex,
    stable_graph::StableGraph,
    visit::{EdgeRef, IntoEdgeReferences},
    Undirected,
};

use crate::ForceGraph;

/// Errors that can be returned by the functions in this module.
#[derive(Clone, Debug)]
pub enum DotError {
    /// Logically, this should never happen.
    IndexNotFound(String),
}

impl fmt::Display for DotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IndexNotFound(n) => write!(f, "Index for {n} was not found in the graph"),
        }
    }
}

impl Error for DotError {}

/// Convert a [`ForceGraph`] to the DOT
pub fn graph_to_dot<N, E>(graph: &ForceGraph<N, E>) -> Result<String, DotError> {
    let mut new_graph: StableGraph<String, (), Undirected> = StableGraph::default();
    let mut indices: HashMap<String, NodeIndex> = HashMap::new();

    for idx in graph.node_indices() {
        let node = &graph[idx];

        indices.insert(node.name.clone(), new_graph.add_node(node.name.clone()));
    }

    for edge in graph.edge_references() {
        let source = &graph[edge.source()].name;
        let target = &graph[edge.target()].name;

        let source_idx = match indices.get(source) {
            Some(idx) => *idx,
            None => return Err(DotError::IndexNotFound(source.clone())),
        };

        let target_idx = match indices.get(target) {
            Some(idx) => *idx,
            None => return Err(DotError::IndexNotFound(target.clone())),
        };

        new_graph.add_edge(source_idx, target_idx, ());
    }

    Ok(format!("{:?}", Dot::with_config(&new_graph, &[Config::EdgeNoLabel])).replace("\\\"", ""))
}
