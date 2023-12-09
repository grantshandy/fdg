use std::{collections::HashMap, ops::AddAssign};

use nalgebra::{Point, SVector};
use petgraph::{stable_graph::NodeIndex, visit::EdgeRef};

use crate::{Field, Force, ForceGraph};

#[derive(Clone)]
pub struct FruchtermanReingold<T: Field, const D: usize> {
    pub dt: T,
    pub cooloff_factor: T,
    pub scale: T,
    pub velocities: HashMap<NodeIndex, SVector<T, D>>,
}

impl<T: Field, const D: usize> Default for FruchtermanReingold<T, D> {
    fn default() -> Self {
        Self {
            dt: T::from(0.035).unwrap(),
            cooloff_factor: T::from(0.975).unwrap(),
            scale: T::from(45.0).unwrap(),
            velocities: HashMap::new(),
        }
    }
}

impl<T: Field, const D: usize, N, E> Force<T, D, N, E> for FruchtermanReingold<T, D> {
    fn apply(&mut self, graph: &mut ForceGraph<T, D, N, E>) {
        let start_positions: HashMap<NodeIndex, Point<T, D>> = graph
            .node_indices()
            .map(|idx| (idx, graph.node_weight(idx).unwrap().1))
            .collect();

        for (idx, _pos) in &start_positions {
            let mut velocity: SVector<T, D> = *self
                .velocities
                .get(idx)
                .unwrap_or(&SVector::<T, D>::zeros());

            velocity.add_assign(
                (get_attraction(idx, &graph, &start_positions, self.scale)
                    + get_repulsion(idx, &graph, &start_positions, self.scale))
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
    pub velocities: HashMap<NodeIndex, SVector<T, D>>,
}

impl<T: Field, const D: usize> Default for FruchtermanReingoldWeighted<T, D> {
    fn default() -> Self {
        Self {
            dt: T::from(0.035).unwrap(),
            cooloff_factor: T::from(0.975).unwrap(),
            scale: T::from(45.0).unwrap(),
            velocities: HashMap::new(),
        }
    }
}

impl<T: Field, const D: usize, N> Force<T, D, N, T> for FruchtermanReingoldWeighted<T, D> {
    fn apply(&mut self, graph: &mut ForceGraph<T, D, N, T>) {
        let start_positions: HashMap<NodeIndex, Point<T, D>> = graph
            .node_indices()
            .map(|idx| (idx, graph.node_weight(idx).unwrap().1))
            .collect();

        for (idx, _pos) in &start_positions {
            let mut velocity: SVector<T, D> = *self
                .velocities
                .get(idx)
                .unwrap_or(&SVector::<T, D>::zeros());

            velocity.add_assign(
                (get_attraction_weighted(idx, &graph, &start_positions, self.scale)
                    + get_repulsion(idx, &graph, &start_positions, self.scale))
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

fn get_repulsion<T: Field, const D: usize, N, E>(
    idx: &NodeIndex,
    graph: &ForceGraph<T, D, N, E>,
    start_positions: &HashMap<NodeIndex, Point<T, D>>,
    scale: T,
) -> SVector<T, D> {
    let pos = start_positions.get(idx).unwrap();

    graph
        .node_indices()
        .filter(|other_idx| other_idx != idx)
        .map(|other_idx| start_positions.get(&other_idx).unwrap())
        .map(|other_pos| {
            (other_pos - pos).normalize()
                * -(scale.powi(2) / nalgebra::distance_squared(other_pos, pos))
        })
        .sum()
}

fn get_attraction<T: Field, const D: usize, N, E>(
    idx: &NodeIndex,
    graph: &ForceGraph<T, D, N, E>,
    start_positions: &HashMap<NodeIndex, Point<T, D>>,
    scale: T,
) -> SVector<T, D> {
    let pos = start_positions.get(idx).unwrap();

    graph
        .neighbors_undirected(*idx)
        .filter(|neighbor_idx| neighbor_idx != idx)
        .map(|neighbor_idx| start_positions.get(&neighbor_idx).unwrap())
        .map(|neighbor_pos| {
            (neighbor_pos - pos).normalize()
                * (nalgebra::distance_squared(neighbor_pos, pos).mul(scale.recip()))
        })
        .sum()
}

fn get_attraction_weighted<T: Field, const D: usize, N>(
    idx: &NodeIndex,
    graph: &ForceGraph<T, D, N, T>,
    start_positions: &HashMap<NodeIndex, Point<T, D>>,
    scale: T,
) -> SVector<T, D> {
    let pos = start_positions.get(idx).unwrap();

    graph
        .edges(*idx)
        .map(|edge| {
            let neighbor = if edge.source() == *idx {
                edge.target()
            } else {
                edge.source()
            };

            (start_positions.get(&neighbor).unwrap(), edge.weight())
        })
        .map(|(neighbor_pos, weight)| {
            (neighbor_pos - pos).normalize()
                * (nalgebra::distance_squared(neighbor_pos, pos).mul(scale.recip()))
                * *weight
        })
        .sum()
}
