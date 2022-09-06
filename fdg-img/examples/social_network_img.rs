use std::fs;

use fdg_img::Settings;
use fdg_sim::json;
use plotters::style::{text_anchor::*, BLACK, *};

fn main() {
    let graph = json::graph_from_json(include_str!("../../datasets/social_network.json")).unwrap();

    let text_style = Some(TextStyle {
        font: ("sans-serif", 20).into_font(),
        color: BLACK.to_backend_color(),
        pos: Pos {
            h_pos: HPos::Left,
            v_pos: VPos::Center,
        },
    });

    let svg = fdg_img::gen_image(
        &graph,
        Some(Settings {
            text_style,
            node_color: (100, 100, 100, 1.0),
            edge_color: (150, 150, 150, 1.0),
            ..Default::default()
        }),
    )
    .unwrap();

    fs::write("social_network.svg", svg.as_bytes()).unwrap();
}
