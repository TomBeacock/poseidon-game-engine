use core::fmt;
use auto_ops::{impl_op_ex, impl_op_ex_commutative};

/// A 2D Vector with f32 components
#[derive(Clone, Copy, PartialEq)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32
}

impl Default for Vec2f { 
    fn default() -> Self {
        Self::zero()
    }
}

impl Vec2f {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Creates vector: (0, 0)
    pub const fn zero() -> Self {
        Vec2f { x: 0.0, y: 0.0 }
    }

    /// Creates vector: (1, 1)
    pub const fn one() -> Self {
        Vec2f { x: 1.0, y: 1.0 }
    }

    /// Creates vector: (1, 0)
    pub const fn right() -> Self {
        Vec2f { x: 1.0, y: 0.0 }
    }

    /// Creates vector: (-1, 0)
    pub const fn left() -> Self {
        Vec2f { x: -1.0, y: 0.0 }
    }

    /// Creates vector: (0, 1)
    pub const fn up() -> Self {
        Vec2f { x: 0.0, y: 1.0 }
    }

    /// Creates vector: (0, -1)
    pub const fn down() -> Self {
        Vec2f { x: 0.0, y: -1.0 }
    }
 
    /// Calculates the distance between two points in 2D space
    pub fn distance(lhs: Vec2f, rhs: Vec2f) -> f32 {
        Vec2f {
            x: lhs.x - rhs.x,
            y: lhs.y - rhs.y
        }.magnitude()
    }

    /// Calculate the dot product of two vectors
    pub fn dot(lhs: Vec2f, rhs: Vec2f) -> f32 {
        lhs.x * rhs.x + lhs.y * rhs.y
    }

    /// Get the squared length of the vector
    /// 
    /// Recommended when comparing lengths as it
    /// avoids unnecessary square root operations
    pub fn sqr_magnitude(self) -> f32 {
        self.x * self.x + self.y * self.y
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
    }

    /// Get this vector normalized (Scale of length 1)
    pub fn normalized(self) -> Vec2f {
        let mag = self.magnitude();
        if mag == 0.0 { return Vec2f::zero(); }

        let scale = 1.0 / mag;
        Vec2f {
            x: self.x * scale,
            y: self.y * scale
        }
    }
}

impl_op_ex!(- | a: &Vec2f | -> Vec2f {
    Vec2f { 
        x: -a.x,
        y: -a.y
    }
});

impl_op_ex!(+ | a: &Vec2f, b: &Vec2f | -> Vec2f {
    Vec2f {
        x: a.x + b.x,
        y: a.y + b.y
    }
});

impl_op_ex!(- | a: &Vec2f, b: &Vec2f | -> Vec2f {
    Vec2f {
        x: a.x - b.x,
        y: a.y - b.y
    }
});

impl_op_ex_commutative!(* | a: &Vec2f, b: &f32 | -> Vec2f {
    Vec2f {
        x: a.x * b,
        y: a.y * b
    }
});

impl_op_ex!(/ | a: &Vec2f, b: &f32 | -> Vec2f {
    Vec2f {
        x: a.x / b,
        y: a.y / b
    }
});

impl_op_ex!(+= | a: &mut Vec2f, b: &Vec2f| {
    a.x += b.x;
    a.y += b.y;
});

impl_op_ex!(-= | a: &mut Vec2f, b: &Vec2f| {
    a.x -= b.x;
    a.y -= b.y;
});

impl_op_ex!(*= | a: &mut Vec2f, b: &f32| {
    a.x *= b;
    a.y *= b;
});

impl_op_ex!(/= | a: &mut Vec2f, b: &f32| {
    a.x /= b;
    a.y /= b;
});

impl fmt::Display for Vec2f {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}