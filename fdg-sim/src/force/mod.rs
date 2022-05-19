use crate::ForceGraph;
use std::ops::RangeInclusive;
use dyn_clone::DynClone;

mod fruchterman_reingold;

pub use fruchterman_reingold::FruchtermanReingold;

#[derive(Clone, Debug)]
pub enum Value {
    Number(f32, RangeInclusive<f32>),
    Bool(bool),
}

/// A trait that contains all the methods that you need to create a force on the simulation.
pub trait Force<D>: DynClone {
    /// Move the graph in any way you need.
    fn update(&self, graph: &mut ForceGraph<D>, dt: f32);
    /// Retrieve a mutable version of your internal dictionary that cooresponds to reused variables.
    fn dict_mut(&mut self) -> &mut [(&'static str, Value)];
    /// Retrieve your internal dictionary that cooresponds to reused variables.
    fn dict(&self) -> &[(&'static str, Value)];
    /// Reset your internal dictionary to the original settings.
    fn reset(&mut self);
    /// Retrieve a name for your force
    fn name(&self) -> &'static str;
}

dyn_clone::clone_trait_object!(<D> Force<D> where D: Clone);

