use glam::Vec3;

use crate::ForceGraph;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f32, RangeInclusive<f32>),
    Bool(bool),
}

impl Value {
    /// Retrieves the bool from a value. If you mess up and call it on a number it will return false.
    pub const fn bool(&self) -> bool {
        match self {
            Self::Bool(b) => *b,
            _ => false,
        }
    }

    /// Retrieves the number from a value. If you mess up and call it on a bool it will return 0.0.
    pub const fn number(&self) -> f32 {
        match self {
            Self::Number(n, _) => *n,
            _ => 0.0,
        }
    }
}

#[derive(Clone)]
pub struct Force<D: Clone> {
    dict: Vec<(&'static str, Value)>,
    dict_default: Vec<(&'static str, Value)>,
    name: &'static str,
    continuous: bool,
    info: Option<&'static str>,
    update: fn(dict: Vec<(&'static str, Value)>, graph: &mut ForceGraph<D>, dt: f32),
}

impl<D: Clone> Force<D> {
    pub fn update(&self, graph: &mut ForceGraph<D>, dt: f32) {
        (self.update)(self.dict.clone(), graph, dt);
    }

    pub fn dict_mut(&mut self) -> &mut [(&'static str, Value)] {
        &mut self.dict
    }

    pub fn dict(&self) -> &[(&'static str, Value)] {
        &self.dict
    }

    pub fn reset(&mut self) {
        self.dict = self.dict_default.clone();
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn continuous(&self) -> bool {
        self.continuous
    }

    pub fn info(&self) -> Option<&'static str> {
        self.info
    }
}

pub fn fruchterman_reingold<D: Clone>(scale: f32, cooloff_factor: f32) -> Force<D> {
    fn update<D: Clone>(dict: Vec<(&'static str, Value)>, graph: &mut ForceGraph<D>, dt: f32) {
        let graph_clone = graph.clone();

        let scale = dict[0].1.number();
        let cooloff_factor = dict[1].1.number();

        for node_index in graph_clone.node_indices() {
            if graph_clone[node_index].locked {
                continue;
            }

            let mut final_force = Vec3::ZERO;

            for other_node_index in graph_clone.node_indices() {
                if other_node_index == node_index {
                    continue;
                }

                let node_one = &graph_clone[node_index];
                let node_two = &graph_clone[other_node_index];

                final_force += -((scale * scale) / node_one.location.distance(node_two.location))
                    * ((node_two.location - node_one.location)
                        / node_one.location.distance(node_two.location))
            }

            for neighbor_index in graph_clone.neighbors(node_index) {
                let node_one = &graph_clone[node_index];
                let node_two = &graph_clone[neighbor_index];

                final_force += (node_one.location.distance_squared(node_two.location) / scale)
                    * ((node_two.location - node_one.location)
                        / node_one.location.distance(node_two.location))
            }

            let node = &mut graph[node_index];

            let acceleration = final_force / node.mass;
            node.velocity += acceleration * dt;
            node.velocity *= cooloff_factor;
            node.location += node.velocity * dt;
        }
    }

    let dict = vec![
        ("Scale", Value::Number(scale, 1.0..=200.0)),
        ("Cooloff Factor", Value::Number(cooloff_factor, 0.0..=1.0)),
    ];

    Force {
        dict: dict.clone(),
        dict_default: dict,
        name: "Fruchterman-Reingold (1991)",
        continuous: true,
        info: Some(
            "A force directed graph drawing algorithm based on Fruchterman-Reingold (1991).",
        ),
        update,
    }
}

pub fn scale<D: Clone>() -> Force<D> {
    fn update<D>(dict: Vec<(&'static str, Value)>, graph: &mut ForceGraph<D>, _dt: f32) {
        let scale = dict[0].1.number();

        for node in graph.node_weights_mut() {
            node.location *= scale;
        }
    }

    let dict = vec![("Scale Factor", Value::Number(1.5, 0.1..=10.0))];

    Force {
        dict: dict.clone(),
        dict_default: dict,
        name: "Scale",
        continuous: false,
        info: Some(
            "Scales the layout around the center.",
        ),
        update,
    }
}

pub fn translate<D: Clone>() -> Force<D> {
    fn update<D>(dict: Vec<(&'static str, Value)>, graph: &mut ForceGraph<D>, _dt: f32) {
        let distance = dict[0].1.number();

        for node in graph.node_weights_mut() {
            if dict[1].1.bool() {
                node.location.y -= distance;
            }

            if dict[2].1.bool() {
                node.location.y += distance;
            }

            if dict[3].1.bool() {
                node.location.x -= distance;
            }

            if dict[4].1.bool() {
                node.location.x += distance;
            }
        }
    }

    let dict = vec![
        ("Distance", Value::Number(7.0, 0.0..=100.0)),
        ("Up", Value::Bool(false)),
        ("Down", Value::Bool(false)),
        ("Left", Value::Bool(false)),
        ("Right", Value::Bool(false)),
    ];

    Force {
        dict: dict.clone(),
        dict_default: dict,
        name: "Translate",
        continuous: false,
        info: Some(
            "Moves the layout in any direction.",
        ),
        update,
    }
}