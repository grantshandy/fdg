use crate::ForceGraph;

use super::{Force, Value};

#[derive(Clone, Debug)]
pub struct Translate {
    dict: Vec<(&'static str, Value)>,
    dict_default: Vec<(&'static str, Value)>,
}

impl Default for Translate {
    fn default() -> Self {
        let dict = vec![
            ("Distance", Value::Number(7.0, 0.0..=100.0)),
            ("Up", Value::Bool(false)),
            ("Down", Value::Bool(false)),
            ("Left", Value::Bool(false)),
            ("Right", Value::Bool(false)),
        ];

        Self {
            dict: dict.clone(),
            dict_default: dict,
        }
    }
}

impl<D: Clone> Force<D> for Translate {
    fn update(&self, graph: &mut ForceGraph<D>, _dt: f32) {
        let distance = self.dict[0].1.number();

        for node in graph.node_weights_mut() {
            if self.dict[1].1.bool() {
                node.location.y -= distance;
            }
    
            if self.dict[2].1.bool() {
                node.location.y += distance;
            }
    
            if self.dict[3].1.bool() {
                node.location.x -= distance;
            }
    
            if self.dict[4].1.bool() {
                node.location.x += distance;
            }
        }
    }

    fn dict_mut(&mut self) -> &mut [(&'static str, Value)] {
        &mut self.dict
    }

    fn dict(&self) -> &[(&'static str, Value)] {
        &self.dict
    }

    fn reset(&mut self) {
        self.dict = self.dict_default.clone();
    }

    fn name(&self) -> &'static str {
        "Translate"
    }

    fn continuous(&self) -> bool {
        false
    }
}
