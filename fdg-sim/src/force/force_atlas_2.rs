use glam::Vec3;

use crate::{force::Value, ForceGraph};

use super::Force;

pub fn force_atlas_2<N: Clone, E: Clone>(scale: f32, cooloff_factor: f32) -> Force<N, E> {
    fn update<N: Clone, E: Clone>(
        dict: Vec<(&'static str, Value)>,
        graph: &mut ForceGraph<N, E>,
        dt: f32,
    ) {
        let graph_clone = graph.clone();

        let scale = dict[0].1.number();
        let cooloff_factor = dict[1].1.number();

        for node_index in graph_clone.node_indices() {
            if graph_clone[node_index].locked {
                continue;
            }

            let mut final_force = Vec3::ZERO;

            let node_one = &graph_clone[node_index];

            for other_node_index in graph_clone.node_indices() {
                if other_node_index == node_index {
                    continue;
                }

                let node_two = &graph_clone[other_node_index];

                let unit_vector = (node_two.location - node_one.location)
                    / node_one.location.distance(node_two.location);

                final_force += node_one.location.distance(node_two.location) * unit_vector;
            }

            for neighbor_index in graph_clone.neighbors(node_index) {
                let node_two = &graph_clone[neighbor_index];

                let unit_vector = (node_two.location - node_one.location)
                    / node_one.location.distance(node_two.location);

                final_force += (scale
                    * (((graph_clone.neighbors(node_index).count() + 1)
                        * (graph_clone.neighbors(neighbor_index).count() + 1))
                        as f32
                        / node_one.location.distance(node_two.location)))
                    * unit_vector;
            }

            let node = &mut graph[node_index];

            node.velocity += final_force * dt;
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
        name: "ForceAtlas2",
        continuous: true,
        info: Some("A force directed graph drawing algorithm based on ForceAtlas2 in Gephi."),
        update,
    }
}
