use glam::Vec3;

use crate::{
    force::{DictionaryEntry, ForceValue},
    ForceGraph,
};

use super::Force;

/// A force directed graph drawing algorithm based on Fruchterman-Reingold (1991).
pub fn fruchterman_reingold<N: Clone, E: Clone>(scale: f32, cooloff_factor: f32) -> Force<N, E> {
    fn update<N: Clone, E: Clone>(
        dict: Vec<DictionaryEntry>,
        graph: &mut ForceGraph<N, E>,
        dt: f32,
    ) {
        let graph_clone = graph.clone();

        let scale = dict[0].value.number();
        let cooloff_factor = dict[1].value.number();

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

            node.velocity += final_force * dt;
            node.velocity *= cooloff_factor;
            node.location += node.velocity * dt;
        }
    }

    let dict = vec![
        DictionaryEntry::new("Scale", ForceValue::Number(scale, 1.0..=200.0)),
        DictionaryEntry::new(
            "Cooloff Factor",
            ForceValue::Number(cooloff_factor, 0.0..=1.0),
        ),
    ];

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
