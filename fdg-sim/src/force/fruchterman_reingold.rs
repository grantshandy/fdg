use glam::Vec3;

use crate::ForceGraph;

use super::{Force, Value};

#[derive(Clone, Debug)]
pub struct FruchtermanReingold {
    dict: Vec<(&'static str, Value)>,
    dict_default: Vec<(&'static str, Value)>,
}

impl FruchtermanReingold {
    pub fn new(scale: f32, cooloff_factor: f32) -> Self {
        let dict = vec![
            ("Scale", Value::Number(scale, 1.0..=200.0)),
            ("Cooloff Factor", Value::Number(cooloff_factor, 0.0..=1.0)),
        ];

        Self {
            dict: dict.clone(),
            dict_default: dict,
        }
    }
}

impl Default for FruchtermanReingold {
    fn default() -> Self {
        Self::new(45.0, 0.975)
    }
}

impl<D: Clone> Force<D> for FruchtermanReingold {
    fn update(&self, graph: &mut ForceGraph<D>, dt: f32) {
        let graph_clone = graph.clone();

        let scale: f32 = match self.dict[0].1 {
            Value::Number(n, _) => n,
            _ => panic!(""),
        };
        let cooloff_factor = match self.dict[1].1 {
            Value::Number(n, _) => n,
            _ => panic!(""),
        };

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
        "Fruchterman-Reingold (1991)"
    }
}
