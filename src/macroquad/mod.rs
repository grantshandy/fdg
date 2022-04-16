use crate::Simulation;
use macroquad::prelude::*;

pub async fn run_window<D: Clone + PartialEq>(sim: &mut Simulation<D>) {
    loop {
        clear_background(WHITE);

        sim.visit_nodes(|node| {
            let x = x_to_macroquad(node.location.x);
            let y = y_to_macroquad(node.location.y);

            draw_circle(x, y, 10.0, BLACK);
        });

        sim.step(get_frame_time());

        next_frame().await;
    }
}

fn x_to_macroquad(x: f32) -> f32 {
    (screen_width() / 2.0) + x
}

fn y_to_macroquad(x: f32) -> f32 {
    (screen_height() / 2.0) + x
}
