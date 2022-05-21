use std::sync::Arc;

use wgpu::{Instance, Backends, Adapter, RequestAdapterOptions};

use crate::ForceGraph;

use super::{Force, Value};

#[derive(Clone, Debug)]
pub struct FruchtermanReingoldGpu {
    dict: Vec<(&'static str, Value)>,
    dict_default: Vec<(&'static str, Value)>,
    instance: Arc<Instance>,
    adapter: Arc<Adapter>,
}

impl FruchtermanReingoldGpu {
    pub fn new() -> Option<Self> {
        pollster::block_on(async {
            let dict = Vec::new();
            let instance = Instance::new(Backends::all());
            let adapter = instance.request_adapter(&RequestAdapterOptions::default()).await?;

            Some(Self {
                dict: dict.clone(),
                dict_default: dict,
                instance: Arc::new(instance),
                adapter: Arc::new(adapter),
            })
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
