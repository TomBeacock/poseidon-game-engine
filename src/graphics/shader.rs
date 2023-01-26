use std::ffi::CString;

use crate::math::{vec2f::Vec2f, vec3f::Vec3f, vec4f::Vec4f, mat4f::Mat4f};

pub struct Shader {
    program: u32
}

impl Shader {
    /// Creates a new `Shader`
    /// 
    /// # Arguments
    /// 
    /// * `vertex_source` - The source code of the vertex shader
    /// * `fragment_source` - The source code of the fragment shader
    pub fn new(vertex_source: &str, fragment_source: &str) -> Self {
        let program: u32;

        unsafe {
            // Vertex shader
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            assert_ne!(vertex_shader, 0);
            gl::ShaderSource(
                vertex_shader,
                1,
                &(vertex_source.as_bytes().as_ptr().cast()),
                &(vertex_source.len().try_into().unwrap())
            );
            gl::CompileShader(vertex_shader);

            let mut success = 0;
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0;
                gl::GetShaderInfoLog(
                    vertex_shader,
                    1024,
                    &mut log_len,
                    v.as_mut_ptr().cast(),
                );
                v.set_len(log_len.try_into().unwrap());
                panic!("Vertex shader compile error: {}", String::from_utf8_lossy(&v));
            }

            // Fragment shader
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            assert_ne!(fragment_shader, 0);
            gl::ShaderSource(
                fragment_shader,
                1,
                &(fragment_source.as_bytes().as_ptr().cast()),
                &(fragment_source.len().try_into().unwrap())
            );
            gl::CompileShader(fragment_shader);

            let mut success = 0;
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0;
                gl::GetShaderInfoLog(
                    vertex_shader,
                    1024,
                    &mut log_len,
                    v.as_mut_ptr().cast(),
                );
                v.set_len(log_len.try_into().unwrap());
                panic!("Fragment shader compile error: {}", String::from_utf8_lossy(&v));
            }

            // Program
            program = gl::CreateProgram();
            assert_ne!(program, 0);
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);

            let mut success = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetProgramInfoLog(
                    program,
                    1024,
                    &mut log_len,
                    v.as_mut_ptr().cast(),
                );
                v.set_len(log_len.try_into().unwrap());
                panic!("Program link error: {}", String::from_utf8_lossy(&v));
            }
    
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }
        Shader { program }
    }

    /// Make this shader the active `Shader`
    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }

    /// Unbind the current `Shader`
    pub fn unbind() {
        unsafe {
            gl::UseProgram(0);
        }
    }

    /// Set a float shader variable
    /// 
    /// # Arguments
    /// 
    /// `name` - The name of the variable
    /// `val` - The float value to set
    pub fn set_float(&self, name: &CString, val: f32) {
        unsafe {
            let location = gl::GetUniformLocation(
                self.program,
                name.as_ptr()
            );
            gl::Uniform1f(location, val);
        }
    }

    /// Set a 2D vector shader variable
    /// 
    /// # Arguments
    /// 
    /// `name` - The name of the variable
    /// `val` - The `Vec2f` value to set
    pub fn set_vec2f(&self, name: &CString, val: Vec2f) {
        unsafe {
            let location = gl::GetUniformLocation(
                self.program,
                name.as_ptr()
            );
            gl::Uniform2f(location, val.x, val.y);
        }
    }

    /// Set a 3D vector shader variable
    /// 
    /// # Arguments
    /// 
    /// `name` - The name of the variable
    /// `val` - The `Vec3f` value to set
    pub fn set_vec3f(&self, name: &CString, val: Vec3f) {
        unsafe {
            let location = gl::GetUniformLocation(
                self.program,
                name.as_ptr()
            );
            gl::Uniform3f(location, val.x, val.y, val.z);
        }
    }

    /// Set a 4D vector shader variable
    /// 
    /// # Arguments
    /// 
    /// `name` - The name of the variable
    /// `val` - The `Vec4f` value to set
    pub fn set_vec4f(&self, name: &CString, val: Vec4f) {
        unsafe {
            let location = gl::GetUniformLocation(
                self.program,
                name.as_ptr()
            );
            gl::Uniform4f(location, val.x, val.y, val.z, val.w);
        }
    }

    /// Set a 4x4 matrix shader variable
    /// 
    /// # Arguments
    /// 
    /// `name` - The name of the variable
    /// `val` - The `Mat4f` value to set
    pub fn set_mat4f(&self, name: &CString, val: Mat4f) {
        unsafe {
            let location = gl::GetUniformLocation(
                self.program,
                name.as_ptr()
            );
            gl::UniformMatrix4fv(
                location,
                1,
                gl::FALSE,
                val.values.as_ptr()
            );
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
        }
    }
}