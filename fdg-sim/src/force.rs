use glam::Vec3;

use crate::ForceGraph;

/// Forces that dictate how your nodes move.
#[derive(Clone)]
pub struct Force<D, F> {
    internal_update: fn(&mut F, f32, Vec<(String, f32)>, &mut ForceGraph<D>),
    // For internal data, e.g. shaders, cache, etc.
    internal_data: F,
    // For data you want the user to be able to edit in real time.
    pub dict: Vec<(String, f32)>,
}

impl<D: Clone, F: Clone> Force<D, F> {
    pub fn update(&mut self, graph: &mut ForceGraph<D>, dt: f32) {
        (self.internal_update)(&mut self.internal_data, dt, self.dict, &mut graph);
    }
}

#[derive(Clone)]
pub struct FruchtermanReingold;

impl FruchtermanReingold {
    pub fn new<D: Clone>(scale: f32) -> Force<D, Self> {
        let dict = vec![
            ("Scale".to_string(), scale),
            ("Cooloff Factor".to_string(), 0.975),
        ];

        fn internal_update<D: Clone>(
            _data: &mut FruchtermanReingold,
            dt: f32,
            dict: Vec<(String, f32)>,
            graph: &mut ForceGraph<D>,
        ) {
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
                        / dict[0].1)
                        * ((node_two.location - node_one.location)
                            / node_one.location.distance(node_two.location));
                }

                for neighbor_index in graph_clone.neighbors(node_index) {
                    let node_one = &graph_clone[node_index];
                    let node_two = &graph_clone[neighbor_index];

                    final_force += -((dict[0].1 * dict[0].1)
                        / node_one.location.distance(node_two.location))
                        * ((node_two.location - node_one.location)
                            / node_one.location.distance(node_two.location));
                }

                let node = &mut graph[node_index];

                let acceleration = final_force / node.mass;
                node.velocity += acceleration * dt;
                node.velocity *= dict[1].1;

                node.location += node.velocity * dt;
            }
        }

        Force {
            internal_update,
            internal_data: FruchtermanReingold,
            dict,
        }
    }
}
