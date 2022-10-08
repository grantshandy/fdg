#![doc = include_str!("../README.md")]

pub use egui_macroquad::macroquad;
pub use fdg_sim;

use egui_macroquad::egui::{Align2, Checkbox, CollapsingHeader, ComboBox, Slider, Window};
use egui_macroquad::macroquad::prelude::*;
use fdg_sim::force::{self, Force, Value};
use fdg_sim::glam::Vec3;
use fdg_sim::petgraph::stable_graph::NodeIndex;
use fdg_sim::{Dimensions, ForceGraph, Node, Simulation, SimulationParameters};

struct ApplicationState<N, E> {
    sim: Simulation<N, E>,
    current_force: Force<N, E>,
    available_forces: Vec<Force<N, E>>,
    dark_mode: bool,
    sim_speed: u8,
    zoom: f32,
    show_nodes: bool,
    show_edges: bool,
    show_grid: bool,
    node_size: f32,
    edge_size: f32,
    view_angle: f32,
    view_radius: f32,
    orbit: bool,
    orbit_speed: f32,
    current_dragging_node: Option<NodeIndex>,
    manual_mode: bool,
    step_time: f32,
    running: bool,
}

impl<N, E> Default for ApplicationState<N, E> {
    fn default() -> Self {
        let sim: Simulation<N, E> = Simulation::default();

        let default_scale: f32 = 45.0;
        let default_cooloff: f32 = 0.975;

        let available_forces = vec![
            force::handy(default_scale, default_cooloff, true, true),
            force::fruchterman_reingold(default_scale, default_cooloff),
            force::scale(),
            force::translate(),
        ];
        let current_force = force::handy(default_scale, default_cooloff, true, true);

        Self {
            sim,
            current_force,
            available_forces,
            dark_mode: true,
            sim_speed: 1,
            zoom: 1.0,
            show_nodes: true,
            show_edges: true,
            show_grid: true,
            node_size: 5.0,
            edge_size: 1.5,
            view_angle: 0.0,
            view_radius: 200.0,
            orbit: true,
            orbit_speed: 1.0,
            current_dragging_node: None,
            manual_mode: false,
            step_time: 0.035,
            running: true,
        }
    }
}

impl<N: Clone, E: Clone> ApplicationState<N, E> {
    pub fn new(graph: ForceGraph<N, E>) -> Self {
        let sim = Simulation::from_graph(graph, SimulationParameters::default());

        Self {
            sim,
            ..Default::default()
        }
    }

    pub async fn run(&mut self) {
        loop {
            // clear the background
            clear_background(if self.dark_mode {
                Color::from_rgba(15, 23, 42, 255)
            } else {
                LIGHTGRAY
            });

            if is_key_down(KeyCode::R) {
                self.sim.reset_node_placement();
            }

            self.render_graph();
            self.draw_gui();

            // update the simulation
            if self.running && !self.manual_mode && self.current_force.continuous() {
                for _ in 0..self.sim_speed {
                    self.sim.update_custom(&self.current_force, self.step_time);
                }
            }

            egui_macroquad::draw();

            // go to the next frame
            next_frame().await;
        }
    }

    fn render_graph(&mut self) {
        if self.sim.parameters().dimensions == Dimensions::Two {
            let zoom_factor = 1.0 / self.zoom;

            // update dragging node's status
            let (mut mouse_x, mut mouse_y) = mouse_position();
            mouse_x = (mouse_x - (screen_width() / 2.0)) * zoom_factor;
            mouse_y = (mouse_y - (screen_height() / 2.0)) * zoom_factor;

            // dragging node and hovering node are different because
            // sometimes the dragging node won't catch up to the mouse
            // cursor so we have to keep it as dragging until we release
            // the mouse

            // not the best explanation but you get the idea

            // node we are hovering over on this frame
            let hovered_node = match self
                .sim
                .find(Vec3::new(mouse_x, mouse_y, 0.0), self.node_size * 1.5)
            {
                Some(hovered_node) => {
                    // initialize dragging node if we're clicking
                    if self.current_dragging_node.is_none()
                        && is_mouse_button_down(MouseButton::Left)
                    {
                        self.current_dragging_node = Some(hovered_node)
                    }

                    Some(hovered_node)
                }
                None => None,
            };

            let w = screen_width() * zoom_factor;
            let h = screen_height() * zoom_factor;

            // set camera position with simulation position
            // (0,0) as the center of the screen (accounting for zoom)
            set_camera(&Camera2D::from_display_rect(Rect::new(
                -(w / 2.0),
                -(h / 2.0),
                w,
                h,
            )));

            // draw edges and nodes
            if self.show_edges {
                self.sim.visit_edges(&mut |source, target| {
                    draw_line(
                        source.location.x,
                        source.location.y,
                        target.location.x,
                        target.location.y,
                        self.edge_size,
                        RED,
                    );
                });
            }

            if self.show_nodes {
                self.sim.visit_nodes(&mut |node| {
                    draw_circle(
                        node.location.x,
                        node.location.y,
                        self.node_size,
                        get_node_color(self.dark_mode),
                    );
                });
            }

            // turn off or on the current dragging node based on clicking and hovering
            if let Some(index) = self.current_dragging_node {
                let node = &mut self
                    .sim
                    .get_graph_mut()
                    .node_weight_mut(index)
                    .expect("sim.find didn't return a valid index");

                draw_node_text(&node, self.dark_mode);

                if is_mouse_button_down(MouseButton::Left) {
                    set_default_camera();
                    
                    node.old_location.x = mouse_x;
                    node.old_location.y = mouse_y;

                    node.velocity = Vec3::ZERO;
                    node.velocity = Vec3::ZERO;

                    node.location.x = mouse_x;
                    node.location.y = mouse_y;
                } else if is_mouse_button_released(MouseButton::Left) {
                    self.current_dragging_node = None;
                }
            } else if let Some(index) = hovered_node {
                let node = self
                    .sim
                    .get_graph()
                    .node_weight(index)
                    .expect("sim.find didn't return a valid index");

                draw_node_text(&node, self.dark_mode);
            }
        } else {
            // set 3D camera position with some fun trig math
            let adj_radius = self.view_radius * (1.0 / (self.zoom / 2.0));
            let (x, y) = (
                adj_radius * self.view_angle.cos(),
                adj_radius * self.view_angle.sin(),
            );

            if self.orbit {
                self.view_angle += 0.0015 * self.orbit_speed;
            }

            set_camera(&Camera3D {
                position: vec3(x, self.view_radius * 1.5, y),
                up: vec3(0.0, 1.0, 0.0),
                target: vec3(0.0, 0.0, 0.0),
                ..Default::default()
            });

            if self.show_grid {
                draw_grid(200, 25.0, DARKBLUE, GRAY);
            }

            if self.show_edges {
                self.sim.visit_edges(&mut |source, target| {
                    draw_line_3d(
                        glam_to_macroquad_vec3(source.location),
                        glam_to_macroquad_vec3(target.location),
                        RED,
                    );
                });
            }

            if self.show_nodes {
                self.sim.visit_nodes(&mut |node| {
                    draw_sphere(
                        glam_to_macroquad_vec3(node.location),
                        self.node_size,
                        None,
                        get_node_color(self.dark_mode),
                    );
                });
            }
        }
    }

    fn draw_gui(&mut self) {
        egui_macroquad::ui(|ctx| {
            Window::new("Settings")
                .anchor(Align2::LEFT_TOP, [20.0, 20.0])
                .fixed_size([50.0, 50.0])
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Restart Simulation").clicked() {
                            self.sim.reset_node_placement();
                        }

                        if ui.button("Reset Settings").clicked() {
                            let default_state: ApplicationState<N, E> = ApplicationState::default();

                            self.current_force.reset();

                            self.orbit_speed = default_state.orbit_speed;
                            self.show_grid = default_state.show_grid;
                            self.sim.parameters_mut().node_start_size =
                                default_state.sim.parameters().node_start_size;
                            self.sim.parameters_mut().dimensions =
                                default_state.sim.parameters().dimensions;
                            self.sim_speed = default_state.sim_speed;
                            self.zoom = default_state.zoom;
                            self.node_size = default_state.node_size;
                            self.edge_size = default_state.edge_size;
                            self.step_time = default_state.step_time;
                        }

                        let current_dimensions = self.sim.parameters().dimensions;

                        if ui
                            .button(match current_dimensions {
                                Dimensions::Two => "View in 3D",
                                Dimensions::Three => "View in 2D",
                            })
                            .clicked()
                        {
                            self.sim.parameters_mut().dimensions = match current_dimensions {
                                Dimensions::Two => Dimensions::Three,
                                Dimensions::Three => Dimensions::Two,
                            };

                            self.sim.reset_node_placement();
                        }
                    });
                    ui.separator();
                    if ui
                        .button(if self.dark_mode {
                            "Switch to Light Mode"
                        } else {
                            "Switch to Dark Mode"
                        })
                        .clicked()
                    {
                        if self.dark_mode {
                            self.dark_mode = false;
                        } else {
                            self.dark_mode = true;
                        }
                    }
                    ui.separator();
                    if self.current_force.continuous() {
                        ui.add(Checkbox::new(&mut self.manual_mode, "Manual Mode"));
                        ui.add(Slider::new(&mut self.step_time, 0.001..=0.5).text("Step Time"));

                        if self.manual_mode {
                            if ui.button("Step").clicked() {
                                self.sim.update_custom(&self.current_force, self.step_time);
                            }
                        } else {
                            ui.add(
                                Slider::new(&mut self.sim_speed, 1..=10).text("Simulation Speed"),
                            );

                            if ui
                                .button(if self.running { "Stop" } else { "Start" })
                                .clicked()
                            {
                                if self.running {
                                    self.running = false;
                                } else {
                                    self.running = true;
                                }
                            }
                        }
                    } else if ui.button("Run").clicked() {
                        self.sim.update_custom(&self.current_force, 0.0);
                    }
                    ui.separator();
                    ui.add(Slider::new(&mut self.zoom, 0.05..=2.0).text("Zoom"));
                    match self.sim.parameters().dimensions {
                        Dimensions::Three => {
                            ui.add_enabled(
                                self.orbit,
                                Slider::new(&mut self.orbit_speed, 0.1..=5.0).text("Orbit Speed"),
                            );
                            ui.checkbox(&mut self.orbit, "Orbit");
                            ui.checkbox(&mut self.show_grid, "Show Grid");
                        }
                        Dimensions::Two => {
                            ui.add_enabled(
                                self.show_edges,
                                Slider::new(&mut self.edge_size, 1.0..=10.0).text("Edge Size"),
                            );
                        }
                    }
                    ui.add_enabled(
                        self.show_nodes,
                        Slider::new(&mut self.node_size, 1.0..=25.0).text("Node Size"),
                    );
                    ui.add(
                        Slider::new(&mut self.sim.parameters_mut().node_start_size, 0.5..=1000.0)
                            .text("Node Start Area"),
                    );
                    ui.checkbox(&mut self.show_nodes, "Show Nodes");
                    ui.checkbox(&mut self.show_edges, "Show Edges");
                    ui.separator();

                    // box for choosing what force to use
                    ComboBox::new("force_selector", "")
                        .selected_text(self.current_force.name())
                        .show_ui(ui, |ui| {
                            for force in self.available_forces.iter() {
                                ui.selectable_value(
                                    &mut self.current_force,
                                    force.clone(),
                                    force.name(),
                                );
                            }
                        });
                    // optional info for the force
                    if let Some(info) = self.current_force.info() {
                        CollapsingHeader::new("Info")
                            .default_open(false)
                            .show(ui, |ui| {
                                ui.label(info);
                            });
                    }

                    // properties for the current force
                    for (name, value) in self.current_force.dict_mut() {
                        match value {
                            Value::Number(value, range) => {
                                ui.add(Slider::new(value, range.clone()).text(name));
                            }
                            Value::Bool(value) => {
                                ui.add(Checkbox::new(value, name.as_str()));
                            }
                        };
                    }
                    
                    // final bit of information
                    ui.separator();
                    ui.horizontal(|ui| {
                       let graph = self.sim.get_graph();
                        
                        ui.label(format!("Node Count: {}", graph.node_count()));
                        ui.separator();
                        ui.label(format!("Edge Count: {}", graph.edge_count()));
                        ui.separator();
                        ui.label(format!("FPS: {}", get_fps()));
                    });
                });
        });
    }
}

pub async fn run_window<N: Clone, E: Clone>(graph: &ForceGraph<N, E>) {
    let mut window = ApplicationState::new(graph.clone());
    window.run().await;
}

fn get_node_color(dark: bool) -> Color {
    let color: u8 = if dark { 255 } else { 0 };

    Color::from_rgba(color, color, color, 255)
}

fn glam_to_macroquad_vec3(v: fdg_sim::glam::Vec3) -> macroquad::math::Vec3 {
    vec3(v.x, v.y, v.z)
}

fn draw_node_text<N>(node: &Node<N>, dark_mode: bool) {
    set_default_camera();

    let (screen_x, screen_y) = mouse_position();
    let offset = 10.0;

    draw_text(
        &node.name,
        screen_x + offset,
        screen_y - offset,
        30.0,
        if dark_mode { LIGHTGRAY } else { DARKBLUE },
    );
}
