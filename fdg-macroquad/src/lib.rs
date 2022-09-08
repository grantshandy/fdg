#![doc = include_str!("../README.md")]

use egui_macroquad::{
    egui::{self, Checkbox, CollapsingHeader, ComboBox, Slider},
    macroquad::prelude::*,
};
use fdg_sim::{
    force::{self, Value},
    glam::Vec3,
    json,
    petgraph::graph::NodeIndex,
    Dimensions, ForceGraph, Node, Simulation, SimulationParameters,
};
use serde::Serialize;
pub use serde_json::Value as JsonValue;

pub use {egui_macroquad::macroquad, fdg_sim};

pub async fn run_window<
    N: Serialize + Clone + Default + PartialEq,
    E: Serialize + Clone + Default + PartialEq,
>(
    graph: &ForceGraph<N, E>,
) {
    let mut sim = Simulation::from_graph(graph.clone(), SimulationParameters::default());

    let orig_params = sim.parameters().clone();
    let orig_graph = sim.get_graph().clone();
    let mut current_force = force::fruchterman_reingold(45.0, 0.975);

    let forces = vec![
        current_force.clone(),
        force::handy(45.0, 0.975, true, true),
        force::scale(),
        force::translate(),
    ];

    let mut dark = true;

    let mut sim_speed: u8 = 1;
    let mut zoom: f32 = 1.0;
    let mut json = false;

    let default_node_size = 5.0;
    let default_edge_size = 1.5;
    let mut node_size = default_node_size;
    let mut edge_size = default_edge_size;

    let mut angle: f32 = 0.0;
    let radius = 200.0;

    let mut orbit_speed: f32 = 1.0;
    let mut orbit = true;
    let mut show_grid = true;

    let mut show_edges = true;
    let mut show_nodes = true;

    let mut dragging_node: Option<NodeIndex> = None;
    let mut manual = false;
    let mut running = true;
    let default_step_length: f32 = 0.035;
    let mut step_length = default_step_length;

    let mut json_buffer = String::new();

    if json {
        json_buffer = update_json_buffer(sim.get_graph());
    }

    loop {
        // Draw background
        clear_background(if dark {
            Color::from_rgba(15, 23, 42, 255)
        } else {
            LIGHTGRAY
        });

        if is_key_down(KeyCode::R) {
            sim.reset_node_placement();
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

            let hovered_node = match sim.find(Vec3::new(mouse.0, mouse.1, 0.0), node_size) {
                Some(hovered) => {
                    if dragging_node.is_none() && is_mouse_button_down(MouseButton::Left) {
                        dragging_node = Some(hovered);
                    }

                    Some(hovered)
                }
                None => None,
            };

            if let Some(index) = dragging_node {
                let node = &mut sim.get_graph_mut()[index];

                if is_mouse_button_down(MouseButton::Left) {
                    node.locked = true;
                    node.location.x = mouse.0;
                    node.location.y = mouse.1;
                } else if is_mouse_button_released(MouseButton::Left) {
                    node.locked = false;
                    dragging_node = None;
                }
            }

            if show_edges {
                sim.visit_edges(&mut |source, target| {
                    draw_line(
                        source.location.x,
                        source.location.y,
                        target.location.x,
                        target.location.y,
                        edge_size,
                        RED,
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
                            mode_color_convert(dark, 0),
                            mode_color_convert(dark, 0),
                            mode_color_convert(dark, 0),
                            255,
                        ),
                    );
                });
            }

            if dragging_node.is_some() || hovered_node.is_some() {
                set_default_camera();

                if let Some(index) = dragging_node {
                    let node = &sim.get_graph()[index];
                    let screen_mouse = mouse_position();
                    draw_text(
                        &node.name,
                        screen_mouse.0 + 10.0,
                        screen_mouse.1 - 10.0,
                        30.0,
                        if dark { LIGHTGRAY } else { DARKBLUE },
                    );
                } else if let Some(index) = hovered_node {
                    let node: &Node<N> = &sim.get_graph()[index];
                    let screen_mouse = mouse_position();
                    draw_text(
                        &node.name,
                        screen_mouse.0 + 10.0,
                        screen_mouse.1 - 10.0,
                        30.0,
                        if dark { LIGHTGRAY } else { DARKBLUE },
                    );
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
                    draw_line_3d(
                        vec3(source.location.x, source.location.y, source.location.z),
                        vec3(target.location.x, target.location.y, target.location.z),
                        RED,
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
                            mode_color_convert(dark, 0),
                            mode_color_convert(dark, 0),
                            mode_color_convert(dark, 0),
                            255,
                        ),
                    );
                });
            }
        }

        // Draw gui
        egui_macroquad::ui(|egui_ctx| {
            if json {
                egui::Window::new("Json")
                    .anchor(egui::Align2::RIGHT_TOP, [-20.0, 20.0])
                    .fixed_size([200.0, 500.0])
                    .show(egui_ctx, |ui| {
                        egui::ScrollArea::new([true, true]).show(ui, |ui| {
                            ui.text_edit_multiline(&mut json_buffer);
                        });
                    });
            }

            egui::Window::new("Settings")
                .anchor(egui::Align2::LEFT_TOP, [20.0, 20.0])
                .fixed_size([50.0, 50.0])
                .show(egui_ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Restart Simulation").clicked() {
                            sim.set_graph(orig_graph.clone());
                            sim.reset_node_placement();

                            json_buffer = update_json_buffer(sim.get_graph());
                        }

                        if ui.button("Reset Settings").clicked() {
                            let mut p = sim.parameters_mut();
                            p.node_start_size = orig_params.node_start_size;
                            current_force.reset();
                            sim_speed = 1;
                            orbit_speed = 1.0;
                            zoom = 1.0;
                            node_size = default_node_size;
                            edge_size = default_edge_size;
                            step_length = default_step_length;
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
                    if ui
                        .button(if dark {
                            "Switch to Light Mode"
                        } else {
                            "Switch to Dark Mode"
                        })
                        .clicked()
                    {
                        if dark {
                            dark = false;
                        } else {
                            dark = true;
                        }
                    }
                    ui.separator();
                    if current_force.continuous() {
                        ui.add(Checkbox::new(&mut manual, "Manual"));
                        ui.add(Slider::new(&mut step_length, 0.001..=0.5).text("Step Length"));

                        if manual {
                            if ui.button("Step").clicked() {
                                sim.update_custom(&current_force, step_length);
                            }
                        } else {
                            ui.add(Slider::new(&mut sim_speed, 1..=10).text("Simulation Speed"));
                            let running_text = if running { "Stop" } else { "Start" };

                            if ui.button(running_text).clicked() {
                                if running {
                                    running = false;
                                } else {
                                    running = true;
                                }
                            }
                        }
                    } else if ui.button("Run").clicked() {
                        sim.update_custom(&current_force, 0.0);
                    }
                    ui.separator();
                    ui.add(Slider::new(&mut zoom, 0.05..=2.0).text("Zoom"));
                    match sim.parameters().dimensions {
                        Dimensions::Three => {
                            ui.add_enabled(
                                orbit,
                                Slider::new(&mut orbit_speed, 0.1..=5.0).text("Orbit Speed"),
                            );
                            ui.checkbox(&mut orbit, "Orbit");
                            ui.checkbox(&mut show_grid, "Show Grid");
                        }
                        Dimensions::Two => {
                            ui.add_enabled(
                                show_edges,
                                Slider::new(&mut edge_size, 1.0..=10.0).text("Edge Size"),
                            );
                        }
                    }
                    ui.add_enabled(
                        show_nodes,
                        Slider::new(&mut node_size, 1.0..=25.0).text("Node Size"),
                    );
                    ui.add(
                        Slider::new(&mut sim.parameters_mut().node_start_size, 0.5..=1000.0)
                            .text("Node Start Area"),
                    );
                    ui.checkbox(&mut show_edges, "Show Edges");
                    ui.checkbox(&mut show_nodes, "Show Nodes");
                    if ui.checkbox(&mut json, "Show Json").changed() {
                        json_buffer = update_json_buffer(sim.get_graph());
                    };
                    ui.separator();
                    ComboBox::new("force_selector", "")
                        .selected_text(current_force.name())
                        .show_ui(ui, |ui| {
                            for force in &forces {
                                ui.selectable_value(
                                    &mut current_force,
                                    force.clone(),
                                    force.name(),
                                );
                            }
                        });
                    if let Some(info) = current_force.info() {
                        CollapsingHeader::new("Info")
                            .default_open(false)
                            .show(ui, |ui| {
                                ui.label(info);
                            });
                    }
                    for (name, value) in current_force.dict_mut() {
                        match value {
                            Value::Number(value, range) => {
                                ui.add(Slider::new(value, range.clone()).text(name))
                            }
                            Value::Bool(value) => ui.add(Checkbox::new(value, name.to_string())),
                        };
                    }
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
        if running && !manual && current_force.continuous() {
            for _ in 0..sim_speed {
                sim.update_custom(&current_force, step_length);
            }
        }

        // draw gui
        egui_macroquad::draw();

        // go to next frame
        next_frame().await;
    }
}

fn mode_color_convert(dark: bool, i: u8) -> u8 {
    if dark {
        255 - i
    } else {
        i
    }
}

fn update_json_buffer<N: Serialize, E: Serialize>(graph: &ForceGraph<N, E>) -> String {
    match json::graph_to_json(graph) {
        Ok(s) => match serde_json::to_string_pretty(&s) {
            Ok(s) => s,
            Err(err) => format!("json string formatting error: {err}"),
        },
        Err(err) => format!("json serializing error: {err}"),
    }
}
