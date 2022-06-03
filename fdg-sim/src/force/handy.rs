use glam::Vec3;

use crate::{force::Value, ForceGraph};

use super::Force;

pub fn handy<D: Clone>(scale: f32, cooloff_factor: f32, centering: bool) -> Force<D> {
    fn update<D: Clone>(dict: Vec<(&'static str, Value)>, graph: &mut ForceGraph<D>, dt: f32) {
        let graph_clone = graph.clone();

        let scale = dict[0].1.number();
        let cooloff_factor = dict[1].1.number();
        let centering = dict[2].1.bool();

        let mut vec_sum = Vec3::ZERO;

        for node_index in graph_clone.node_indices() {
            if centering {
                vec_sum += graph_clone[node_index].location
            }

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

                final_force += -((scale * scale) / node_one.location.distance(node_two.location))
                    * unit_vector;
            }

            for neighbor_index in graph_clone.neighbors(node_index) {
                let node_two = &graph_clone[neighbor_index];

                let unit_vector = (node_two.location - node_one.location)
                    / node_one.location.distance(node_two.location);

                final_force +=
                    (node_one.location.distance_squared(node_two.location) / scale) * unit_vector;
            }

            let node = &mut graph[node_index];

            node.velocity += final_force * dt;
            node.velocity *= cooloff_factor;
            node.location += node.velocity * dt;
        }

        if centering {
            let avg_vec = vec_sum / graph_clone.node_count() as f32;

            for node_index in graph_clone.node_indices() {
                let node = &mut graph[node_index];

                node.location = node.location - avg_vec;
            }
        }
    }

    let dict = vec![
        ("Scale", Value::Number(scale, 1.0..=200.0)),
        ("Cooloff Factor", Value::Number(cooloff_factor, 0.0..=1.0)),
        ("Centering", Value::Bool(centering)),
    ];

    Force {
        dict: dict.clone(),
        dict_default: dict,
        name: "Handy",
        continuous: true,
        info: Some("Custom fda"),
        update,
    }
}
