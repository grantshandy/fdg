use crate::ForceGraph;
use macroquad::prelude::*;

pub async fn run_window<D>(graph: ForceGraph<D>) {
    loop {
        clear_background(BLACK);

        next_frame().await;
    }
}
