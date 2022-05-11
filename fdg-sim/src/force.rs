use glam::Vec3;

use crate::Node;

/// Forces that dictate how your nodes move.
#[derive(Clone)]
pub struct Forces<D> {
    general_force: fn(&Vec<f32>, &Node<D>, &Node<D>) -> Vec3,
    neighbor_force: fn(&Vec<f32>, &Node<D>, &Node<D>) -> Vec3,
    dict: Vec<f32>,
}

impl<D> Forces<D> {
    pub fn apply_general_force(&self, node_one: &Node<D>, node_two: &Node<D>) -> Vec3 {
        (self.general_force)(&self.dict, node_one, node_two)
    }

    pub fn apply_neighbor_force(&self, node_one: &Node<D>, node_two: &Node<D>) -> Vec3 {
        (self.neighbor_force)(&self.dict, node_one, node_two)
    }

    pub fn dict(&self) -> &Vec<f32> {
        &self.dict
    }
}

/// The default implementation of [`Forces`] uses Fruchterman & Reingold (1991).
impl<D> Default for Forces<D> {
    fn default() -> Self {
        Forces::fruchterman_reingold(45.0)
    }
}

impl<D> Forces<D> {
    pub fn fruchterman_reingold(ideal_distance: f32) -> Self {
        let dict = vec![ideal_distance];

        fn general_force<D>(dict: &Vec<f32>, node_one: &Node<D>, node_two: &Node<D>) -> Vec3 {
            -((dict[0] * dict[0]) / node_one.location.distance(node_two.location))
                * ((node_two.location - node_one.location)
                    / node_one.location.distance(node_two.location))
        }

        fn neighbor_force<D>(dict: &Vec<f32>, node_one: &Node<D>, node_two: &Node<D>) -> Vec3 {
            (node_one.location.distance_squared(node_two.location) / dict[0])
                * ((node_two.location - node_one.location)
                    / node_one.location.distance(node_two.location))
        }

        Self {
            general_force,
            neighbor_force,
            dict,
        }
    }
}