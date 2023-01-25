use core::fmt;

use auto_ops::{impl_op_ex_commutative, impl_op_ex};

use super::{vec3f::Vec3f, vec4f::Vec4f};

#[derive(Clone, Copy, PartialEq)]
pub struct Mat4f {
    pub values: [f32; 4 * 4]
}

const fn cell(x: usize, y: usize) -> usize {
    x * 4 + y
}

impl Mat4f {
    /// Creates an identity matrix
    pub const fn identity() -> Self {
        Mat4f { values: [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        ] }
    }

    /// Get a value in the matrix
    /// 
    /// # Arguments
    /// 
    /// * `row` - The row in the matrix
    /// * `column` - The column in the matrix
    pub const fn get(self, row: usize, column: usize) -> f32 {
        self.values[cell(column, row)]
    }

    /// Set a value in the matrix
    /// 
    /// # Arguments
    /// 
    /// * `row` - The row in the matrix
    /// * `column` - The column in the matrix
    /// * `value` - The value to set
    pub fn set(&mut self, row: usize, column: usize, value: f32) {
        self.values[cell(column, row)] = value;
    }

    /// Creates a translation matrix
    /// 
    /// # Arguments
    /// 
    /// * `translation` - The translation as a 3D vector (t<sub>x</sub>, t<sub>y</sub>, t<sub>z</sub>)
    pub const fn translate(translation: Vec3f) -> Self {
        let mut res = Self::identity();
        res.values[cell(3,0)] = translation.x;
        res.values[cell(3,1)] = translation.y;
        res.values[cell(3,2)] = translation.z;
        res
    }

    /// Creates a rotation matrix
    /// 
    /// # Arguments
    /// 
    /// * `yaw` - Angle of rotation (in radians) about the y-axis
    /// * `pitch` - Angle of rotation (in radians) about the x-axis
    /// * `roll` - Angle of rotation (in radians) about the z-axis
    pub fn rotate_yaw_pitch_roll(yaw: f32, pitch: f32, roll: f32) -> Self {
        let sin_yaw = yaw.sin();
        let cos_yaw = yaw.cos();
        let sin_pitch = pitch.sin();
        let cos_pitch = pitch.cos();
        let sin_roll = roll.sin();
        let cos_roll = roll.cos();

        let mut res = Self::identity();
        res.values[cell(0, 0)] = cos_yaw * cos_roll + sin_yaw * sin_pitch * sin_roll;
        res.values[cell(1, 0)] = sin_yaw * sin_pitch * cos_roll - cos_yaw * sin_roll;
        res.values[cell(2, 0)] = sin_yaw * cos_pitch;
        res.values[cell(0, 1)] = cos_pitch * sin_roll;
        res.values[cell(1, 1)] = cos_pitch * cos_roll;
        res.values[cell(2, 1)] = -sin_pitch;
        res.values[cell(0, 2)] = cos_yaw * sin_pitch * sin_roll - sin_yaw * cos_roll;
        res.values[cell(1, 2)] = sin_yaw * sin_roll + cos_yaw * sin_pitch * cos_roll;
        res.values[cell(2, 2)] = cos_yaw * cos_pitch;
        res
    }

    /// Creates a scale matrix
    /// 
    /// # Arguments
    /// 
    /// * `scale` - The scale as a 3D vector (s<sub>x</sub>, s<sub>y</sub>, s<sub>z</sub>)
    pub const fn scale(scale: Vec3f) -> Self {
        let mut res = Self::identity();
        res.values[cell(0,0)] = scale.x;
        res.values[cell(1,1)] = scale.x;
        res.values[cell(2,2)] = scale.x;
        res
    }

    /// Creates a transformation matrix.
    /// Combines translation, rotation, and scale into a single matrix.
    /// 
    /// # Arguments
    /// 
    /// * `translation` - The translation component as a 3D vector (t<sub>x</sub>, t<sub>y</sub>, t<sub>z</sub>)
    /// * `rotation` - The rotation component as a 3D vector (r<sub>x</sub>, r<sub>y</sub>, r<sub>z</sub>)
    /// * `scale` - The rotation component as a 3D vector (s<sub>x</sub>, s<sub>y</sub>, s<sub>z</sub>)
    pub fn transformation(translation: Vec3f, rotation: Vec3f, scale: Vec3f) -> Self {
        Self::translate(translation) *
        Self::rotate_yaw_pitch_roll(rotation.y, rotation.x, rotation.z) *
        Self::scale(scale)
    }

    /// Creates an orthographic projection matrix.
    /// Centered on the origin.
    /// 
    /// # Arguments
    /// 
    /// * `width` - The width of the viewport
    /// * `height` - The height of the viewport
    /// * `near` - The near clipping plane
    /// * `far` - The far clipping plane
    pub fn ortho(width: f32, height: f32, near: f32, far: f32) -> Self {
        let half_width = width / 2.0;
        let half_height: f32 = height / 2.0;
        Self::ortho_off_center(
            -half_width, half_width,
            half_height, -half_height,
            near, far
        )
    }

    /// Creates an orthographic projection matrix.
    /// Defined by viewport bounds.
    /// 
    /// # Arguments
    /// 
    /// * `left` - The left bound of the viewport
    /// * `right` - The right bound of the viewport
    /// * `top` - The top bound of the viewport
    /// * `bottom` - The bottom bound of the viewport
    /// * `near` - The near clipping plane
    /// * `far` - The far clipping plane
    pub fn ortho_off_center(left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) -> Self {
        let range = 1.0 / (far - near);
        let mut res = Mat4f::identity();
        res.values[cell(0, 0)] = 2.0 / (right - left);
        res.values[cell(1, 1)] = 2.0 / (top - bottom);
        res.values[cell(2, 2)] = range;
        res.values[cell(3, 0)] = (left + right) / (left - right);
        res.values[cell(3, 1)] = (top + bottom) / (bottom - top);
        res.values[cell(3, 2)] = -near * range;
        res
    }

    /// Creates a perspective projection matrix.
    /// Centered on the origin.
    /// 
    /// # Arguments
    /// 
    /// * `fov` - The field of view (in radians)
    /// * `aspect` - The aspect ratio of the viewport
    /// * `near` - The near clipping plane
    /// * `far` - The far clipping plane
    pub fn persp_fov(fov: f32, aspect: f32, near: f32, far: f32) -> Self {

        let y_scale = 1.0 / (fov * 0.5).tan();
        let q = far / (far - near);

        let mut res = Mat4f::identity();
        res.values[cell(0, 0)] = y_scale / aspect;
        res.values[cell(1, 1)] = y_scale;
        res.values[cell(2, 2)] = q;
        res.values[cell(2, 3)] = 1.0;
        res.values[cell(3, 2)] = -q * near;
        res
    }
}

impl_op_ex_commutative!(* | a: &Mat4f, b: &f32 | -> Mat4f {
    let mut values = a.values;
    for i in 0..values.len() { values[i] *= b; }
    Mat4f { values: values}
});

impl_op_ex!(* | a: &Mat4f, b: &Vec4f | -> Vec4f {
    Vec4f {
        x: a.values[cell(0, 0)] * b.x + a.values[cell(1, 0)] * b.y + a.values[cell(2, 0)] * b.z + a.values[cell(3, 0)] * b.w,
        y: a.values[cell(0, 1)] * b.x + a.values[cell(1, 1)] * b.y + a.values[cell(2, 1)] * b.z + a.values[cell(3, 1)] * b.w,
        z: a.values[cell(0, 2)] * b.x + a.values[cell(1, 2)] * b.y + a.values[cell(2, 2)] * b.z + a.values[cell(3, 2)] * b.w,
        w: a.values[cell(0, 3)] * b.x + a.values[cell(1, 3)] * b.y + a.values[cell(2, 3)] * b.z + a.values[cell(3, 3)] * b.w
    }
});

impl_op_ex!(* | a: &Mat4f, b: &Mat4f | -> Mat4f {
    let mut res = Mat4f::identity();

    for r in 0..4 {
        for c in 0..4 {
            res.values[cell(c, r)] = 
                a.values[cell(0, r)] * b.values[cell(c, 0)] +
                a.values[cell(1, r)] * b.values[cell(c, 1)] +
                a.values[cell(2, r)] * b.values[cell(c, 2)] +
                a.values[cell(3, r)] * b.values[cell(c, 3)];
        }
    }
    res
});

impl fmt::Display for Mat4f {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"\
        ┌ {}, {}, {}, {} ┐\n\
        │ {}, {}, {}, {} │\n\
        │ {}, {}, {}, {} │\n\
        └ {}, {}, {}, {} ┘",
        self.values[cell(0, 0)],
        self.values[cell(1, 0)],
        self.values[cell(2, 0)],
        self.values[cell(3, 0)],
        self.values[cell(0, 1)],
        self.values[cell(1, 1)],
        self.values[cell(2, 1)],
        self.values[cell(3, 1)],
        self.values[cell(0, 2)],
        self.values[cell(1, 2)],
        self.values[cell(2, 2)],
        self.values[cell(3, 2)],
        self.values[cell(0, 3)],
        self.values[cell(1, 3)],
        self.values[cell(2, 3)],
        self.values[cell(3, 3)])
    }
}