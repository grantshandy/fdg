use glam::Vec3;

use crate::ForceGraph;

/// Forces that dictate how your nodes move.
pub trait Force<D: Clone> {
    fn update(&self, graph: &mut ForceGraph<D>, dt: f32);
}

#[derive(Clone)]
pub struct FruchtermanReingold;

impl FruchtermanReingold {
    pub fn new<D: Clone>(scale: f32) -> Self {
        let dict = vec![
            ("Scale".to_string(), scale),
            ("Cooloff Factor".to_string(), 0.975),
        ];
    }
}

impl<D: Clone> Force<D> for FruchtermanReingold {
    fn update(&self, graph: &mut ForceGraph<D>, dt: f32) {
        let graph_clone = graph.clone();

        for node_index in graph_clone.node_indices() {
            if graph[node_index].locked {
                continue;
            }

            let mut final_force = Vec3::ZERO;

            for other_node_index in graph_clone.node_indices() {
                // skip duplicates
                if other_node_index == node_index {
                    continue;
                }

                let node_one = &graph_clone[node_index];
                let node_two = &graph_clone[other_node_index];

                final_force += (node_one.location.distance_squared(node_two.location)
                    / self.dict[0].1)
                    * ((node_two.location - node_one.location)
                        / node_one.location.distance(node_two.location));
            }

            for neighbor_index in graph_clone.neighbors(node_index) {
                let node_one = &graph_clone[node_index];
                let node_two = &graph_clone[neighbor_index];

                final_force += -((self.dict[0].1 * self.dict[0].1)
                    / node_one.location.distance(node_two.location))
                    * ((node_two.location - node_one.location)
                        / node_one.location.distance(node_two.location));
            }

            let node = &mut graph[node_index];

            let acceleration = final_force / node.mass;
            node.velocity += acceleration * dt;
            node.velocity *= self.dict[1].1;

            node.location += node.velocity * dt;
        }
    }
}