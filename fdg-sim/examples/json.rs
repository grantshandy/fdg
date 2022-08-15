use fdg_sim::json;
use petgraph::visit::{EdgeRef, IntoEdgeReferences};

const JSON: &str = r#"{
    "graph": {
        "nodes": {
            "A": {
                "label": "label for A is moved to data section in node"
            },
            "B": {
                "metadata": {
                    "something": "here"
                }
            },
            "C": {}
        },
        "edges": [
            {
                "source": "A",
                "target": "B",
                "metadata": 123451.45
            },
            {
                "source": "B",
                "target": "C",
                "metadata": "just a string here!"
            },
            {
                "source": "C",
                "target": "A",
                "metadata": { "key": "value" }
            }
        ]
    }
}"#;

fn main() {
    let graph = json::graph_from_json(JSON).unwrap();

    println!("---- nodes ----");
    for node in graph.node_weights() {
        println!("name: {}, data: {}", node.name, node.data);
    }

    println!("---- edges ----");
    for edge in graph.edge_references() {
        println!(
            "source: {}, target: {}, data: {}",
            &graph[edge.source()].name,
            &graph[edge.target()].name,
            edge.weight()
        );
    }

    println!("---- output ----");
    println!("{}", json::graph_to_json(&graph).unwrap());
}
