use fdg_sim::gml;

fn main() {
    let graph = gml::graph_from_gml(include_str!("../../datasets/basic.gml")).unwrap();

    let gml = gml::gml_from_graph(&graph);

    println!("{gml}");
}