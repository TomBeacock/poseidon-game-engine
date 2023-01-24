use core::fmt;
use std::ops;

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

impl ops::Add<Vec2f> for Vec2f {
    type Output = Vec2f;

    fn add(self, rhs: Vec2f) -> Vec2f {
        Vec2f {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl ops::AddAssign<Vec2f> for Vec2f {
    fn add_assign(& mut self, rhs: Vec2f) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub<Vec2f> for Vec2f {
    type Output = Vec2f;

    fn sub(self, rhs: Vec2f) -> Vec2f {
        Vec2f { 
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl ops::SubAssign<Vec2f> for Vec2f {
    fn sub_assign(& mut self, rhs: Vec2f) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::Mul<f32> for Vec2f {
    type Output = Vec2f;

    fn mul(self, rhs: f32) -> Vec2f {
        Vec2f {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl ops::MulAssign<f32> for Vec2f {
    fn mul_assign(& mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl ops::Div<f32> for Vec2f {
    type Output = Vec2f;

    fn div(self, rhs: f32) -> Vec2f {
        Vec2f {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}

impl ops::DivAssign<f32> for Vec2f {
    fn div_assign(& mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl fmt::Display for Vec2f {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}