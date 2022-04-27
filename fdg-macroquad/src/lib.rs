use egui_macroquad::{egui, macroquad::prelude::*};
use fdg_sim::Simulation;

pub async fn run_window<D: Clone + PartialEq>(sim: &mut Simulation<D>) {
    let mut zoom: f32 = 2.0;
    let mut speed: f32 = 1.0;
    let orig_params = sim.parameters.clone();

    loop {
        if is_key_down(KeyCode::R) {
            sim.reset_node_placement();
        }

        // Update zoom
        {
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
        }

        // Draw background
        clear_background(LIGHTGRAY);

        // Set camera
        {
            let w = screen_width() * zoom;
            let h = screen_height() * zoom;

            set_camera(&Camera2D::from_display_rect(Rect::new(
                -(w / 2.0),
                -(h / 2.0),
                w,
                h,
            )));
        }

        // Draw edges and nodes
        {
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
                draw_circle(node.location.x, node.location.y, node.mass * 10.0, BLACK);
            });
        }

        // show label

        // Draw gui
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Settings")
                .default_size((50.0, 50.0))
                .show(egui_ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Restart Simulation").clicked() {
                            sim.reset_node_placement();
                        }

                        if ui.button("Reset Settings").clicked() {
                            sim.parameters = orig_params.clone();
                            speed = 1.0;
                        }
                    });
                    ui.separator();
                    ui.add(egui::Slider::new(&mut zoom, 0.5..=15.0).text("Zoom"));
                    ui.add(
                        egui::Slider::new(&mut sim.parameters.cooloff_factor, 0.0..=1.0)
                            .text("Cool-Off Factor"),
                    );
                    ui.add(
                        egui::Slider::new(&mut speed, 0.1..=5.0)
                            .text("Speed"),
                    );
                    ui.separator();
                    ui.horizontal(|ui| {
                        let g = sim.get_graph();
                        ui.label(format!("Node Count: {}", g.node_count()));
                        ui.separator();
                        ui.label(format!("Edge Count: {}", g.edge_count()));
                        ui.separator();
                        ui.label(format!("FPS: {}", get_fps()));
                    });
                });
        });

        // update sim
        sim.update(0.035 * speed);

        // draw gui
        egui_macroquad::draw();

        // go to next frame
        next_frame().await;
    }
}
