#![doc = include_str!("../README.md")]

use std::error::Error;

use fdg_sim::{
    force::Force,
    glam::Vec3,
    petgraph::visit::{EdgeRef, IntoEdgeReferences},
    ForceGraph, Simulation, SimulationParameters,
};
use plotters::prelude::*;

/// Parameters for drawing the SVG image.
pub struct Settings {
    pub iterations: usize,
    pub dt: f32,
    pub node_size: u32,
    pub node_color: (u8, u8, u8),
    pub edge_size: u32,
    pub edge_color: (u8, u8, u8),
    pub background_color: (u8, u8, u8),
    pub print_progress: bool,
    ///If supplied, the names of nodes will be written
    pub text_style: Option<TextStyle<'static>>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            iterations: 2000,
            dt: 0.035,
            node_size: 1,
            node_color: (0, 0, 0),
            edge_size: 3,
            edge_color: (255, 0, 0),
            background_color: (255, 255, 255),
            print_progress: true,
            text_style: None,
        }
    }
}

/// Generate an image from a graph and a force.
pub fn gen_image<N: Clone, E: Clone>(
    graph: &ForceGraph<N, E>,
    force: &Force<N, E>,
    settings: Option<Settings>,
) -> Result<String, Box<dyn Error>> {
    let settings = match settings {
        Some(settings) => settings,
        None => Settings::default(),
    };

    let mut sim = Simulation::from_graph(graph, SimulationParameters::default());
    sim.parameters_mut().set_force(force.clone());

    for i in 0..settings.iterations {
        if settings.print_progress && i % 10 == 0 {
            println!("{}/{}", i, settings.iterations);
        }
        sim.update(settings.dt);
    }

    // get the size of the graph (avg of width and height to account for oddly shaped graphs)
    let graph_size: f32 = {
        let mut top = 0.0;
        let mut bottom = 0.0;
        let mut left = 0.0;
        let mut right = 0.0;

        for node in sim.get_graph().node_weights() {
            let loc = node.location;

            let rightmost = match settings.text_style.clone() {
                //Make sure that the text isn't cut off
                Some(ts) => {
                    loc.x
                        + ts.font
                            .box_size(&node.name)
                            .ok()
                            .map(|x| x.0 as f32)
                            .unwrap_or(0.0)
                }
                None => loc.x,
            };

            if rightmost > right {
                right = rightmost;
            }

            if loc.x < left {
                left = loc.x;
            }

            if loc.y > top {
                top = loc.y
            }

            if loc.y < bottom {
                bottom = loc.y;
            }
        }

        let sum = (right - left) + (top - bottom);

        sum / 2.0
    };

    let f = 1.5;

    let image_size = ((graph_size * f) as u32, (graph_size * f) as u32);

    // translate all points to center
    let mut location_sum = Vec3::ZERO;
    for node in sim.get_graph().node_weights() {
        location_sum += node.location;
    }

    let avg_vec = location_sum / sim.get_graph().node_count() as f32;
    for node in sim.get_graph_mut().node_weights_mut() {
        node.location -= avg_vec;
    }

    // translate all the points over into image coordinate space
    for node in sim.get_graph_mut().node_weights_mut() {
        node.location.x += (image_size.0 / 2) as f32;
        node.location.y += (image_size.1 / 2) as f32;
    }

    let mut buffer = String::new();

    let backend = SVGBackend::with_string(&mut buffer, image_size).into_drawing_area();

    backend
        .fill(&RGBAColor(
            settings.background_color.0,
            settings.background_color.1,
            settings.background_color.2,
            1.0,
        ))
        .unwrap();

    for edge in sim.get_graph().edge_references() {
        let source = &sim.get_graph()[edge.source()].location;
        let target = &sim.get_graph()[edge.target()].location;

        backend.draw(&PathElement::new(
            vec![
                (source.x as i32, source.y as i32),
                (target.x as i32, target.y as i32),
            ],
            ShapeStyle {
                color: RGBAColor(
                    settings.edge_color.0,
                    settings.edge_color.1,
                    settings.edge_color.2,
                    1.0,
                ),
                filled: true,
                stroke_width: settings.edge_size,
            },
        ))?;
    }

    for node in sim.get_graph().node_weights() {
        backend.draw(&Circle::new(
            (node.location.x as i32, node.location.y as i32),
            settings.node_size * 10,
            ShapeStyle {
                color: RGBAColor(
                    settings.node_color.0,
                    settings.node_color.1,
                    settings.node_color.2,
                    1.0,
                ),
                filled: true,
                stroke_width: 1,
            },
        ))?;
    }

    if let Some(text_style) = settings.text_style {
        for node in sim.get_graph().node_weights() {
            let pos = (
                node.location.x as i32 + (text_style.font.get_size() / 2.) as i32,
                node.location.y as i32,
            );
            backend.draw_text(node.name.as_str(), &text_style, pos)?;
        }
    }

    drop(backend);

    Ok(buffer)
}
