use std::sync::Arc;

use super::Node;
use glam::Vec3;

#[derive(Clone)]
pub struct Force<D: PartialEq + Clone + 'static> {
    pub name: String,
    pub force_charge: f32,
    pub callback: Arc<dyn Fn(&Self, &Node<D>, &Node<D>) -> Vec3>,
}

impl<D: PartialEq + Clone> Force<D> {
    pub fn apply(&self, node_one: &Node<D>, node_two: &Node<D>) -> Vec3 {
        (self.callback)(&self, node_one, node_two)
    }

    pub fn coulomb() -> Self {
        fn callback<D: Clone + PartialEq>(
            force: &Force<D>,
            node_one: &Node<D>,
            node_two: &Node<D>,
        ) -> Vec3 {
            //there is probably a better way to do this without using angles -- note for later
            //calculates distance (r^2 in coulomb's equation) to save a few cpu cycles
            let distance_squared = node_one.location.distance_squared(node_two.location);
            let displacement = node_one.location - node_two.location;

            //computes angle between the two nodes in question
            let angle = (displacement.y).atan2(displacement.x);

            //calculate force according to coulomb's equation
            let force =
                (-force.force_charge * 10.0) * node_one.mass * node_two.mass / distance_squared;

            //calculate force vector
            Vec3::new(force * angle.cos(), force * angle.sin(), 0.0)
        }

        Self {
            name: "Coulomb".to_string(),
            force_charge: -10.0,
            callback: Arc::new(callback),
        }
    }

    pub fn hooke() -> Self {
        fn callback<D: Clone + PartialEq>(
            _force: &Force<D>,
            node_one: &Node<D>,
            node_two: &Node<D>,
        ) -> Vec3 {
            //there is probably a better way to do this without using angles -- note for later
            //calculates distance (r^2 in coulomb's equation) to save a few cpu cycles
            let distance = node_one.location.distance(node_two.location);
            let displacement = node_one.location - node_two.location;

            //computes angle between the two nodes in question
            let angle = (displacement.y).atan2(displacement.x);

            //calculate force according to coulomb's equation
            let force = 10.0 * -(distance - 100.0);

            //calculate force vector
            Vec3::new(force * angle.cos(), force * angle.sin(), 0.0)
        }

        Self {
            name: "Hooke".to_string(),
            force_charge: -10.0,
            callback: Arc::new(callback),
        }
    }
}

#[derive(Clone)]
pub struct SimulationForces {
    
}