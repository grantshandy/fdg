use fdg_sim::gml;

fn main() {
    let graph = gml::graph_from_gml(include_str!("../../datasets/pyramid.gml")).unwrap();

    let gml = gml::graph_to_gml(&graph);

    println!("{gml}");
}
