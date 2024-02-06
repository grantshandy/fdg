use fdg::{
    fruchterman_reingold::{FruchtermanReingold, FruchtermanReingoldConfiguration},
    nalgebra::Rotation2,
    petgraph::Graph,
    simple::Center,
    Force, ForceGraph,
};

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
        fdg::init_force_graph_uniform(graph, 200.0);

    // custom closure force which rotates each node
    let mut rotate = |graph: &mut ForceGraph<f32, 2, &str, ()>| {
        graph
            .node_weights_mut()
            .for_each(|(_, p)| *p = Rotation2::new(0.005).transform_point(p))
    };
    let mut force = FruchtermanReingold {
        conf: FruchtermanReingoldConfiguration {
            scale: 400.0,
            ..Default::default()
        },
        ..Default::default()
    };

    loop {
        // apply the fruchterman-reingold force 4 times
        force.apply_many(&mut force_graph, 4);

        Center::default().apply(&mut force_graph);
        rotate.apply(&mut force_graph);

        clear_background(WHITE);

        for idx in force_graph.edge_indices() {
            let ((_, source), (_, target)) = force_graph
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

        for (name, pos) in force_graph.node_weights() {
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
