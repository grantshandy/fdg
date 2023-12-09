use std::ops::AddAssign;

use nalgebra::{Point, SVector};
use petgraph::stable_graph::NodeIndex;
use rustc_hash::FxHashMap;

use crate::{Field, Force, ForceGraph, Node};

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

#[derive(Clone)]
pub struct FruchtermanReingold<T: Field, const D: usize> {
    pub dt: T,
    pub cooloff_factor: T,
    pub scale: T,
    pub velocities: FxHashMap<NodeIndex, SVector<T, D>>,
}

impl<T: Field, const D: usize> Default for FruchtermanReingold<T, D> {
    fn default() -> Self {
        Self {
            dt: T::from(0.035).unwrap(),
            cooloff_factor: T::from(0.975).unwrap(),
            scale: T::from(45.0).unwrap(),
            velocities: FxHashMap::default(),
        }
    }
}

impl<T: Field, const D: usize, N, E> Force<T, D, N, E> for FruchtermanReingold<T, D> {
    fn apply(&mut self, graph: &mut ForceGraph<T, D, N, E>) {
        let start_positions: FxHashMap<NodeIndex, Point<T, D>> = graph
            .node_indices()
            .map(|idx| (idx, graph.node_weight(idx).unwrap().1))
            .collect();

        for idx in start_positions.keys() {
            let mut velocity: SVector<T, D> = *self
                .velocities
                .get(idx)
                .unwrap_or(&SVector::<T, D>::zeros());

            velocity.add_assign(
                (fruchterman_reingold::get_attraction(*idx, graph, &start_positions, self.scale)
                    + fruchterman_reingold::get_repulsion(
                        *idx,
                        graph,
                        &start_positions,
                        self.scale,
                    ))
                    * self.dt,
            );
            velocity.scale_mut(self.cooloff_factor);

            self.velocities.insert(*idx, velocity);

            graph
                .node_weight_mut(*idx)
                .unwrap()
                .1
                .add_assign(velocity * self.dt);
        }
    }
}

#[derive(Clone)]
pub struct FruchtermanReingoldWeighted<T: Field, const D: usize> {
    pub dt: T,
    pub cooloff_factor: T,
    pub scale: T,
    pub velocities: FxHashMap<NodeIndex, SVector<T, D>>,
}

impl<T: Field, const D: usize> Default for FruchtermanReingoldWeighted<T, D> {
    fn default() -> Self {
        Self {
            dt: T::from(0.035).unwrap(),
            cooloff_factor: T::from(0.975).unwrap(),
            scale: T::from(45.0).unwrap(),
            velocities: FxHashMap::default(),
        }
    }
}

impl<T: Field, const D: usize, N> Force<T, D, N, T> for FruchtermanReingoldWeighted<T, D> {
    fn apply(&mut self, graph: &mut ForceGraph<T, D, N, T>) {
        let start_positions: FxHashMap<NodeIndex, Point<T, D>> = graph
            .node_indices()
            .map(|idx| (idx, graph.node_weight(idx).unwrap().1))
            .collect();

        for idx in start_positions.keys() {
            let mut velocity: SVector<T, D> = *self
                .velocities
                .get(idx)
                .unwrap_or(&SVector::<T, D>::zeros());

            velocity.add_assign(
                (fruchterman_reingold::get_attraction_weighted(
                    *idx,
                    graph,
                    &start_positions,
                    self.scale,
                ) + fruchterman_reingold::get_repulsion(
                    *idx,
                    graph,
                    &start_positions,
                    self.scale,
                )) * self.dt,
            );
            velocity.scale_mut(self.cooloff_factor);

            self.velocities.insert(*idx, velocity);

            graph
                .node_weight_mut(*idx)
                .unwrap()
                .1
                .add_assign(velocity * self.dt);
        }
    }
}

mod fruchterman_reingold {
    use nalgebra::{Point, SVector};
    use petgraph::{stable_graph::NodeIndex, visit::EdgeRef};
    use rustc_hash::FxHashMap;

    use crate::{Field, ForceGraph};

    pub fn get_repulsion<T: Field, const D: usize, N, E>(
        idx: NodeIndex,
        graph: &ForceGraph<T, D, N, E>,
        start_positions: &FxHashMap<NodeIndex, Point<T, D>>,
        scale: T,
    ) -> SVector<T, D> {
        let pos = start_positions.get(&idx).unwrap();

        graph
            .node_indices()
            .filter(|other_idx| other_idx != &idx)
            .map(|other_idx| start_positions.get(&other_idx).unwrap())
            .map(|other_pos| {
                (other_pos - pos).normalize()
                    * -(scale.simd_powi(2) / nalgebra::distance_squared(other_pos, pos))
            })
            .sum()
    }

    pub fn get_attraction<T: Field, const D: usize, N, E>(
        idx: NodeIndex,
        graph: &ForceGraph<T, D, N, E>,
        start_positions: &FxHashMap<NodeIndex, Point<T, D>>,
        scale: T,
    ) -> SVector<T, D> {
        let pos = start_positions.get(&idx).unwrap();

        graph
            .neighbors_undirected(idx)
            .filter(|neighbor_idx| neighbor_idx != &idx)
            .map(|neighbor_idx| start_positions.get(&neighbor_idx).unwrap())
            .map(|neighbor_pos| {
                (neighbor_pos - pos).normalize()
                    * (nalgebra::distance_squared(neighbor_pos, pos) / scale)
            })
            .sum()
    }

    pub fn get_attraction_weighted<T: Field, const D: usize, N>(
        idx: NodeIndex,
        graph: &ForceGraph<T, D, N, T>,
        start_positions: &FxHashMap<NodeIndex, Point<T, D>>,
        scale: T,
    ) -> SVector<T, D> {
        let pos = start_positions.get(&idx).unwrap();

        graph
            .edges(idx)
            .map(|edge| {
                let neighbor = if edge.source() == idx {
                    edge.target()
                } else {
                    edge.source()
                };

                (start_positions.get(&neighbor).unwrap(), edge.weight())
            })
            .map(|(neighbor_pos, weight)| {
                (neighbor_pos - pos).normalize()
                    * (nalgebra::distance_squared(neighbor_pos, pos) / scale)
                    * *weight
            })
            .sum()
    }
}
