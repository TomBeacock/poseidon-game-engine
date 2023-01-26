use crate::math::vec4f::Vec4f;

use super::vertex_array::VertexArray;

static mut INITIALIZED: bool = false;

pub struct Renderer {}

impl Renderer {
    pub fn init() {
        unsafe { 
            if INITIALIZED { return; }

            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            INITIALIZED = true;
        }
    }

    pub fn set_viewport(x: u32, y: u32, width: u32, height: u32) {
        unsafe {
            gl::Viewport(x as i32, y as i32, width as i32, height as i32);
        }
    }

    pub fn set_clear_color(color: Vec4f) {
        unsafe {
            gl::ClearColor(color.x, color.y, color.z, color.w);
        }
    }

    pub fn clear() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn draw_elements(vertex_array: &VertexArray, count: u32) {
        vertex_array.bind();
        unsafe {
            gl::DrawElements(gl::TRIANGLES, count as i32, gl::UNSIGNED_INT, 0 as *const _)
        }
    }
}