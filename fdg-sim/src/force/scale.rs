use crate::ForceGraph;

use super::{Force, Value};

#[derive(Clone, Debug, PartialEq)]
pub struct Scale {
    dict: Vec<(&'static str, Value)>,
    default_dict: Vec<(&'static str, Value)>,
}

impl Default for Scale {
    fn default() -> Self {
        let dict = vec![("Scale Factor", Value::Number(1.5, 0.1..=10.0))];

        Self {
            dict: dict.clone(),
            default_dict: dict,
        }
    }
}

impl<D: Clone> Force<D> for Scale {
    fn update(&self, graph: &mut ForceGraph<D>, _dt: f32) {
        let scale = self.dict[0].1.number();

        for node in graph.node_weights_mut() {
            node.location *= scale;
        }
    }

    fn dict_mut(&mut self) -> &mut [(&'static str, Value)] {
        &mut self.dict
    }

    fn dict(&self) -> &[(&'static str, Value)] {
        &self.dict
    }

    fn reset(&mut self) {
        self.dict = self.default_dict.clone();
    }

    fn name(&self) -> &'static str {
        "Scale"
    }

    fn continuous(&self) -> bool {
        false
    }

    fn info(&self) -> Option<&'static str> {
        Some("Scales the layout around the center.")
    }
}
