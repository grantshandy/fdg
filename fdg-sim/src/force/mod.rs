use crate::ForceGraph;
use std::ops::RangeInclusive;

mod fruchterman_reingold;
mod handy;

pub use {fruchterman_reingold::fruchterman_reingold, handy::handy};

/// An entry in a [`Force`]'s dictionary.
#[derive(Clone, Debug, PartialEq)]
pub struct DictionaryEntry {
    pub name: &'static str,
    pub value: ForceValue,
}

impl DictionaryEntry {
    /// Create a new [`DictionaryEntry`].
    pub fn new(name: &'static str, value: ForceValue) -> Self {
        Self { name, value }
    }

    /// Retrieve a mutable reference to the value
    pub fn value_mut(&mut self) -> &mut ForceValue {
        &mut self.value
    }
}

/// A value that you can change in a [`Force`]'s dictionary.
#[derive(Clone, Debug, PartialEq)]
pub enum ForceValue {
    Number(f32, RangeInclusive<f32>),
    Bool(bool),
}

impl ForceValue {
    /// Retrieves the bool from a value.
    pub const fn bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Same as bool but returns a mutable version.
    pub fn bool_mut(&mut self) -> Option<&mut bool> {
        match self {
            Self::Bool(b) => Some(b),
            _ => None,
        }
    }

    /// Retrieves the number from a value. If you mess up and call it on a bool it will return 0.0.
    pub const fn number(&self) -> Option<f32> {
        match self {
            Self::Number(n, _) => Some(*n),
            _ => None,
        }
    }

    /// Same as number but returns a mutable version.
    pub fn number_mut(&mut self) -> Option<&mut f32> {
        match self {
            Self::Number(n, _) => Some(n),
            _ => None,
        }
    }
}

// A struct that defines how your force behaves
#[derive(Clone)]
pub struct Force<N: Clone, E: Clone> {
    dict: Vec<DictionaryEntry>,
    dict_default: Vec<DictionaryEntry>,
    name: &'static str,
    continuous: bool,
    info: Option<&'static str>,
    update: fn(dict: &[DictionaryEntry], graph: &mut ForceGraph<N, E>, dt: f32),
}

impl<N: Clone, E: Clone> Force<N, E> {
    /// Retrieve the name of the force.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Retrieve the force's information.
    pub fn info(&self) -> Option<&'static str> {
        self.info
    }

    /// Update the graph's node's positions for a given interval.
    pub fn update(&self, graph: &mut ForceGraph<N, E>, dt: f32) {
        (self.update)(&self.dict, graph, dt);
    }

    /// Retrieve a mutable reference to the force's internal dictionary.
    pub fn dict_mut(&mut self) -> &mut [DictionaryEntry] {
        &mut self.dict
    }

    /// Retrieve a reference to the force's internal dictionary.
    pub fn dict(&self) -> &[DictionaryEntry] {
        &self.dict
    }

    /// Reset the force's internal dictionary.
    pub fn reset(&mut self) {
        self.dict = self.dict_default.clone();
    }

    /// Retrieve if the force is continuous.
    /// Continuous forces run their update on every frame, non-continuous forces run their update every time the user clicks a "Run" button.
    pub fn continuous(&self) -> bool {
        self.continuous
    }
}

impl<N: Clone, E: Clone> PartialEq for Force<N, E> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

/// A force for scaling the layout around the center of the graph.
pub fn scale<N: Clone, E: Clone>() -> Force<N, E> {
    fn update<N, E>(dict: &[DictionaryEntry], graph: &mut ForceGraph<N, E>, _dt: f32) {
        let scale = dict[0].value.number().unwrap();

        for node in graph.node_weights_mut() {
            node.location *= scale;
        }
    }

    let dict = vec![DictionaryEntry::new(
        "Scale Factor",
        ForceValue::Number(1.5, 0.1..=2.0),
    )];

    Force {
        dict: dict.clone(),
        dict_default: dict,
        name: "Scale",
        continuous: false,
        info: Some("Scales the layout around the center of the graph."),
        update,
    }
}

/// A force for translating the graph in any direction.
pub fn translate<N: Clone, E: Clone>() -> Force<N, E> {
    fn update<N, E>(dict: &[DictionaryEntry], graph: &mut ForceGraph<N, E>, _dt: f32) {
        let distance = dict[0].value.number().unwrap();

        for node in graph.node_weights_mut() {
            if dict[1].value.bool().unwrap() {
                node.location.y -= distance;
            }

            if dict[2].value.bool().unwrap() {
                node.location.y += distance;
            }

            if dict[3].value.bool().unwrap() {
                node.location.x -= distance;
            }

            if dict[4].value.bool().unwrap() {
                node.location.x += distance;
            }
        }
    }

    let dict = vec![
        DictionaryEntry::new("Distance", ForceValue::Number(7.0, 0.0..=100.0)),
        DictionaryEntry::new("Up", ForceValue::Bool(false)),
        DictionaryEntry::new("Down", ForceValue::Bool(false)),
        DictionaryEntry::new("Left", ForceValue::Bool(false)),
        DictionaryEntry::new("Right", ForceValue::Bool(false)),
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
