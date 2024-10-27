use glam::Vec3;
use sokudo_io::{read::collider::{ParsedCollider, ParsedColliderBody}, write::{collider::WriteCollider, inspect::InspectElements, transform::WriteTransform}};

use crate::{particle::Particle, rigid_body::RigidBody};

#[derive(Debug)]
pub struct Collider {
    /// This collider's unique ID.
    pub id: u32,
    /// The body of this collider.
    pub body: ColliderBody,
    /// Whether or not this collider is locked. 
    /// This turns off gravity and gives it infinite mass. 
    pub locked: bool,

    pub position: Vec3,
    pub previous_position: Vec3,
    pub velocity: Vec3,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColliderId(u32);

impl ColliderId {
    pub fn new(i: usize) -> ColliderId {
        ColliderId(i as u32)
    }
}

#[derive(Debug)]
pub enum ColliderBody {
    Particle(Particle),
    Rigid(RigidBody),
}

impl Collider {
    /// Simulates the collision between two [`Collider`]s, applying the necessary forces to resolve
    /// the collision if necessary.
    pub fn collide(&mut self, other: &mut Self, inspector: &mut InspectElements) {
    }
}

impl ColliderBody {
    #[inline]
    pub fn mass(&self) -> f32 {
        match self {
            ColliderBody::Particle(particle) => particle.mass,
            ColliderBody::Rigid(rb) => rb.mass,
        }
    }
}

impl From<ParsedCollider> for Collider {
    fn from(value: ParsedCollider) -> Self {
        Collider {
            id: value.id,
            locked: value.locked,
            body: value.body.into(),

            position: value.position,
            previous_position: value.position,
            velocity: value.velocity,
        }
    }
}

impl From<ParsedColliderBody> for ColliderBody {
    fn from(value: ParsedColliderBody) -> Self {
        match value {
            ParsedColliderBody::Particle(particle) => ColliderBody::Particle(particle.into()),
            ParsedColliderBody::RigidBody(rb) => ColliderBody::Rigid(rb.into()),
        }
    }
}

impl From<&Collider> for WriteCollider {
    fn from(value: &Collider) -> Self {
        let transform = match &value.body {
            ColliderBody::Particle(_) => WriteTransform::from_translate(value.position),
            ColliderBody::Rigid(rb) => (&rb.transform).into(),
        };
        
        WriteCollider {
            id: value.id,
            transform,
        }
    }
}

impl std::hash::Hash for Collider {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u32(self.id);
    }
}
