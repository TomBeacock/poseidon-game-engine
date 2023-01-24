use core::fmt;
use std::ops;

use auto_ops::{impl_op_ex, impl_op_ex_commutative};

/// A 3D Vector with f32 components
#[derive(Clone, Copy, PartialEq)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3f {
    /// Creates vector: (0, 0, 0) 
    pub const fn zero() -> Self {
        Vec3f { x: 0.0, y: 0.0, z: 0.0 }
    }

    /// Creates vector: (1, 1, 1)
    pub const fn one() -> Self {
        Vec3f { x: 1.0, y: 1.0, z: 1.0 }
    }

    /// Creates vector: (1, 0, 0)
    pub const fn right() -> Self {
        Vec3f { x: 1.0, y: 0.0, z: 0.0 }
    }

    /// Creates vector: (-1, 0, 0)
    pub const fn left() -> Self {
        Vec3f { x: -1.0, y: 0.0, z: 0.0 }
    }

    /// Creates vector: (0, 1, 0)
    pub const fn up() -> Self {
        Vec3f { x: 0.0, y: 1.0, z: 0.0 }
    }

    /// Creates vector: (0, -1, 0)
    pub const fn down() -> Self {
        Vec3f { x: 0.0, y: -1.0, z: 0.0 }
    }

    /// Creates vector: (0, 0, 1)
    pub const fn forward() -> Self {
        Vec3f { x: 0.0, y: 0.0, z: 1.0 }
    }

    /// Creates vector: (0, 0, -1)
    pub const fn back() -> Self {
        Vec3f { x: 0.0, y: 0.0, z: -1.0 }
    }
 
    /// Calculates the distance between two points in 3D space
    pub fn distance(lhs: Vec3f, rhs: Vec3f) -> f32 {
        Vec3f {
            x: lhs.x - rhs.x,
            y: lhs.y - rhs.y,
            z: lhs.z - rhs.z
        }.magnitude()
    }

    /// Calculate the dot product of two vectors
    pub fn dot(lhs: Vec3f, rhs: Vec3f) -> f32 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    /// Calculate the cross product of two vectors
    pub fn cross(lhs: Vec3f, rhs: Vec3f) -> Vec3f {
        Vec3f {
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: lhs.z * rhs.x - lhs.x * rhs.z,
            z: lhs.x * rhs.y - lhs.y * rhs.x
        }
    }

    /// Get the squared length of the vector
    /// 
    /// Recommended when comparing lengths as it
    /// avoids unnecessary square root operations
    pub fn sqr_magnitude(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Get the length of the vector
    /// 
    /// If comparing lengths use sqr_magnitude
    /// instead for improved performance
    pub fn magnitude(self) -> f32 {
        self.sqr_magnitude().sqrt()
    }

    /// Normalize this vector (Scale of length 1)
    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        if mag == 0.0 { return; }

        let scale = 1.0 / mag;
        self.x *= scale;
        self.y *= scale;
        self.z *= scale;
    }

    /// Get this vector normalized (Scale of length 1)
    pub fn normalized(self) -> Vec3f {
        let mag = self.magnitude();
        if mag == 0.0 { return Vec3f::zero(); }

        let scale = 1.0 / mag;
        Vec3f {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale
        }
    }
}

impl_op_ex!(- | a: &Vec3f | -> Vec3f {
    Vec3f { 
        x: -a.x,
        y: -a.y,
        z: -a.z
    }
});

impl_op_ex!(+ | a: &Vec3f, b: &Vec3f | -> Vec3f {
    Vec3f {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z
    }
});

impl_op_ex!(- | a: &Vec3f, b: &Vec3f | -> Vec3f {
    Vec3f {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z
    }
});

impl_op_ex_commutative!(* | a: &Vec3f, b: &f32 | -> Vec3f {
    Vec3f {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b
    }
});

impl_op_ex!(/ | a: &Vec3f, b: &f32 | -> Vec3f {
    Vec3f {
        x: a.x / b,
        y: a.y / b,
        z: a.z / b
    }
});

impl_op_ex!(+= | a: &mut Vec3f, b: &Vec3f| {
    a.x += b.x;
    a.y += b.y;
    a.z += b.z;
});

impl_op_ex!(-= | a: &mut Vec3f, b: &Vec3f| {
    a.x -= b.x;
    a.y -= b.y;
    a.z -= b.z;
});

impl_op_ex!(*= | a: &mut Vec3f, b: &f32| {
    a.x *= b;
    a.y *= b;
    a.z *= b;
});

impl_op_ex!(/= | a: &mut Vec3f, b: &f32| {
    a.x /= b;
    a.y /= b;
    a.z /= b;
});

impl fmt::Display for Vec3f {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}