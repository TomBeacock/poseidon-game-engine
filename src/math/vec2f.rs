use core::fmt;
use std::ops;

#[derive(Copy, Clone)]
pub struct Vec2f {
    x: f32,
    y: f32
}

impl Vec2f {
    // Static Methods
    pub fn zero() -> Self {
        Vec2f { x: 0.0, y: 0.0 }
    }

    pub fn one() -> Self {
        Vec2f { x: 1.0, y: 1.0 }
    }

    pub fn right() -> Self {
        Vec2f { x: 1.0, y: 0.0 }
    }

    pub fn left() -> Self {
        Vec2f { x: -1.0, y: 0.0 }
    }

    pub fn up() -> Self {
        Vec2f { x: 0.0, y: 1.0 }
    }

    pub fn down() -> Self {
        Vec2f { x: 0.0, y: -1.0 }
    }
 
    pub fn dot(lhs: Vec2f, rhs: Vec2f) -> f32 {
        lhs.x * rhs.x + lhs.y * rhs.y
    }

    pub fn distance(lhs: Vec2f, rhs: Vec2f) -> f32 {
       Vec2f { x: lhs.x - rhs.x, y: lhs.y - rhs.y}.magnitude()
    }

    // Methods
    pub fn sqr_magnitude(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn magnitude(self) -> f32 {
        self.sqr_magnitude().sqrt()
    }

    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        if mag == 0.0 { return; }

        let scale = 1.0 / mag;
        self.x *= scale;
        self.y *= scale;
    }

    pub fn normalized(self) -> Vec2f {
        let mag = self.magnitude();
        if mag == 0.0 { return Vec2f::zero(); }

        let scale = 1.0 / mag;
        Vec2f { x: self.x * scale, y: self.y * scale }
    }
}

impl ops::Add<Vec2f> for Vec2f {
    type Output = Vec2f;

    fn add(self, rhs: Vec2f) -> Vec2f {
        Vec2f { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl ops::Sub<Vec2f> for Vec2f {
    type Output = Vec2f;

    fn sub(self, rhs: Vec2f) -> Vec2f {
        Vec2f { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl ops::Mul<f32> for Vec2f {
    type Output = Vec2f;

    fn mul(self, rhs: f32) -> Vec2f {
        Vec2f { x: self.x * rhs, y: self.y * rhs }
    }
}

impl fmt::Display for Vec2f {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}