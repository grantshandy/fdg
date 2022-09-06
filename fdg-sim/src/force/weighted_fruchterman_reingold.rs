use glam::Vec3;
use hashlink::LinkedHashMap;
use petgraph::visit::EdgeRef;

use crate::{force::Value, ForceGraph};

use super::Force;

/// A force directed graph drawing algorithm based on Fruchterman-Reingold (1991). but also multiplying attractions by edge weights
pub fn weighted_fruchterman_reingold<N: Clone, E: Clone + Into<f32>>(
    scale: f32,
    cooloff_factor: f32,
) -> Force<N, E> {
    fn update<N: Clone, E: Clone + Into<f32>>(
        dict: &LinkedHashMap<String, Value>,
        graph: &mut ForceGraph<N, E>,
        dt: f32,
    ) {
        let graph_clone = graph.clone();

        let scale = dict.get("Scale").unwrap().number().unwrap();
        let cooloff_factor = dict.get("Cooloff Factor").unwrap().number().unwrap();

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

                final_force += -((scale * scale) / node_one.location.distance(node_two.location))
                    * ((node_two.location - node_one.location)
                        / node_one.location.distance(node_two.location))
            }

            for edge in graph_clone.edges(node_index) {
                let neighbor_index = if edge.source() == node_index {
                    edge.target()
                } else {
                    edge.source()
                };
                let node_two = &graph_clone[neighbor_index];

                let weight: f32 = edge.weight().clone().into();

                final_force += (node_one.location.distance_squared(node_two.location) / scale)
                    * ((node_two.location - node_one.location)
                        / node_one.location.distance(node_two.location))
                    * weight
            }

            let node = &mut graph[node_index];

            node.velocity += final_force * dt;
            node.velocity *= cooloff_factor;
            node.location += node.velocity * dt;
        }
    }

    let mut dict = LinkedHashMap::new();
    dict.insert("Scale".to_string(), Value::Number(scale, 1.0..=200.0));
    dict.insert(
        "Cooloff Factor".to_string(),
        Value::Number(cooloff_factor, 0.0..=1.0),
    );

    Force {
        dict: dict.clone(),
        dict_default: dict,
        name: "Fruchterman-Reingold (1991)",
        continuous: true,
        info: Some(
            "A force directed graph drawing algorithm based on Fruchterman-Reingold (1991).",
        ),
        update,
    }
}
