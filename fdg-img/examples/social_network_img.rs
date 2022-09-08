use std::fs;

use fdg_img::{
    style::{
        text_anchor::{HPos, Pos, VPos},
        Color, IntoFont, RGBAColor, TextStyle, BLACK,
    },
    Settings,
};
use fdg_sim::json;

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
        graph,
        Some(Settings {
            text_style,
            node_color: RGBAColor(100, 100, 100, 1.0),
            edge_color: RGBAColor(150, 150, 150, 1.0),
            ..Default::default()
        }),
    )
    .unwrap();

    fs::write("social_network.svg", svg.as_bytes()).unwrap();
}
