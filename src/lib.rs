#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(clippy::pedantic)]

use nalgebra::{ClosedAdd, ClosedDiv, ClosedMul, ClosedSub, Const, OVector, Scalar, SimdRealField, Point};
use num_traits::Float;
use petgraph::{
    stable_graph::{DefaultIx, StableGraph},
    Directed,
};
use rand::distributions::Distribution;

mod forces;

#[doc(inline)]
pub use forces::*;

pub use nalgebra;
pub use petgraph;
pub use rand::distributions;

pub type ForceGraph<T, const D: usize, N, E, Ty = Directed, Ix = DefaultIx> =
    StableGraph<Node<T, D, N>, E, Ty, Ix>;

pub struct Node<T: Field, const D: usize, N>(pub N, pub Point<T, D>);

impl<T: Field, const D: usize, N> From<(N, Point<T, D>)> for Node<T, D, N> {
    fn from((data, pos): (N, Point<T, D>)) -> Self {
        Self(data, pos)
    }
}

/// T: Coordinate type (f32/f64)
/// D: Number of dimensions
/// N: Node weight type (any)
/// E: Edge weight type (any)
pub trait Force<T: Field, const D: usize, N, E> {
    /// Apply a force to a given graph.
    fn apply(&mut self, graph: &mut ForceGraph<T, D, N, E>);
}

/// Create a `ForceGraph` from any graph and randomize the node positions within a given distribution.
pub fn init_force_graph<T: Field, const D: usize, N: Clone, E: Clone>(
    input: impl Into<StableGraph<N, E>>,
    distribution: impl Distribution<T>,
) -> ForceGraph<T, D, N, E> {
    let mut rng = rand::thread_rng();

    input.into().map(
        |_, node| {
            (
                node.clone(),
                Point::from(OVector::from_distribution_generic(
                    Const::<D>,
                    Const::<1>,
                    &distribution,
                    &mut rng,
                )),
            )
                .into()
        },
        |_, edge| edge.clone(),
    )
}

/// A composite trait that covers both f32 and f64.
pub trait Field:
    SimdRealField + Float + Scalar + ClosedMul + ClosedDiv + ClosedAdd + ClosedSub + Send + Sync
{
}
impl<T> Field for T where
    T: SimdRealField + Float + Scalar + ClosedMul + ClosedDiv + ClosedAdd + ClosedSub + Send + Sync
{
}
