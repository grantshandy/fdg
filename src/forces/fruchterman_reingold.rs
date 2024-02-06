//! Implementations of the Fruchterman-Reingold (1991) force-directed graph drawing layout.

use std::{collections::HashMap, hash::BuildHasherDefault, ops::AddAssign};

use nalgebra::{Point, SVector};
use petgraph::{stable_graph::NodeIndex, visit::EdgeRef};

use crate::{Field, Force, ForceGraph};

type HashFn = BuildHasherDefault<rustc_hash::FxHasher>;

/// General configuration parameters for Fruchterman-Reingold (1991) forces.
#[derive(Debug, Clone)]
pub struct FruchtermanReingoldConfiguration<T: Field> {
    pub dt: T,
    pub cooloff_factor: T,
    pub scale: T,
}

use FruchtermanReingoldConfiguration as Config;

impl<T: Field> Default for Config<T> {
    fn default() -> Self {
        Self {
            dt: T::from(0.035).unwrap(),
            cooloff_factor: T::from(0.975).unwrap(),
            scale: T::from(45.0).unwrap(),
        }
    }
}

/// A basic implementation
#[derive(Default, Debug, Clone)]
pub struct FruchtermanReingold<T: Field, const D: usize> {
    pub conf: Config<T>,
    pub velocities: HashMap<NodeIndex, SVector<T, D>, HashFn>,
}

impl<T: Field, const D: usize, N, E> Force<T, D, N, E> for FruchtermanReingold<T, D> {
    fn apply(&mut self, graph: &mut ForceGraph<T, D, N, E>) {
        let start_positions: HashMap<NodeIndex, Point<T, D>, HashFn> = graph
            .node_indices()
            .map(|idx| (idx, graph.node_weight(idx).unwrap().1))
            .collect();

        for idx in start_positions.keys() {
            let mut velocity: SVector<T, D> = *self
                .velocities
                .get(idx)
                .unwrap_or(&SVector::<T, D>::zeros());

            let pos = start_positions.get(idx).unwrap();

            let attraction = graph
                .neighbors_undirected(*idx)
                .filter(|neighbor_idx| neighbor_idx != idx)
                .map(|neighbor_idx| start_positions.get(&neighbor_idx).unwrap())
                .map(|neighbor_pos| {
                    (neighbor_pos - pos).normalize()
                        * (nalgebra::distance_squared(neighbor_pos, pos) / self.conf.scale)
                })
                .sum::<SVector<T, D>>();
            let repulsion = graph
                .node_indices()
                .filter(|other_idx| other_idx != idx)
                .map(|other_idx| start_positions.get(&other_idx).unwrap())
                .map(|other_pos| {
                    (other_pos - pos).normalize()
                        * -(self.conf.scale.simd_powi(2)
                            / nalgebra::distance_squared(other_pos, pos))
                })
                .sum::<SVector<T, D>>();

            velocity.add_assign((attraction + repulsion) * self.conf.dt);
            velocity.scale_mut(self.conf.cooloff_factor);

            self.velocities.insert(*idx, velocity);

            graph
                .node_weight_mut(*idx)
                .unwrap()
                .1
                .add_assign(velocity * self.conf.dt);
        }
    }
}

/// A simple implementation of Fruchterman-Reingold (1991) weighted by edge values.
///
/// Attraction between nodes is multiplied by the value of their linking edge, the edge weight must be the same type as the vector coords.
#[derive(Default, Debug, Clone)]
pub struct FruchtermanReingoldWeighted<T: Field, const D: usize> {
    pub conf: Config<T>,
    pub velocities: HashMap<NodeIndex, SVector<T, D>, HashFn>,
}

impl<T: Field, const D: usize, N> Force<T, D, N, T> for FruchtermanReingoldWeighted<T, D> {
    fn apply(&mut self, graph: &mut ForceGraph<T, D, N, T>) {
        let start_positions: HashMap<NodeIndex, Point<T, D>, HashFn> = graph
            .node_indices()
            .map(|idx| (idx, graph.node_weight(idx).unwrap().1))
            .collect();

        for idx in start_positions.keys() {
            let mut velocity: SVector<T, D> = *self
                .velocities
                .get(idx)
                .unwrap_or(&SVector::<T, D>::zeros());

            let pos = start_positions.get(idx).unwrap();

            let attraction = graph
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
                        * (nalgebra::distance_squared(neighbor_pos, pos) / self.conf.scale)
                        * *weight
                })
                .sum::<SVector<T, D>>();
            let repulsion = graph
                .node_indices()
                .filter(|other_idx| other_idx != idx)
                .map(|other_idx| start_positions.get(&other_idx).unwrap())
                .map(|other_pos| {
                    (other_pos - pos).normalize()
                        * -(self.conf.scale.simd_powi(2)
                            / nalgebra::distance_squared(other_pos, pos))
                })
                .sum::<SVector<T, D>>();

            velocity.add_assign((attraction + repulsion) * self.conf.dt);
            velocity.scale_mut(self.conf.cooloff_factor);

            self.velocities.insert(*idx, velocity);

            graph
                .node_weight_mut(*idx)
                .unwrap()
                .1
                .add_assign(velocity * self.conf.dt);
        }
    }
}

#[cfg(feature = "rayon")]
#[doc(inline)]
pub use parallel::*;

#[cfg(feature = "rayon")]
mod parallel {
    use super::{
        AddAssign, Config, Field, Force, ForceGraph, HashFn, HashMap, NodeIndex, Point,
        SVector,
    };
    use rayon::prelude::*;

    /// An alternative implementation of Fruchterman-Reingold (1991) that computes the forces in parallel with [`rayon`].
    #[derive(Default, Debug, Clone)]
    pub struct FruchtermanReingoldParallel<T: Field, const D: usize> {
        pub conf: Config<T>,
        pub velocities: HashMap<NodeIndex, SVector<T, D>, HashFn>,
    }

    impl<T: Field, const D: usize, N: Send + Sync, E: Send + Sync> Force<T, D, N, E>
        for FruchtermanReingoldParallel<T, D>
    {
        fn apply(&mut self, graph: &mut ForceGraph<T, D, N, E>) {
            let start_positions: HashMap<NodeIndex, Point<T, D>, HashFn> = graph
                .node_indices()
                .par_bridge()
                .map(|idx| (idx, graph.node_weight(idx).unwrap().1))
                .collect();

            let changes: Vec<_> = start_positions
                .keys()
                .par_bridge()
                .map(|idx| {
                    let mut velocity: SVector<T, D> =
                        *self.velocities.get(idx).unwrap_or(&SVector::zeros());

                    let pos = start_positions.get(idx).unwrap();

                    let attraction = graph
                        .neighbors_undirected(*idx)
                        .par_bridge()
                        .filter(|neighbor_idx| neighbor_idx != idx)
                        .map(|neighbor_idx| start_positions.get(&neighbor_idx).unwrap())
                        .map(|neighbor_pos| {
                            (neighbor_pos - pos).normalize()
                                * (nalgebra::distance_squared(neighbor_pos, pos) / self.conf.scale)
                        })
                        .sum::<SVector<T, D>>();
                    let repulsion = graph
                        .node_indices()
                        .par_bridge()
                        .filter(|other_idx| other_idx != idx)
                        .map(|other_idx| start_positions.get(&other_idx).unwrap())
                        .map(|other_pos| {
                            (other_pos - pos).normalize()
                                * -(self.conf.scale.simd_powi(2)
                                    / nalgebra::distance_squared(other_pos, pos))
                        })
                        .sum::<SVector<T, D>>();

                    velocity.add_assign((attraction + repulsion) * self.conf.dt);
                    velocity.scale_mut(self.conf.cooloff_factor);

                    (idx, velocity)
                })
                .collect();

            for (idx, velocity) in changes {
                self.velocities.insert(*idx, velocity);
                graph
                    .node_weight_mut(*idx)
                    .unwrap()
                    .1
                    .add_assign(velocity * self.conf.dt);
            }
        }
    }
}
