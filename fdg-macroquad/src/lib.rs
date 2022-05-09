use egui_macroquad::{egui, macroquad::prelude::*};
use fdg_sim::{Dimensions, Simulation, Vec3, petgraph::graph::NodeIndex};

pub async fn run_window<D: Clone + PartialEq>(sim: &mut impl Simulation<D>) {
    let orig_params = sim.parameters().clone();
    let ideal_distance = sim.forces().dict()[0];

    let mut zoom: f32 = 2.0;
    let mut sim_speed: u8 = 1;

    let default_node_size = 5.0;
    let default_edge_size = 1.5;
    let mut node_size = default_node_size;
    let mut edge_size = default_edge_size;

    let mut angle: f32 = 0.0;
    let radius = 200.0;

    let mut orbit_speed: f32 = 1.0;
    let mut orbit: bool = true;
    let mut show_grid: bool = true;

    let mut show_edges: bool = true;
    let mut show_nodes: bool = true;

    let mut dragging_node: Option<NodeIndex> = None;

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
        if sim.parameters().dimensions == Dimensions::Two {
            let w = screen_width() * (1.0 / zoom);
            let h = screen_height() * (1.0 / zoom);

            set_camera(&Camera2D::from_display_rect(Rect::new(
                -(w / 2.0),
                -(h / 2.0),
                w,
                h,
            )));

            let mut mouse = mouse_position();
            mouse.0 = (mouse.0 - (screen_width() / 2.0)) * (1.0 / zoom);
            mouse.1 = (mouse.1 - (screen_height() / 2.0)) * (1.0 / zoom);

            let hovered_node = if let Some(index) = sim.find(Vec3::new(mouse.0, mouse.1, 0.0), node_size * 1.5) {
                if is_mouse_button_down(MouseButton::Left) && dragging_node.is_none() {
                    dragging_node = Some(index);
                }

                Some(index)
            } else {
                None
            };

            if let Some(index) = dragging_node {
                let node = &mut sim.get_graph_mut()[index];

                if is_mouse_button_down(MouseButton::Left) {
                    node.locked = true;
                    node.color = [169, 169, 169, 255];
                    node.location.x = mouse.0;
                    node.location.y = mouse.1;
                } else if is_mouse_button_released(MouseButton::Left) {
                    node.locked = false;
                    node.color = [0, 0, 0, 255];
                    dragging_node = None;
                }
            }

            if show_edges {
                sim.visit_edges(&mut |source, target| {
                    let x = source.location.distance(target.location) / ideal_distance;
                    draw_line(
                        source.location.x,
                        source.location.y,
                        target.location.x,
                        target.location.y,
                        edge_size,
                        Color::new(x * x, 0.0, 0.0, 1.0 / (x / 2.0)),
                    );
                });
            }

            if show_nodes {
                sim.visit_nodes(&mut |node| {
                    draw_circle(
                        node.location.x,
                        node.location.y,
                        node_size,
                        Color::from_rgba(
                            node.color[0],
                            node.color[1],
                            node.color[2],
                            node.color[3],
                        ),
                    );
                });
            }

            if dragging_node.is_some() || hovered_node.is_some() {
                set_default_camera();

                if let Some(index) = dragging_node {
                    let node = &sim.get_graph()[index];
                    let screen_mouse = mouse_position();
                    draw_text(&node.name, screen_mouse.0 + 10.0, screen_mouse.1 - 10.0, 30.0, DARKBLUE);
                } else if let Some(index) = hovered_node {
                    let node = &sim.get_graph()[index];
                    let screen_mouse = mouse_position();
                    draw_text(&node.name, screen_mouse.0 + 10.0, screen_mouse.1 - 10.0, 30.0, DARKBLUE);
                }
            }
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

            if show_edges {
                sim.visit_edges(&mut |source, target| {
                    let x = source.location.distance(target.location) / ideal_distance;
                    draw_line_3d(
                        vec3(source.location.x, source.location.y, source.location.z),
                        vec3(target.location.x, target.location.y, target.location.z),
                        Color::new(x * x, 0.0, 0.0, 1.0 / (x / 2.0)),
                    );
                });
            }

            if show_nodes {
                sim.visit_nodes(&mut |node| {
                    draw_sphere(
                        vec3(node.location.x, node.location.y, node.location.z),
                        node_size,
                        None,
                        Color::from_rgba(
                            node.color[0],
                            node.color[1],
                            node.color[2],
                            node.color[3],
                        ),
                    );
                });
            }
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
                            let mut p = sim.parameters_mut();
                            p.cooloff_factor = orig_params.cooloff_factor;
                            p.node_start_size = orig_params.node_start_size;
                            sim_speed = 1;
                            orbit_speed = 1.0;
                            zoom = 1.0;
                            node_size = default_node_size;
                            edge_size = default_edge_size;
                        }

                        if ui
                            .button(match sim.parameters().dimensions {
                                Dimensions::Two => "View in 3D",
                                Dimensions::Three => "View in 2D",
                            })
                            .clicked()
                        {
                            sim.parameters_mut().dimensions = match sim.parameters().dimensions {
                                Dimensions::Two => Dimensions::Three,
                                Dimensions::Three => Dimensions::Two,
                            };

                            sim.reset_node_placement();
                        }
                    });
                    ui.separator();
                    ui.add(egui::Slider::new(&mut zoom, 0.05..=5.0).text("Zoom"));
                    match sim.parameters().dimensions {
                        Dimensions::Three => {
                            ui.add_enabled(
                                orbit,
                                egui::Slider::new(&mut orbit_speed, 0.1..=5.0).text("Orbit Speed"),
                            );
                            ui.checkbox(&mut orbit, "Orbit");
                            ui.checkbox(&mut show_grid, "Show Grid");
                        }
                        Dimensions::Two => {
                            ui.add_enabled(
                                show_edges,
                                egui::Slider::new(&mut edge_size, 1.0..=10.0).text("Edge Size"),
                            );
                        }
                    }
                    ui.add_enabled(
                        show_nodes,
                        egui::Slider::new(&mut node_size, 1.0..=25.0).text("Node Size"),
                    );
                    ui.checkbox(&mut show_edges, "Show Edges");
                    ui.checkbox(&mut show_nodes, "Show Nodes");
                    ui.separator();
                    ui.add(
                        egui::Slider::new(&mut sim.parameters_mut().cooloff_factor, 0.0..=1.0)
                            .text("Cool-Off Factor"),
                    );
                    ui.add(
                        egui::Slider::new(&mut sim.parameters_mut().node_start_size, 0.5..=1000.0)
                            .text("Node Start Area"),
                    );
                    ui.add(egui::Slider::new(&mut sim_speed, 1..=6).text("Simulation Speed"));
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
        for _ in 0..sim_speed {
            sim.update(0.035);
        }

        // draw gui
        egui_macroquad::draw();

        // go to next frame
        next_frame().await;
    }
}
