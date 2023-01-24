use core::fmt;
use std::ops;

use auto_ops::{impl_op_ex, impl_op_ex_commutative};

#[derive(Clone, Copy, PartialEq)]
pub struct Vec4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Vec4f {
    /// Creates vector: (0, 0, 0, 0)
    pub const fn zero() -> Self {
        Vec4f { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
    }

    /// Creates vector: (1, 1, 1, 1)
    pub const fn one() -> Self {
        Vec4f { x: 1.0, y: 1.0, z: 1.0, w: 1.0 }
    }
 
    /// Calculates the distance between two points in 4D space
    pub fn distance(lhs: Vec4f, rhs: Vec4f) -> f32 {
        Vec4f {
            x: lhs.x - rhs.x,
            y: lhs.y - rhs.y,
            z: lhs.z - rhs.z,
            w: lhs.w - rhs.w
        }.magnitude()
    }

    /// Calculate the dot product of two vectors
    pub fn dot(lhs: Vec4f, rhs: Vec4f) -> f32 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z + lhs.w * rhs.w
    }

    /// Get the squared length of the vector
    /// 
    /// Recommended when comparing lengths as it
    /// avoids unnecessary square root operations
    pub fn sqr_magnitude(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
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
        self.w *= scale;
    }

    /// Get this vector normalized (Scale of length 1)
    pub fn normalized(self) -> Vec4f {
        let mag = self.magnitude();
        if mag == 0.0 { return Vec4f::zero(); }

        let scale = 1.0 / mag;
        Vec4f {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
            w: self.w * scale
        }
    }
}

impl_op_ex!(- | a: &Vec4f | -> Vec4f {
    Vec4f { 
        x: -a.x,
        y: -a.y,
        z: -a.z,
        w: -a.w,
    }
});

impl_op_ex!(+ | a: &Vec4f, b: &Vec4f | -> Vec4f {
    Vec4f {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
        w: a.w + b.w
    }
});

impl_op_ex!(- | a: &Vec4f, b: &Vec4f | -> Vec4f {
    Vec4f {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
        w: a.w - b.w
    }
});

impl_op_ex_commutative!(* | a: &Vec4f, b: &f32 | -> Vec4f {
    Vec4f {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b,
        w: a.w * b
    }
});

impl_op_ex!(/ | a: &Vec4f, b: &f32 | -> Vec4f {
    Vec4f {
        x: a.x / b,
        y: a.y / b,
        z: a.z / b,
        w: a.w / b
    }
});

impl_op_ex!(+= | a: &mut Vec4f, b: &Vec4f| {
    a.x += b.x;
    a.y += b.y;
    a.z += b.z;
    a.w += b.w;
});

impl_op_ex!(-= | a: &mut Vec4f, b: &Vec4f| {
    a.x -= b.x;
    a.y -= b.y;
    a.z -= b.z;
    a.w -= b.w;
});

impl_op_ex!(*= | a: &mut Vec4f, b: &f32| {
    a.x *= b;
    a.y *= b;
    a.z *= b;
    a.w *= b;
});

impl_op_ex!(/= | a: &mut Vec4f, b: &f32| {
    a.x /= b;
    a.y /= b;
    a.z /= b;
    a.w /= b;
});

impl fmt::Display for Vec4f {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}