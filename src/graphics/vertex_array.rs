use super::{array_buffer::{ArrayBuffer, AttributeType}, index_buffer::IndexBuffer};

/// An array of vertex data
pub struct VertexArray {
    id: u32
}

impl VertexArray {
    /// Creates a new `VertexArray`
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        VertexArray { id }
    }

    /// Make this buffer the active `VertexArray`
    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    /// Unbind the current `VertexArray`
    pub fn unbind() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    /// Adds a `VertexBuffer` to this array
    /// 
    /// # Arguments
    /// 
    /// * `vertex_buffer` - The `VertexBuffer` to add
    pub fn add_vertex_buffer(&self, vertex_buffer: &ArrayBuffer) {
        self.bind();
        vertex_buffer.bind();

        let layout = vertex_buffer.layout();
        let attributes = layout.attributes();
        let offsets = layout.offsets();

        for i in 0..attributes.len() {
            let attribute_type = attributes[i].attribute_type();
            unsafe {
                gl::EnableVertexAttribArray(i as u32);
                match attribute_type {
                    AttributeType::Int => {
                        gl::VertexAttribIPointer(
                            i as u32,
                            attribute_type.component_count() as i32,
                            attribute_type.opengl_type(),
                            layout.stride() as i32,
                            offsets[i] as *const _
                        );
                    }
                    AttributeType::Float |
                    AttributeType::Vec2f |
                    AttributeType::Vec3f |
                    AttributeType::Vec4f => {
                        gl::VertexAttribPointer(
                            i as u32,
                            attribute_type.component_count() as i32,
                            attribute_type.opengl_type(),
                            if attributes[i].normalized() { gl::TRUE } else { gl::FALSE },
                            layout.stride() as i32,
                            offsets[i] as *const _
                        );
                    }
                }
            }
        }
        Self::unbind();
    }

    /// Sets the `IndexBuffer` of this array
    /// 
    /// # Arguments
    /// 
    /// * `index_buffer` - The `IndexBuffer` to set
    pub fn set_index_buffer(&self, index_buffer: &IndexBuffer) {
        self.bind();
        index_buffer.bind();
        Self::unbind();
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id)
        }
    }
}