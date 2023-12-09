use nalgebra::SVector;

use crate::{Field, Force, ForceGraph, Node};

mod fruchterman_reingold;

#[doc(inline)]
pub use fruchterman_reingold::FruchtermanReingold;

#[derive(Copy, Clone)]
pub struct Scale<T: Field> {
    pub factor: T,
}

impl<T: Field, const D: usize, N, E> Force<T, D, N, E> for Scale<T> {
    fn apply(&mut self, graph: &mut ForceGraph<T, D, N, E>) {
        graph
            .node_weights_mut()
            .for_each(|Node(_, pos)| pos.coords.scale_mut(self.factor));
    }
}

#[derive(Copy, Clone)]
pub struct Translate<T: Field, const D: usize> {
    pub translation: SVector<T, D>,
}

impl<T: Field, const D: usize> Translate<T, D> {
    pub fn new(translation: SVector<T, D>) -> Self {
        Self { translation }
    }
}

impl<T: Field, const D: usize, N, E> Force<T, D, N, E> for Translate<T, D> {
    fn apply(&mut self, graph: &mut ForceGraph<T, D, N, E>) {
        graph
            .node_weights_mut()
            .for_each(|Node(_, pos)| pos.coords += self.translation);
    }
}

#[derive(Default, Copy, Clone)]
pub struct Center;

impl<T: Field, const D: usize, N, E> Force<T, D, N, E> for Center {
    fn apply(&mut self, graph: &mut ForceGraph<T, D, N, E>) {
        let avg: SVector<T, D> = graph
            .node_weights()
            .map(|Node(_, p)| p.coords)
            .sum::<SVector<T, D>>()
            .scale(T::from(graph.node_count()).unwrap().recip());

        graph.node_weights_mut().for_each(|Node(_, p)| *p -= avg);
    }
}
