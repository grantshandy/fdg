use std::fs;

use fdg_img::{
    style::{
        text_anchor::{HPos, Pos, VPos},
        Color, IntoFont, TextStyle, BLACK,
    },
    Settings,
};
use fdg_sim::json;

fn main() {
    let graph = json::graph_from_json(include_str!("../../datasets/les_miserables.json")).unwrap();

    let svg = fdg_img::gen_image(
        graph,
        Some(Settings {
            text_style: Some(TextStyle {
                font: ("sans-serif", 20).into_font(),
                color: BLACK.to_backend_color(),
                pos: Pos {
                    h_pos: HPos::Left,
                    v_pos: VPos::Center,
                },
            }),
            ..Default::default()
        }),
    )
    .unwrap();

    fs::write("json.svg", svg.as_bytes()).unwrap();
}
