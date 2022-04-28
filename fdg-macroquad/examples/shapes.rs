use fdg_sim::{ForceGraph, ForceGraphHelper, Simulation, SimulationParameters};

#[macroquad::main("Force Graph Shape Demo")]
async fn main() {
    pretty_env_logger::init();

    let mut graph: ForceGraph<()> = ForceGraph::default();

    // cube
    let cube_one_a = graph.add_force_node("", ());
    let cube_one_b = graph.add_force_node("", ());
    let cube_two_a = graph.add_force_node("", ());
    let cube_two_b = graph.add_force_node("", ());
    let cube_three_a = graph.add_force_node("", ());
    let cube_three_b = graph.add_force_node("", ());
    let cube_four_a = graph.add_force_node("", ());
    let cube_four_b = graph.add_force_node("", ());
    graph.add_edge(cube_one_a, cube_two_a, ());
    graph.add_edge(cube_two_a, cube_three_a, ());
    graph.add_edge(cube_three_a, cube_four_a, ());
    graph.add_edge(cube_four_a, cube_one_a, ());
    graph.add_edge(cube_one_b, cube_two_b, ());
    graph.add_edge(cube_two_b, cube_three_b, ());
    graph.add_edge(cube_three_b, cube_four_b, ());
    graph.add_edge(cube_four_b, cube_one_b, ());
    graph.add_edge(cube_one_a, cube_one_b, ());
    graph.add_edge(cube_two_a, cube_two_b, ());
    graph.add_edge(cube_three_a, cube_three_b, ());
    graph.add_edge(cube_four_a, cube_four_b, ());

    // square pyramid
    let pyramid_one = graph.add_force_node("", ());
    let pyramid_two = graph.add_force_node("", ());
    let pyramid_three = graph.add_force_node("", ());
    let pyramid_four = graph.add_force_node("", ());
    let pyramid_center = graph.add_force_node("", ());
    graph.add_edge(pyramid_one, pyramid_two, ());
    graph.add_edge(pyramid_two, pyramid_three, ());
    graph.add_edge(pyramid_three, pyramid_four, ());
    graph.add_edge(pyramid_four, pyramid_one, ());
    graph.add_edge(pyramid_center, pyramid_one, ());
    graph.add_edge(pyramid_center, pyramid_two, ());
    graph.add_edge(pyramid_center, pyramid_three, ());
    graph.add_edge(pyramid_center, pyramid_four, ());

    fdg_macroquad::run_window(&mut Simulation::from_graph(
        graph,
        SimulationParameters::default(),
    ))
    .await;
}
