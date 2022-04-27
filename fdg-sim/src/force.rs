use super::Node;
use glam::Vec3;

#[derive(Clone)]
pub struct SimulationForces<D> {
    general_force: fn(&Vec<f32>, &Node<D>, &Node<D>) -> Vec3,
    neighbor_force: fn(&Vec<f32>, &Node<D>, &Node<D>) -> Vec3,
    dict: Vec<f32>,
}

impl<D> SimulationForces<D> {
    pub fn apply_repulsive(&self, node_one: &Node<D>, node_two: &Node<D>) -> Vec3 {
        (self.general_force)(&self.dict, node_one, node_two)
    }

    pub fn apply_attractive(&self, node_one: &Node<D>, node_two: &Node<D>) -> Vec3 {
        (self.neighbor_force)(&self.dict, node_one, node_two)
    }
}

/// The default implementation of [`SimulationForces`] uses hooke and coulomb's law.
impl<D> Default for SimulationForces<D> {
    fn default() -> Self {
        SimulationForces::fruchterman_reingold(300.0)
    }
}

impl<D> SimulationForces<D> {
    pub fn hooke_coulomb(
        ideal_spring_length: f32,
        spring_constant: f32,
        force_charge: f32,
    ) -> Self {
        let dict = vec![ideal_spring_length, spring_constant, force_charge];

        fn hooke<D>(dict: &Vec<f32>, node_one: &Node<D>, node_two: &Node<D>) -> Vec3 {
            let distance = node_one.location.distance(node_two.location);
            let displacement = node_one.location - node_two.location;

            //computes angle between the two nodes in question
            let angle = (displacement.y).atan2(displacement.x);

            //calculate force according to hooke's equation
            let force = (dict[1] * 10.0) * -(distance - dict[0]);
            //calculate force vector
            Vec3::new(force * angle.cos(), force * angle.sin(), 0.0)
        }

        fn coulomb<D>(dict: &Vec<f32>, node_one: &Node<D>, node_two: &Node<D>) -> Vec3 {
            //there is probably a better way to do this without using angles -- note for later
            //calculates distance (r^2 in coulomb's equation) to save a few cpu cycles
            let distance_squared = node_one.location.distance_squared(node_two.location);
            let displacement = node_one.location - node_two.location;

            //computes angle between the two nodes in question
            let angle = (displacement.y).atan2(displacement.x);

            //calculate force according to coulomb's equation
            let force = (dict[2] * 10.0) * node_one.mass * node_two.mass / distance_squared;

            //calculate force vector
            Vec3::new(force * angle.cos(), force * angle.sin(), 0.0)
        }

        Self {
            general_force: coulomb,
            neighbor_force: hooke,
            dict,
        }
    }

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
