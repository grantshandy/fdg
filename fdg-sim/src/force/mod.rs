use crate::ForceGraph;
use std::ops::RangeInclusive;

mod force_atlas_2;
mod fruchterman_reingold;
mod handy;

pub use force_atlas_2::force_atlas_2;
pub use fruchterman_reingold::fruchterman_reingold;
pub use handy::handy;

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
pub struct Force<N: Clone, E: Clone> {
    dict: Vec<(&'static str, Value)>,
    dict_default: Vec<(&'static str, Value)>,
    name: &'static str,
    continuous: bool,
    info: Option<&'static str>,
    update: fn(dict: Vec<(&'static str, Value)>, graph: &mut ForceGraph<N, E>, dt: f32),
}

impl<N: Clone, E: Clone> PartialEq for Force<N, E> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<N: Clone, E: Clone> Force<N, E> {
    pub fn update(&self, graph: &mut ForceGraph<N, E>, dt: f32) {
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

pub fn scale<N: Clone, E: Clone>() -> Force<N, E> {
    fn update<N, E>(dict: Vec<(&'static str, Value)>, graph: &mut ForceGraph<N, E>, _dt: f32) {
        let scale = dict[0].1.number();

        for node in graph.node_weights_mut() {
            node.location *= scale;
        }
    }

    let dict = vec![("Scale Factor", Value::Number(1.5, 0.1..=2.0))];

    Force {
        dict: dict.clone(),
        dict_default: dict,
        name: "Scale",
        continuous: false,
        info: Some("Scales the layout around the center."),
        update,
    }
}

pub fn translate<N: Clone, E: Clone>() -> Force<N, E> {
    fn update<N, E>(dict: Vec<(&'static str, Value)>, graph: &mut ForceGraph<N, E>, _dt: f32) {
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
        info: Some("Moves the layout in any direction."),
        update,
    }
}
