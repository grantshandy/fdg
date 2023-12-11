use ::rand::distributions::Uniform;
use fdg_sim::{
    petgraph::Graph, Center, Force, ForceGraph, FruchtermanReingold,
    FruchtermanReingoldConfiguration, Node, Translate,
};
use nalgebra::vector;

use macroquad::prelude::*;

#[macroquad::main("fdg demo")]
async fn main() {
    let mut graph = Graph::<&str, ()>::new();
    let pg = graph.add_node("petgraph");
    let fb = graph.add_node("fixedbitset");
    let qc = graph.add_node("quickcheck");
    let rand = graph.add_node("rand");
    let libc = graph.add_node("libc");
    graph.extend_with_edges(&[(pg, fb), (pg, qc), (qc, rand), (rand, libc), (qc, libc)]);

    let mut force_graph: ForceGraph<f32, 2, &str, ()> =
        fdg_sim::init_force_graph(graph, Uniform::new(-200.0, 200.0));

    let mut center = Center::default();
    let mut translate = Translate::new(vector![0.0, -100.0]);
    let mut force = FruchtermanReingold {
        conf: FruchtermanReingoldConfiguration {
            scale: 400.0,
            ..Default::default()
        },
        ..Default::default()
    };

    loop {
        // apply the fruchterman-reingold force 4 times
        for _ in 0..4 {
            force.apply(&mut force_graph);
        }

        // move the graph mean position to 0,0
        center.apply(&mut force_graph);

        // translate the whole graph up 100 units
        translate.apply(&mut force_graph);

        clear_background(WHITE);

        for idx in force_graph.edge_indices() {
            let (Node(_, source), Node(_, target)) = force_graph
                .edge_endpoints(idx)
                .map(|(a, b)| {
                    (
                        force_graph.node_weight(a).unwrap(),
                        force_graph.node_weight(b).unwrap(),
                    )
                })
                .unwrap();

            draw_line(
                translate_x(source.coords.column(0)[0]),
                translate_y(source.coords.column(0)[1]),
                translate_x(target.coords.column(0)[0]),
                translate_y(target.coords.column(0)[1]),
                4.0,
                BLACK,
            );
        }

        for Node(name, pos) in force_graph.node_weights() {
            let x = translate_x(pos.coords.column(0)[0]);
            let y = translate_y(pos.coords.column(0)[1]);

            draw_circle(x, y, 20.0, RED);
            draw_text(name, x - 30.0, y - 30.0, 40.0, BLACK);
        }

        next_frame().await
    }
}

fn translate_x(x: f32) -> f32 {
    (screen_width() / 2.0) + x
}

fn translate_y(y: f32) -> f32 {
    (screen_height() / 2.0) + y
}
