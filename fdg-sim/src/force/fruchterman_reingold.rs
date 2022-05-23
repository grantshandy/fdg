use glam::Vec3;
use pollster::FutureExt;

use crate::ForceGraph;

use wgpu::{Backends, Instance, RequestAdapterOptions};

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

    fn continuous(&self) -> bool {
        true
    }

    fn info(&self) -> Option<&'static str> {
        Some("A force directed graph drawing algorithm based on Fruchterman-Reingold (1991).")
    }
}

#[derive(Clone)]
pub struct FruchtermanReingoldGpu {
    dict: Vec<(&'static str, Value)>,
    dict_default: Vec<(&'static str, Value)>,
}

impl FruchtermanReingoldGpu {
    pub fn new() -> Option<Self> {
        let dict = Vec::new();
        let instance = Instance::new(Backends::all());
        let adapter = instance
            .request_adapter(&RequestAdapterOptions::default())
            .block_on()?;

        let info = adapter.get_info();
        println!("{:#?}", info);

        Some(Self {
            dict: dict.clone(),
            dict_default: dict,
        })
    }
}

impl Default for FruchtermanReingoldGpu {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

impl<D: Clone> Force<D> for FruchtermanReingoldGpu {
    fn update(&self, _graph: &mut ForceGraph<D>, _dt: f32) {
        // self.adapter
        // self.instance
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
        "Fruchterman-Reingold (1991) (GPU)"
    }

    fn continuous(&self) -> bool {
        true
    }

    fn info(&self) -> Option<&'static str> {
        Some("A GPU accelerated force directed graph drawing algorithm based on Fruchterman-Reingold (1991).")
    }
}
