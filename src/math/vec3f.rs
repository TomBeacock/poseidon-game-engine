use core::fmt;
use std::ops;

#[derive(Copy, Clone)]
pub struct Vec3f {
    x: f32,
    y: f32,
    z: f32
}

impl Vec3f {
    // Static Methods
    pub fn zero() -> Self {
        Vec3f { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn one() -> Self {
        Vec3f { x: 1.0, y: 1.0, z: 1.0 }
    }

    pub fn right() -> Self {
        Vec3f { x: 1.0, y: 0.0, z: 0.0 }
    }

    pub fn left() -> Self {
        Vec3f { x: -1.0, y: 0.0, z: 0.0 }
    }

    pub fn up() -> Self {
        Vec3f { x: 0.0, y: 1.0, z: 0.0 }
    }

    pub fn down() -> Self {
        Vec3f { x: 0.0, y: -1.0, z: 0.0 }
    }

    pub fn forward() -> Self {
        Vec3f { x: 0.0, y: 0.0, z: 1.0 }
    }

    pub fn back() -> Self {
        Vec3f { x: 0.0, y: 0.0, z: -1.0 }
    }
 
    pub fn distance(lhs: Vec3f, rhs: Vec3f) -> f32 {
       Vec3f { x: lhs.x - rhs.x, y: lhs.y - rhs.y, z: lhs.z - rhs.z}.magnitude()
    }

    pub fn dot(lhs: Vec3f, rhs: Vec3f) -> f32 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    pub fn cross(lhs: Vec3f, rhs: Vec3f) -> Vec3f {
        Vec3f {
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: lhs.z * rhs.x - lhs.x * rhs.z,
            z: lhs.x * rhs.y - lhs.y * rhs.x
        }
    }

    // Methods
    pub fn sqr_magnitude(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
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
        self.z *= scale;
    }

    pub fn normalized(self) -> Vec3f {
        let mag = self.magnitude();
        if mag == 0.0 { return Vec3f::zero(); }

        let scale = 1.0 / mag;
        Vec3f { x: self.x * scale, y: self.y * scale, z: self.z * scale }
    }
}

impl ops::Add<Vec3f> for Vec3f {
    type Output = Vec3f;

    fn add(self, rhs: Vec3f) -> Vec3f {
        Vec3f { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl ops::Sub<Vec3f> for Vec3f {
    type Output = Vec3f;

    fn sub(self, rhs: Vec3f) -> Vec3f {
        Vec3f { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl ops::Mul<f32> for Vec3f {
    type Output = Vec3f;

    fn mul(self, rhs: f32) -> Vec3f {
        Vec3f { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl fmt::Display for Vec3f {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}