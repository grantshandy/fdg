use glam::Vec3;
use hashlink::LinkedHashMap;

use crate::{force::Value, ForceGraph};

use super::Force;

/// My own force-directed graph drawing algorithm.
pub fn handy<N: Clone, E: Clone>(
    scale: f32,
    cooloff_factor: f32,
    gravity: bool,
    centering: bool,
) -> Force<N, E> {
    fn update<N: Clone, E: Clone>(
        dict: &LinkedHashMap<String, Value>,
        graph: &mut ForceGraph<N, E>,
        dt: f32,
    ) {
        let graph_clone = graph.clone();

        let repulsive = dict.get("Repulsive").unwrap().bool().unwrap();
        let attractive = dict.get("Attractive").unwrap().bool().unwrap();
        let scale = dict.get("Scale").unwrap().number().unwrap();
        let cooloff_factor = dict.get("Cooloff Factor").unwrap().number().unwrap();
        let gravity_factor = dict.get("Gravity Factor").unwrap().number().unwrap();
        let centering = dict.get("Centering").unwrap().bool().unwrap();
        let gravity = dict.get("Gravity").unwrap().bool().unwrap();

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

            if repulsive {
                for other_node_index in graph_clone.node_indices() {
                    if other_node_index == node_index {
                        continue;
                    }

                    let node_two = &graph_clone[other_node_index];

                    let unit_vector = (node_two.location - node_one.location)
                        / node_one.location.distance(node_two.location);

                    final_force += -((scale * scale)
                        / node_one.location.distance(node_two.location))
                        * unit_vector;
                }
            }

            if attractive {
                for neighbor_index in graph_clone.neighbors(node_index) {
                    let node_two = &graph_clone[neighbor_index];

                    let unit_vector = (node_two.location - node_one.location)
                        / node_one.location.distance(node_two.location);

                    final_force += (node_one.location.distance_squared(node_two.location) / scale)
                        * unit_vector;
                }
            }

            let node = &mut graph[node_index];

            if gravity {
                final_force += -node.location / gravity_factor;
            }

            node.velocity += final_force * dt;
            node.velocity *= cooloff_factor;
            node.location += node.velocity * dt;
        }

        if centering {
            let avg_vec = vec_sum / graph_clone.node_count() as f32;

            for node_index in graph_clone.node_indices() {
                let node = &mut graph[node_index];

                node.location -= avg_vec;
            }
        }
    }

    let mut dict = LinkedHashMap::new();
    dict.insert("Repulsive".to_string(), Value::Bool(true));
    dict.insert("Attractive".to_string(), Value::Bool(true));
    dict.insert("Scale".to_string(), Value::Number(scale, 1.0..=200.0));
    dict.insert(
        "Cooloff Factor".to_string(),
        Value::Number(cooloff_factor, 0.0..=1.0),
    );
    dict.insert("Gravity Factor".to_string(), Value::Number(3.0, 1.0..=10.0));
    dict.insert("Centering".to_string(), Value::Bool(centering));
    dict.insert("Gravity".to_string(), Value::Bool(gravity));

    Force {
        dict: dict.clone(),
        dict_default: dict,
        name: "Handy",
        continuous: true,
        info: Some("Force Directed Algorithm by Grant Handy (2022)"),
        update,
    }
}
