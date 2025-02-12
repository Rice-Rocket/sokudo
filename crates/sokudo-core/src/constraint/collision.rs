use glam::Vec3;

use crate::{collider::{Collider, ColliderBody, ColliderId}, contact::Contact};

use super::Constraint;

pub struct ParticleCollisionConstraint {
    pub particle: ColliderId,
    pub rb: ColliderId,

    pub contact: Contact,
    pub compliance: f32,
}

impl Constraint for ParticleCollisionConstraint {
    #[inline]
    fn bodies(&self) -> Vec<ColliderId> {
        vec![self.particle, self.rb]
    }

    fn c(&self, _bodies: &[&Collider]) -> f32 {
        self.contact.depth
    }

    fn c_gradients(&self, _bodies: &[&Collider]) -> Vec<Vec3> {
        let n = self.contact.normal;
        vec![-n, n]
    }

    fn inverse_masses(&self, bodies: &[&Collider]) -> Vec<f32> {
        let [particle, rb] = *bodies else { return vec![] };

        let ColliderBody::Particle(ref particle_body) = particle.body else {
            return vec![];
        };

        let ColliderBody::Rigid(ref rb_body) = rb.body else {
            return vec![];
        };

        let w1 = if particle.locked { 0.0 } else { particle_body.inverse_mass() };
        let w2 = if rb.locked { 0.0 } else { rb_body.positional_inverse_mass(self.contact.anchor2, self.contact.normal) };

        vec![w1, w2]
    }

    fn anchors(&self) -> Vec<Vec3> {
        vec![self.contact.anchor1, self.contact.anchor2]
    }

    #[inline]
    fn compliance(&self) -> f32 {
        self.compliance
    }
}

pub struct RigidBodyCollisionConstraint {
    pub a: ColliderId,
    pub b: ColliderId,

    pub compliance: f32,
}
