use std::ffi::c_void;

/// A buffer of index graphics data
pub struct IndexBuffer {
    id: u32
}

impl IndexBuffer {
    /// Creates a new `IndexBuffer`
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        IndexBuffer { id }
    }

    /// Make this buffer the active `IndexBuffer`
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }

    /// Unbind the current `IndexBuffer`
    pub fn unbind() {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    /// Set `IndexBuffer`'s data
    /// 
    /// # Arguments
    /// 
    /// * `data` - A c style void pointer to the data
    /// * `size` - The size of the data
    pub fn set_data(&self, data: *const c_void, size: usize) {
        self.bind();
        unsafe {
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                size as isize,
                data,
                gl::STATIC_DRAW
            );
        }
    }
}

impl Drop for IndexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id)
        }
    }
}