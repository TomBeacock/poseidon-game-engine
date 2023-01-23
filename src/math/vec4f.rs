use core::fmt;
use std::ops;

#[derive(Copy, Clone)]
pub struct Vec4f {
    x: f32,
    y: f32,
    z: f32,
    w: f32
}

impl Vec4f {
    // Static Methods
    pub fn zero() -> Self {
        Vec4f { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
    }

    pub fn one() -> Self {
        Vec4f { x: 1.0, y: 1.0, z: 1.0, w: 1.0 }
    }
 
    pub fn distance(lhs: Vec4f, rhs: Vec4f) -> f32 {
        Vec4f {
            x: lhs.x - rhs.x,
            y: lhs.y - rhs.y,
            z: lhs.z - rhs.z,
            w: lhs.w - rhs.w
        }.magnitude()
    }

    pub fn dot(lhs: Vec4f, rhs: Vec4f) -> f32 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z + lhs.w * rhs.w
    }

    // Methods
    pub fn sqr_magnitude(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
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
        self.w *= scale;
    }

    pub fn normalized(self) -> Vec4f {
        let mag = self.magnitude();
        if mag == 0.0 { return Vec4f::zero(); }

        let scale = 1.0 / mag;
        Vec4f { x: self.x * scale, y: self.y * scale, z: self.z * scale, w: self.w * scale }
    }
}

impl ops::Add<Vec4f> for Vec4f {
    type Output = Vec4f;

    fn add(self, rhs: Vec4f) -> Vec4f {
        Vec4f { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z, w: self.w + rhs.w }
    }
}

impl ops::Sub<Vec4f> for Vec4f {
    type Output = Vec4f;

    fn sub(self, rhs: Vec4f) -> Vec4f {
        Vec4f { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z, w: self.w - rhs.w }
    }
}

impl ops::Mul<f32> for Vec4f {
    type Output = Vec4f;

    fn mul(self, rhs: f32) -> Vec4f {
        Vec4f { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs, w: self.w + rhs }
    }
}

impl fmt::Display for Vec4f {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}