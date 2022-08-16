use super::Node;
use glam::Vec3;
use petgraph::{graph::NodeIndex, stable_graph::StableGraph, Undirected};

/// A helper type that creates a [`StableGraph`] with our custom [`Node`] as the weight.
pub type ForceGraph<N, E> = StableGraph<Node<N>, E, Undirected>;

/// Syntactic sugar to make adding [`Node`]s to a [`ForceGraph`] easier.
pub trait ForceGraphHelper<N, E> {
    /// Add a [`Node`] to the graph with only the name and arbitrary data.
    fn add_force_node(&mut self, name: impl AsRef<str>, data: N) -> NodeIndex;
    /// Add a [`Node`] to the graph with the name, arbitrary data, and a custom color.
    fn add_force_node_with_color(
        &mut self,
        name: impl AsRef<str>,
        data: N,
        color: [u8; 4],
    ) -> NodeIndex;
    #[deprecated(
        since = "0.6.0",
        note = "use of color in nodes will be phased out so it can be handled by the user."
    )]
    /// Add a [`Node`] to the graph with the name, arbitrary data, and a custom location.
    fn add_force_node_with_coords(
        &mut self,
        name: impl AsRef<str>,
        data: N,
        location: Vec3,
    ) -> NodeIndex;
}

impl<N, E> ForceGraphHelper<N, E> for ForceGraph<N, E> {
    fn add_force_node(&mut self, name: impl AsRef<str>, data: N) -> NodeIndex {
        self.add_node(Node::new(name, data))
    }

    #[allow(deprecated)]
    fn add_force_node_with_color(
        &mut self,
        name: impl AsRef<str>,
        data: N,
        color: [u8; 4],
    ) -> NodeIndex {
        self.add_node(Node::new_with_color(name, data, color))
    }

    fn add_force_node_with_coords(
        &mut self,
        name: impl AsRef<str>,
        data: N,
        location: Vec3,
    ) -> NodeIndex {
        self.add_node(Node::new_with_coords(name, data, location))
    }
}
