use std::ffi::c_void;

/// Types of attributes in array buffers
#[derive(Clone, Copy)]
pub enum AttributeType {
    Float, Vec2f, Vec3f, Vec4f,
}

impl AttributeType {
    /// Get the size of the attribute (in bytes)
    pub const fn size(&self) -> u32 {
        match *self {
            AttributeType::Float => 4,
            AttributeType::Vec2f => 4 * 2,
            AttributeType::Vec3f => 4 * 3,
            AttributeType::Vec4f => 4 * 4
        }
    }

    /// Get the number of components of the attributes
    pub const fn component_count(&self) -> u32 {
        match *self {
            AttributeType::Float => 1,
            AttributeType::Vec2f => 2,
            AttributeType::Vec3f => 3,
            AttributeType::Vec4f => 4
        }
    }

    // Get the type as an opengl type
    pub const fn opengl_type(&self) -> u32 {
        match *self {
            AttributeType::Float |
            AttributeType::Vec2f |
            AttributeType::Vec3f |
            AttributeType::Vec4f => gl::FLOAT
        }
    }
}

/// Defines an attribute of a buffer
#[derive(Clone, Copy)]
pub struct BufferAttribute {
    attribute_type: AttributeType,
    normalized: bool
}

impl BufferAttribute {
    /// Creates a new `BufferAttribute`
    /// 
    /// # Arguments
    /// 
    /// * `attribute_type` - The type of the attribute
    /// * `normalized` - Whether the attribute is normalized
    pub const fn new(attribute_type: AttributeType, normalized: bool) -> Self {
        BufferAttribute { attribute_type, normalized }
    }

    /// Get the attribute type
    pub const fn attribute_type(self) -> AttributeType {
        self.attribute_type
    }

    // Get whether the attribute is normalized
    pub const fn normalized(self) -> bool {
        self.normalized
    }
}

/// Describes the layout of an array buffer
pub struct BufferLayout {
    attributes: Vec<BufferAttribute>,
    offsets: Vec<u32>,
    stride: u32
}

impl BufferLayout {
    /// Creates a new `BufferLayout`
    /// 
    /// # Arguments
    /// 
    /// * `attributes` - Vector of buffer attributes
    pub fn new(attributes: Vec<BufferAttribute>) -> Self {
        let mut offsets = vec![0; attributes.len()];
        let mut offset = 0;
        for (i, attr) in attributes.iter().enumerate() {
            offsets[i] = offset;
            offset += attr.attribute_type().size();
        }
        BufferLayout {
            attributes, offsets, stride: offset
        }
    }

    /// Get the layout attributes
    pub fn attributes(&self) -> &Vec<BufferAttribute> {
        &self.attributes
    }

    /// Get the offsets (in bytes) for each attribute
    pub fn offsets(&self) -> &Vec<u32> {
        &self.offsets
    }

    /// Get the stride (in bytes) between each array element
    pub fn stride(&self) -> u32 {
        self.stride
    }
}

/// A buffer of graphics data
pub struct ArrayBuffer {
    id: u32,
    layout: BufferLayout
}

impl ArrayBuffer {
    /// Creates a new `ArrayBuffer`
    /// 
    /// # Arguments
    /// 
    /// * `layout` - The layout of the buffer
    pub fn new(layout: BufferLayout) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        ArrayBuffer { id, layout }
    }

    /// Get the buffer's layout
    pub const fn layout(&self) -> &BufferLayout {
        &self.layout
    }

    /// Make this buffer the active `ArrayBuffer`
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    /// Unbind the current `ArrayBuffer`
    pub fn unbind() {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    /// Set `ArrayBuffer`'s data
    /// 
    /// # Arguments
    /// 
    /// * `data` - A c style void pointer to the data
    /// * `size` - The size of the data
    pub fn set_data(&self, data: *const c_void, size: usize) {
        self.bind();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                size as isize,
                data,
                gl::STATIC_DRAW
            );
        }
    }
}

impl Drop for ArrayBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id)
        }
    }
}