use glam::{Mat3, Quat, UVec3, Vec3};
use sokudo_io::read::collider::ParsedRigidBody;

use crate::{shape::{AbstractShape, Shape}, transform::Transform};

#[derive(Debug)]
pub struct RigidBody {
    /// The shape of this rigid body.
    pub shape: Shape,
    /// The scale of the rigid body, in all three dimensions.
    pub scale: Vec3,
    /// The mass of this rigid body.
    pub mass: f32,
    /// The resolution of the vertices, in all three dimensions.
    pub vertex_resolution: UVec3,
    /// The precomputed vertices to test for intersections on this rigid body.
    pub vertices: Vec<Vec3>,

    /// The inverse of the inertia tensor of this rigid body, in local coordinates.
    pub inertia_tensor: InertiaTensor, 

    pub rotation: Quat,
    pub previous_rotation: Quat,
    pub angular_velocity: Vec3,
    pub previous_angular_velocity: Vec3,
}

impl RigidBody {
    pub fn compute_vertices(&mut self) {
        if self.vertices.is_empty() {
            self.vertices = self.shape.vertices(self.vertex_resolution);
        }
    }

    pub fn compute_inertia_tensor(&mut self) {
        self.inertia_tensor = InertiaTensor::new(self.shape.moments(self.scale));
    }

    // TODO: Maybe store global inverse inertia tensor as well + update per frame?
    pub fn global_inverse_inertia(&self) -> Mat3 {
        self.inertia_tensor.rotate(self.rotation).inverse()
    }

    /// Compute the generalized inverse mass of this rigid body at point `r` when applying
    /// positional correction along the vector `n` where `r` is relative to the body's center of
    /// mass in global coordinates.
    pub fn positional_inverse_mass(&self, r: Vec3, n: Vec3) -> f32 {
        let r_cross_n = r.cross(n);
        (1.0 / self.mass) + r_cross_n.dot(self.global_inverse_inertia() * r_cross_n)
    }
}

impl From<ParsedRigidBody> for RigidBody {
    fn from(value: ParsedRigidBody) -> Self {
        RigidBody {
            shape: value.shape.into(),
            mass: value.mass,
            vertex_resolution: if value.vertex_resolution == UVec3::ZERO {
                UVec3::ONE
            } else {
                value.vertex_resolution
            },
            vertices: value.vertices,

            inertia_tensor: InertiaTensor::INFINITY,
            previous_rotation: value.transform.rotate,
            angular_velocity: Vec3::ZERO,
            previous_angular_velocity: Vec3::ZERO,
            rotation: value.transform.rotate,
            scale: value.transform.scale,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct InertiaTensor {
    inverse: Mat3
}

impl Default for InertiaTensor {
    fn default() -> Self {
        Self::INFINITY
    }
}

impl InertiaTensor {
    pub const INFINITY: Self = Self {
        inverse: Mat3::ZERO,
    };

    #[inline]
    pub fn new(principal_moments: Vec3) -> Self {
        let rcp = principal_moments.recip();
        Self::from_inverse_tensor(Mat3::from_diagonal(
            if rcp.is_finite() {
                rcp
            } else {
                Vec3::ZERO
            }
        ))
    }

    #[inline]
    pub fn from_tensor(tensor: Mat3) -> Self {
        Self::from_inverse_tensor(tensor.inverse())
    }

    #[inline]
    pub fn from_inverse_tensor(inverse_tensor: Mat3) -> Self {
        Self {
            inverse: inverse_tensor
        }
    }

    #[inline]
    pub fn inverse(self) -> Mat3 {
        self.inverse
    }

    #[inline]
    pub fn tensor(self) -> Mat3 {
        self.inverse.inverse()
    }

    #[inline]
    pub fn inverse_mut(&mut self) -> &mut Mat3 {
        &mut self.inverse
    }

    #[inline]
    pub fn rotate(self, q: Quat) -> Self {
        let r = Mat3::from_quat(q);
        Self::from_inverse_tensor((r * self.inverse) * r.transpose())
    }

    #[inline]
    pub fn is_finite(&self) -> bool {
        !self.is_infinite() && !self.is_nan()
    }

    #[inline]
    pub fn is_infinite(&self) -> bool {
        *self == Self::INFINITY
    }

    #[inline]
    pub fn is_nan(&self) -> bool {
        self.inverse.is_nan()
    }
}
