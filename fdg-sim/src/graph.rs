use super::Node;
use petgraph::{graph::NodeIndex, stable_graph::StableGraph, Undirected};

/// A helper type that creates a [`StableGraph`] with our custom [`Node`].
pub type ForceGraph<N, E> = StableGraph<Node<N>, E, Undirected>;

/// Syntactic sugar to make adding [`Node`]s to a [`ForceGraph`] easier.
pub trait ForceGraphHelper<N, E> {
    fn add_force_node(&mut self, name: impl AsRef<str>, data: N) -> NodeIndex;
}

impl<N, E> ForceGraphHelper<N, E> for ForceGraph<N, E> {
    fn add_force_node(&mut self, name: impl AsRef<str>, data: N) -> NodeIndex {
        self.add_node(Node::new(name, data))
    }
}
