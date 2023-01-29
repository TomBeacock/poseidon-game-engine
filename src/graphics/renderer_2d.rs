use std::ffi::CString;
use std::mem::size_of_val;

use crate::graphics::index_buffer::IndexBuffer;
use crate::graphics::renderer::Renderer;
use crate::math::{vec2f::Vec2f, vec3f::Vec3f, vec4f::Vec4f};
use crate::math::mat4f::Mat4f;
use super::array_buffer::{BufferLayout, BufferAttribute, AttributeType, ArrayBuffer};
use super::{shader::Shader, vertex_array::VertexArray};

/// Stores a rectangle
pub struct Rect {
    pub position: Vec3f,
    pub size: Vec2f,
    pub pivot: Vec2f
}

impl Rect {
    /// Creates a new `Rect`
    /// 
    /// # Arguments
    /// 
    /// * `position` - Position of the rect (from pivot)
    /// * `size` - Size of the rect
    /// * `pivot` - Pivot point (bottom-left (0.0, 0.0), top-right (1.0, 1.0))
    pub fn new(position: Vec3f, size: Vec2f, pivot: Vec2f) -> Self {
        Rect { position, size, pivot }
    }

    /// Get left bound
    pub fn left(&self) -> f32 {
        self.position.x - self.size.x * self.pivot.x
    }

    /// Get right bound
    pub fn right(&self) -> f32 {
        self.position.x + self.size.x * (1.0 - self.pivot.x)
    }

    /// Get bottom bound
    pub fn bottom(&self) -> f32 {
        self.position.y - self.size.y * self.pivot.y
    }

    /// Get top bound
    pub fn top(&self) -> f32 {
        self.position.y + self.size.y * (1.0 - self.pivot.y)
    }

    /// Get all bounds (left, right, top, bottom)
    pub fn bounds(&self) -> (f32, f32, f32, f32) {
        (self.left(), self.right(), self.top(), self.bottom())
    }
}

/// Renderer for 2D graphics
pub struct Renderer2D {
    default_shader: Shader
}

impl Renderer2D {
    /// Creates new `Renderer2D`
    /// 
    /// # Arguments
    /// 
    /// * `view_projection` - The view projection matrix to use
    pub fn new(view_projection: Mat4f) -> Self {
        const VERTEX_SHADER: &str = r#"#version 330 core
        layout (location = 0) in vec3 v_in_position;
        layout (location = 1) in vec4 v_in_color;
    
        out vec4 v_out_color;

        uniform mat4 u_view_projection;
    
        void main() {
            v_out_color = v_in_color;
            gl_Position = u_view_projection * vec4(v_in_position, 1.0);
        }
        "#;
    
        const FRAGMENT_SHADER: &str = r#"#version 330 core
        in vec4 v_out_color; 

        out vec4 f_out_color;
    
        void main() {
            f_out_color = v_out_color;
        }
        "#;
    
        let default_shader = Shader::new(
            VERTEX_SHADER, 
            FRAGMENT_SHADER
        );
        default_shader.bind();
        default_shader.set_mat4f(&CString::new("u_view_projection").unwrap(), view_projection);

        Renderer2D { default_shader }
    }

    /// Draw a rect
    /// 
    /// # Arguments
    /// 
    /// * `rect` - The rect to draw
    /// * `color` - The color to draw with
    pub fn draw_rect(&self, rect: Rect, color: Vec4f) {
        let vertex_array = VertexArray::new();
        vertex_array.bind();
        
        let bounds = rect.bounds();
        // Each vertex has 7 values: x, y, z, r, g, b, a
        type RectVertex = [f32; 7];
        let vertices : [RectVertex; 4] = [
            [bounds.0, bounds.3, rect.position.z, color.x, color.y, color.z, color.w],
            [bounds.1, bounds.3, rect.position.z, color.x, color.y, color.z, color.w],
            [bounds.1, bounds.2, rect.position.z, color.x, color.y, color.z, color.w],
            [bounds.0, bounds.2, rect.position.z, color.x, color.y, color.z, color.w]
            ];

        let vertex_layout = BufferLayout::new(Vec::from([
            BufferAttribute::new(AttributeType::Vec3f, false),
            BufferAttribute::new(AttributeType::Vec4f, false)
        ]));
        let vertex_buffer = ArrayBuffer::new(vertex_layout);
        vertex_buffer.set_data(vertices.as_ptr().cast(), size_of_val(&vertices));
        
        const INDICES: [u32; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = IndexBuffer::new();
        index_buffer.set_data(INDICES.as_ptr().cast(), size_of_val(&INDICES));
        
        vertex_array.add_vertex_buffer(&vertex_buffer);
        vertex_array.set_index_buffer(&index_buffer);
        
        self.default_shader.bind();
        Renderer::draw_elements(&vertex_array, 6);
    }
}