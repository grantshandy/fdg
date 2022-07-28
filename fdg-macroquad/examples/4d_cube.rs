use fdg_macroquad::JsonValue;
use fdg_sim::{
    petgraph::graph::NodeIndex, ForceGraph, ForceGraphHelper, Simulation, SimulationParameters,
};

#[macroquad::main("Force Graph 4D Cube Demo")]
async fn main() {
    pretty_env_logger::init();

    let mut graph: ForceGraph<JsonValue, JsonValue> = ForceGraph::default();

    // create center cube
    let mut cube: [NodeIndex; 8] = gen_cube(&mut graph);
    let layers: u8 = 1;

    for _ in 0..layers {
        cube = add_layer(&mut graph, cube);
    }

    fdg_macroquad::run_window(&mut Simulation::from_graph(
        &graph,
        SimulationParameters::default(),
    ))
    .await;
}

fn add_layer(
    graph: &mut ForceGraph<JsonValue, JsonValue>,
    inner: [NodeIndex; 8],
) -> [NodeIndex; 8] {
    let mut graph = graph;
    let new_cube = gen_cube(&mut graph);

    graph.add_edge(inner[0], new_cube[0], JsonValue::default());
    graph.add_edge(inner[1], new_cube[1], JsonValue::default());
    graph.add_edge(inner[2], new_cube[2], JsonValue::default());
    graph.add_edge(inner[3], new_cube[3], JsonValue::default());
    graph.add_edge(inner[4], new_cube[4], JsonValue::default());
    graph.add_edge(inner[5], new_cube[5], JsonValue::default());
    graph.add_edge(inner[6], new_cube[6], JsonValue::default());
    graph.add_edge(inner[7], new_cube[7], JsonValue::default());

    new_cube
}

fn gen_cube(graph: &mut ForceGraph<JsonValue, JsonValue>) -> [NodeIndex; 8] {
    let one = graph.add_force_node("one", JsonValue::default());
    let two = graph.add_force_node("two", JsonValue::default());
    let three = graph.add_force_node("three", JsonValue::default());
    let four = graph.add_force_node("four", JsonValue::default());
    let five = graph.add_force_node("five", JsonValue::default());
    let six = graph.add_force_node("six", JsonValue::default());
    let seven = graph.add_force_node("seven", JsonValue::default());
    let eight = graph.add_force_node("eight", JsonValue::default());

    graph.add_edge(one, two, JsonValue::default());
    graph.add_edge(two, three, JsonValue::default());
    graph.add_edge(three, four, JsonValue::default());
    graph.add_edge(four, one, JsonValue::default());
    graph.add_edge(five, six, JsonValue::default());
    graph.add_edge(six, seven, JsonValue::default());
    graph.add_edge(seven, eight, JsonValue::default());
    graph.add_edge(eight, five, JsonValue::default());

    graph.add_edge(one, five, JsonValue::default());
    graph.add_edge(two, six, JsonValue::default());
    graph.add_edge(three, seven, JsonValue::default());
    graph.add_edge(four, eight, JsonValue::default());

    [one, two, three, four, five, six, seven, eight]
}
