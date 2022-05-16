use glam::Vec3;

use super::{Force, Value};

#[derive(Clone)]
pub struct Resize {
    dict: Vec<(&'static str, Value)>,
    dict_default: Vec<(&'static str, Value)>,
}

impl Resize {
    pub fn new() -> Self {
        let dict = vec![("Size", Value::Number(1.0, 0.1..=10.0))];

        Self {
            dict: dict.clone(),
            dict_default: dict,
        }
    }
}

impl Default for Resize {
    fn default() -> Self {
        let dict = vec![("Size", Value::Number(1.0, 0.1..=10.0))];

        Self {
            dict: dict.clone(),
            dict_default: dict,
        }
    }
}

impl<D: Clone> Force<D> for Resize {
    fn update(&self, graph: &mut crate::ForceGraph<D>, _dt: f32) {
        let mut avg_node = Vec3::ZERO;
        let graph_clone = graph.clone();
        let size = match self.dict[0].1 {
            Value::Number(n, _) => n,
            _ => panic!(),
        };

        for index in graph_clone.node_indices() {
            avg_node += graph_clone[index].location;
        }

        avg_node /= graph_clone.node_count() as f32;

        println!("{avg_node}");

        // for index in graph_clone.node_indices() {
        //     let node = &mut graph[index];

        //     let f = node.location + (avg_node * size);

        //     node.location = f;
        // }
    }

    fn dict_mut(&mut self) -> &mut [(&'static str, Value)] {
        &mut self.dict
    }

    fn dict(&self) -> &[(&'static str, Value)] {
        &self.dict
    }

    fn reset(&mut self) {
        self.dict = self.dict_default.clone()
    }

    fn name(&self) -> &'static str {
        "Resize"
    }

    fn continuous(&self) -> bool {
        false
    }
}
