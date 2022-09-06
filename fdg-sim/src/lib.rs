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

mod tests {
    #[test]
    fn dot() {
        use super::{ForceGraph, ForceGraphHelper, dot};

        const RESULT: &str = "graph {\n    0 [ label = \"one\" ]\n    1 [ label = \"two\" ]\n    2 [ label = \"three\" ]\n    0 -- 1 [ ]\n    1 -- 2 [ ]\n}\n";

        let mut graph: ForceGraph<(), ()> = ForceGraph::default();
        let one = graph.add_force_node("one", ());
        let two = graph.add_force_node("two", ());
        let three = graph.add_force_node("three", ());

        graph.add_edge(one, two, ());
        graph.add_edge(two, three, ());

        let dot = dot::graph_to_dot(&graph).unwrap();

        assert_eq!(dot, RESULT);
    }

    #[test]
    #[cfg(feature = "gml")]
    fn gml() {
        use super::{ForceGraph, ForceGraphHelper, gml};

        let mut graph: ForceGraph<(), ()> = ForceGraph::default();
        let one = graph.add_force_node("one", ());
        let two = graph.add_force_node("two", ());
        let three = graph.add_force_node("three", ());

        graph.add_edge(one, two, ());
        graph.add_edge(two, three, ());

        let gml = gml::graph_to_gml(&graph);
        let new_graph = gml::graph_from_gml(gml).unwrap();

        assert_eq!(new_graph.node_count(), 3);
        assert_eq!(new_graph.edge_count(), 2);

        assert!(new_graph
            .node_weights()
            .find(|x| x.name == "one".to_string())
            .is_some());
        assert!(new_graph
            .node_weights()
            .find(|x| x.name == "two".to_string())
            .is_some());
        assert!(new_graph
            .node_weights()
            .find(|x| x.name == "three".to_string())
            .is_some());
    }

    #[test]
    #[cfg(feature = "json")]
    fn json() {
        use super::{ForceGraph, ForceGraphHelper, json};

        let mut graph: ForceGraph<&str, &str> = ForceGraph::default();
        let one = graph.add_force_node("one", "onedata");
        let two = graph.add_force_node("two", "twodata");
        let three = graph.add_force_node("three", "threedata");

        graph.add_edge(one, two, "onetwoedgedata");
        graph.add_edge(two, three, "twothreeedgedata");

        let json = json::graph_to_json(&graph).unwrap();

        let ag = json::graph_from_json(json.to_string()).unwrap();

        assert_eq!(ag.node_count(), 3);
        assert_eq!(ag.edge_count(), 2);

        assert_eq!(
            ag.node_weights()
                .find(|x| x.name == "one")
                .unwrap()
                .data
                .get("metadata")
                .unwrap()
                .to_string()
                .replace('"', ""),
            "onedata".to_string()
        );

        assert!(ag
            .edge_weights()
            .find(|x| x.to_string().replace('"', "") == "onetwoedgedata".to_string())
            .is_some());
    }
}