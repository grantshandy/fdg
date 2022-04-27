use egui_macroquad::{egui, macroquad::prelude::*};
use fdg_sim::SimulationParameters;
pub use fdg_sim::{Dimensions, Simulation};

pub async fn run_window<D: Clone + PartialEq>(sim: &mut Simulation<D>) {
    let orig_params = sim.parameters.clone();

    let mut zoom: f32 = 2.0;
    let mut sim_speed: f32 = 1.0;

    let mut angle: f32 = 0.0;
    let radius = 200.0;

    let mut orbit_speed: f32 = 1.0;
    let mut orbit: bool = true;
    let mut show_grid: bool = true;

    loop {
        // Draw background
        clear_background(LIGHTGRAY);

        if is_key_down(KeyCode::R) {
            sim.reset_node_placement();
        }

        let mouse_wheel_y = mouse_wheel().1;

        if mouse_wheel_y < 0. {
            zoom -= 0.025;
            if zoom < 0.05 {
                zoom = 0.05;
            }
        }

        if mouse_wheel_y > 0. {
            zoom += 0.025;
        }

        // Draw edges and nodes
        if sim.parameters.dimensions == Dimensions::Two {
            let w = screen_width() * (1.0 / zoom);
            let h = screen_height() * (1.0 / zoom);

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
                    2.5,
                    RED,
                );
            });

            sim.visit_nodes(|node| {
                draw_circle(node.location.x, node.location.y, node.mass * 10.0, BLACK);
            });
        } else {
            let adj_radius = radius * (1.0 / (zoom / 2.0));
            let (x, y) = (adj_radius * angle.cos(), adj_radius * angle.sin());

            if orbit {
                angle += 0.0015 * orbit_speed;
            }

            set_camera(&Camera3D {
                position: vec3(x, radius * 1.5, y),
                up: vec3(0., 1.0, 0.),
                target: vec3(0.0, 0.0, 0.0),
                ..Default::default()
            });

            if show_grid {
                draw_grid(200, 25.0, DARKBLUE, GRAY);
            }

            sim.visit_edges(|source, target| {
                draw_line_3d(
                    vec3(source.location.x, source.location.y, source.location.z),
                    vec3(target.location.x, target.location.y, target.location.z),
                    RED,
                );
            });

            sim.visit_nodes(|node| {
                draw_sphere(
                    vec3(node.location.x, node.location.y, node.location.z),
                    node.mass * 5.0,
                    None,
                    BLACK,
                );
            });
        }

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
                            sim.parameters = SimulationParameters {
                                dimensions: sim.parameters.dimensions,
                                ..orig_params.clone()
                            };
                            sim_speed = 1.0;
                            orbit_speed = 1.0;
                            zoom = 1.0;
                        }

                        if ui
                            .button(match sim.parameters.dimensions {
                                Dimensions::Two => "View in 3D",
                                Dimensions::Three => "View in 2D",
                            })
                            .clicked()
                        {
                            sim.parameters.dimensions = match sim.parameters.dimensions {
                                Dimensions::Two => Dimensions::Three,
                                Dimensions::Three => Dimensions::Two,
                            };

                            sim.reset_node_placement();
                        }
                    });
                    ui.separator();
                    ui.add(egui::Slider::new(&mut zoom, 0.05..=5.0).text("Zoom"));
                    if sim.parameters.dimensions == Dimensions::Three {
                        ui.add(egui::Slider::new(&mut orbit_speed, 0.1..=5.0).text("Orbit Speed"));
                        ui.checkbox(&mut orbit, "Orbit");
                        ui.checkbox(&mut show_grid, "Show Grid");
                    }
                    ui.separator();
                    ui.add(
                        egui::Slider::new(&mut sim.parameters.cooloff_factor, 0.0..=1.0)
                            .text("Cool-Off Factor"),
                    );
                    ui.add(egui::Slider::new(&mut sim_speed, 0.1..=5.0).text("Simulation Speed"));
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
        sim.update(0.035 * sim_speed);

        // draw gui
        egui_macroquad::draw();

        // go to next frame
        next_frame().await;
    }
}
