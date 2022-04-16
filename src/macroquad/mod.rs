use crate::{Dimensions, ForceGraph, Simulation, SimulationParameters};
use macroquad::prelude::*;

const ZOOM_FACTOR: f32 = 1.0;

pub async fn run_window<D: Clone + PartialEq>(graph: ForceGraph<D>) {
    let mut sim = Simulation::from_graph(graph, Dimensions::Two, SimulationParameters::default());

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
    ((screen_width() / 2.0) + x) * ZOOM_FACTOR
}

fn y_to_macroquad(x: f32) -> f32 {
    ((screen_height() / 2.0) + x) * ZOOM_FACTOR
}
