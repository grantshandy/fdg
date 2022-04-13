use std::{thread, time::Duration};

use fdg_sim::{petgraph::Graph, Node, Simulation};

fn main() {
    pretty_env_logger::init();

    let mut graph: Graph<Node<()>, ()> = Graph::new();

    let one = graph.add_node(Node::new("One", None));
    let two = graph.add_node(Node::new("Two", None));
    let _three = graph.add_node(Node::new("Three", None));

    graph.add_edge(one, two, ());

    let mut sim = Simulation::new(graph);

    loop {
        // step updates the node locations
        // for debugging it will also log the node locations
        sim.step();

        thread::sleep(Duration::from_secs(1));
    }
}
