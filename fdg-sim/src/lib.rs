#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

/// Change forces that define how your graph behaves.
pub mod force;

mod graph;
mod simulation;

#[cfg(feature = "json")]
/// Import and export graphs with the [jsongraph](http://jsongraphformat.info/) specification.
pub mod json;

#[cfg(feature = "gml")]
/// Import and export graphs with [Graph Modelling Language](https://en.wikipedia.org/wiki/Graph_Modelling_Language) (GML).
pub mod gml;

/// Exports graphs into the [DOT](https://en.wikipedia.org/wiki/DOT_(graph_description_language)) language for use with visualizers like [Graphviz](https://graphviz.org/).
pub mod dot;

pub use glam;
pub use petgraph;

pub use {
    graph::{ForceGraph, ForceGraphHelper},
    simulation::{Dimensions, Node, Simulation, SimulationParameters},
};
