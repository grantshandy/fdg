use crate::Simulation;
use macroquad::prelude::*;

pub async fn run_window<D: Clone + PartialEq>(sim: &mut Simulation<D>) {
    let mut zoom: f32 = 4.0;

    loop {
        clear_background(LIGHTGRAY);

        let w = screen_width() * zoom;
        let h = screen_height() * zoom;

        set_camera(&Camera2D::from_display_rect(Rect::new(
            -(w / 2.0),
            -(h / 2.0),
            w,
            h,
        )));

        sim.visit_edges(|source, target| {
            draw_line(
                source.location.x,
                source.location.y,
                target.location.x,
                target.location.y,
                4.0,
                RED,
            );
        });

        sim.visit_nodes(|node| {
            draw_circle(node.location.x, node.location.y, 10.0, BLACK);
        });

        set_default_camera();
        draw_text(&format!("zoom: {:2}", zoom), 10.0, 20.0, 30.0, DARKGRAY);

        let mouse_wheel_y = mouse_wheel().1;

        if mouse_wheel_y < 0. {
            zoom -= 0.25;
            if zoom < 0.5 {
                zoom = 0.5;
            }
        }
        if mouse_wheel_y > 0. {
            zoom += 0.25;
        }

        sim.step(get_frame_time());
        next_frame().await;
    }
}
