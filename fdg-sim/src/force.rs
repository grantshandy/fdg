use super::Node;
use glam::Vec3;

#[derive(Clone)]
pub struct SimulationForces<D> {
    general_force_charge: f32,
    spring_constant: f32,
    ideal_spring_length: f32,
    general_callback: fn(&Self, &Node<D>, &Node<D>) -> Vec3,
    neighbor_callback: fn(&Self, &Node<D>, &Node<D>) -> Vec3,
}

impl<D> SimulationForces<D> {
    pub fn apply_general(&self, node_one: &Node<D>, node_two: &Node<D>) -> Vec3 {
        (self.general_callback)(&self, node_one, node_two)
    }

    pub fn apply_neighbor(&self, node_one: &Node<D>, node_two: &Node<D>) -> Vec3 {
        (self.neighbor_callback)(&self, node_one, node_two)
    }
}

/// The default implementation of [`SimulationForces`] uses hooke and coulomb's law.
impl<D> Default for SimulationForces<D> {
    fn default() -> Self {
        fn hooke<D>(force: &SimulationForces<D>, node_one: &Node<D>, node_two: &Node<D>) -> Vec3 {
            let distance = node_one.location.distance(node_two.location);
            let displacement = node_one.location - node_two.location;

            //computes angle between the two nodes in question
            let angle = (displacement.y).atan2(displacement.x);

            //calculate force according to hooke's equation
            let force = (force.spring_constant * 10.0) * -(distance - force.ideal_spring_length);
            //calculate force vector
            Vec3::new(force * angle.cos(), force * angle.sin(), 0.0)
        }

        fn coulomb<D>(force: &SimulationForces<D>, node_one: &Node<D>, node_two: &Node<D>) -> Vec3 {
            //there is probably a better way to do this without using angles -- note for later
            //calculates distance (r^2 in coulomb's equation) to save a few cpu cycles
            let distance_squared = node_one.location.distance_squared(node_two.location);
            let displacement = node_one.location - node_two.location;

            //computes angle between the two nodes in question
            let angle = (displacement.y).atan2(displacement.x);

            //calculate force according to coulomb's equation
            let force = (-force.general_force_charge * 10.0) * node_one.mass * node_two.mass
                / distance_squared;

            //calculate force vector
            Vec3::new(force * angle.cos(), force * angle.sin(), 0.0)
        }

        Self {
            general_force_charge: 100.0,
            ideal_spring_length: 100.0,
            spring_constant: 1.0,
            general_callback: coulomb,
            neighbor_callback: hooke,
        }
    }
}
