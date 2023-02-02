use std::ffi::CString;
use std::mem::{size_of_val, size_of};
use std::ops::Index;

use crate::graphics::index_buffer::IndexBuffer;
use crate::graphics::renderer::Renderer;
use crate::graphics::vertex_array;
use crate::math::{vec2f::Vec2f, vec3f::Vec3f, vec4f::Vec4f};
use crate::math::mat4f::Mat4f;
use super::array_buffer::{BufferLayout, BufferAttribute, AttributeType, ArrayBuffer};
use super::texture::Texture;
use super::{shader::Shader, vertex_array::VertexArray};

/// Stores a rectangle
pub struct Rect {
    pub position: Vec3f,
    pub size: Vec2f,
    pub pivot: Vec2f,
    pub uv_min: Vec2f,
    pub uv_max: Vec2f
}

impl Rect {
    /// Creates a new `Rect`
    /// 
    /// # Arguments
    /// 
    /// * `position` - Position of the rect (from pivot)
    /// * `size` - Size of the rect
    /// * `pivot` - Pivot point (bottom-left (0.0, 0.0), top-right (1.0, 1.0))
    pub fn new(position: Vec3f, size: Vec2f, pivot: Vec2f, uv_min: Vec2f, uv_max: Vec2f) -> Self {
        Rect { position, size, pivot, uv_min, uv_max }
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

const MAX_RECTS_IN_BATCH: u32 = 512;
const MAX_VERTS_IN_BATCH: u32 = MAX_RECTS_IN_BATCH * 4;
const MAX_INDICES_IN_BATCH: u32 = MAX_RECTS_IN_BATCH * 6;
const MAX_TEXTURE_SLOTS: u32 = 32;

#[derive(Clone, Copy)]
struct RectVertex {
    position: Vec3f,
    uv: Vec2f,
    color: Vec4f,
    slot: i32
}

impl RectVertex {
    pub fn new(position: Vec3f, uv: Vec2f, color: Vec4f, slot: i32) -> Self {
        RectVertex { position, uv, color, slot }
    }
}

impl Default for RectVertex {
    fn default() -> Self {
        Self { position: Vec3f::zero(), uv: Vec2f::zero(), color: Vec4f::zero(), slot: 0 }
    }
}

struct RectBatch {
    vertices: [RectVertex; MAX_VERTS_IN_BATCH as usize],
    next_rect: usize,
    next_texture_slot: usize,

    vertex_array: VertexArray,
    vertex_buffer: ArrayBuffer,
    index_buffer: IndexBuffer
}

impl RectBatch {
    pub fn new() -> Self {     
        // Vertex buffer
        let vertices = [RectVertex::default(); MAX_VERTS_IN_BATCH as usize];
        let buffer_layout = BufferLayout::new(Vec::from([
            BufferAttribute::new(AttributeType::Vec3f, false),
            BufferAttribute::new(AttributeType::Vec2f, false),
            BufferAttribute::new(AttributeType::Vec4f, false),
            BufferAttribute::new(AttributeType::Int, false),
        ]));
        let vertex_buffer = ArrayBuffer::new_dynamic(buffer_layout, size_of_val(&vertices));
        // Index buffer
        let index_buffer = IndexBuffer::new();
        let mut indices = [0u32; MAX_INDICES_IN_BATCH as usize];
        for i in 0..MAX_RECTS_IN_BATCH {
            let index = i * 6;
            let vertex = i * 4;
            indices[index as usize + 0] = vertex + 0;
            indices[index as usize + 1] = vertex + 1;
            indices[index as usize + 2] = vertex + 2;
            indices[index as usize + 3] = vertex + 0;
            indices[index as usize + 4] = vertex + 2;
            indices[index as usize + 5] = vertex + 3;
        }
        index_buffer.set_data(indices.as_ptr().cast(), MAX_INDICES_IN_BATCH as usize);
        
        // Vertex array
        let vertex_array = VertexArray::new();
        vertex_array.add_vertex_buffer(&vertex_buffer);
        vertex_array.set_index_buffer(&index_buffer);

        RectBatch { 
            vertices,
            next_rect: 0,
            next_texture_slot: 1,
            vertex_array,
            vertex_buffer,
            index_buffer
        }
    }

    pub fn reset(&mut self) {
        self.next_rect = 0;
        self.next_texture_slot = 1;
    }

    pub fn draw(&self) {
        self.vertex_buffer.set_data(
            self.vertices.as_ptr().cast(),
            size_of::<RectVertex>() * 4 * self.next_rect
        );
        Renderer::draw_elements(&self.vertex_array, (self.next_rect * 6) as u32)
    }

    pub fn add_rect(&mut self, rect: Rect, color: Vec4f) {
        let bounds = rect.bounds();

        let i = self.next_rect * 4;
        self.vertices[i + 0] = RectVertex::new(
            Vec3f::new(bounds.0, bounds.3, rect.position.z), rect.uv_min, color, 0);
        self.vertices[i + 1] = RectVertex::new(
            Vec3f::new(bounds.1, bounds.3, rect.position.z), Vec2f::new(rect.uv_max.x, rect.uv_min.y), color, 0);
        self.vertices[i + 2] = RectVertex::new(
            Vec3f::new(bounds.1, bounds.2, rect.position.z), rect.uv_max, color, 0);
        self.vertices[i + 3] = RectVertex::new(
            Vec3f::new(bounds.0, bounds.2, rect.position.z), Vec2f::new(rect.uv_min.x, rect.uv_max.y), color, 0);
        self.next_rect += 1;
    }

    pub fn add_textured_rect(&mut self, rect: Rect, texture: &Texture, tint: Vec4f) {
        let bounds = rect.bounds();

        let i = self.next_rect * 4;
        let slot = self.next_texture_slot as i32;
        self.vertices[i + 0] = RectVertex::new(
            Vec3f::new(bounds.0, bounds.3, rect.position.z), rect.uv_min, tint, slot);
        self.vertices[i + 1] = RectVertex::new(
            Vec3f::new(bounds.1, bounds.3, rect.position.z), Vec2f::new(rect.uv_max.x, rect.uv_min.y), tint, slot);
        self.vertices[i + 2] = RectVertex::new(
            Vec3f::new(bounds.1, bounds.2, rect.position.z), rect.uv_max, tint, self.next_texture_slot as i32);
        self.vertices[i + 3] = RectVertex::new(
            Vec3f::new(bounds.0, bounds.2, rect.position.z), Vec2f::new(rect.uv_min.x, rect.uv_max.y), tint, slot);
        self.next_rect += 1;
        
        texture.bind_to_slot(self.next_texture_slot as u32);
        self.next_texture_slot += 1;
    }
}

/// Renderer for 2D graphics
pub struct Renderer2D {
    default_shader: Shader,
    default_texture: Texture,
    rect_batch: RectBatch
}

impl Renderer2D {
    /// Creates new `Renderer2D`
    /// 
    /// # Arguments
    /// 
    /// * `view_projection` - The view projection matrix to use
    pub fn new(view_projection: Mat4f) -> Self {
        // Initialize default shader
        const VERTEX_SHADER: &str = r#"#version 330 core
        layout (location = 0) in vec3 v_in_position;
        layout (location = 1) in vec2 v_in_uv;
        layout (location = 2) in vec4 v_in_color;
        layout (location = 3) in int v_in_tex_slot;
    
        out vec2 v_out_uv;
        out vec4 v_out_color;
        flat out int v_out_tex_slot;

        uniform mat4 u_view_projection;
    
        void main() {
            v_out_uv = v_in_uv;
            v_out_color = v_in_color;
            v_out_tex_slot = v_in_tex_slot;
            gl_Position = u_view_projection * vec4(v_in_position, 1.0);
        }
        "#;
    
        const FRAGMENT_SHADER: &str = r#"#version 330 core
        in vec2 v_out_uv;
        in vec4 v_out_color; 
        flat in int v_out_tex_slot;

        out vec4 f_out_color;

        uniform sampler2D u_textures[32];
    
        void main() {
            f_out_color = texture(u_textures[v_out_tex_slot], v_out_uv) * v_out_color;
        }
        "#;
    
        let default_shader = Shader::new(
            VERTEX_SHADER, 
            FRAGMENT_SHADER
        );
        default_shader.bind();
        default_shader.set_mat4f(&CString::new("u_view_projection").unwrap(), view_projection);
        let texture_slots: Vec<i32> = (0..MAX_TEXTURE_SLOTS as i32).collect();
        default_shader.set_int_array(&CString::new("u_textures").unwrap(), &texture_slots);

        // Initialize default texture
        let default_texture = Texture::with_data(&Vec::from([255, 255, 255, 255]), 1, 1);

        Renderer2D {
            default_shader,
            default_texture,
            rect_batch: RectBatch::new()
        }
    }

    pub fn begin_batch(&mut self) {
        self.rect_batch.reset();
        self.default_texture.bind_to_slot(0);
    }

    pub fn end_batch(&self) {
        self.default_shader.bind();
        self.rect_batch.draw();
    }

    /// Draw a rect
    /// 
    /// # Arguments
    /// 
    /// * `rect` - The rect to draw
    /// * `color` - The color to draw with
    pub fn draw_rect(&self, rect: Rect, color: Vec4f) {
        self.draw_textured_rect(rect, &self.default_texture, color);
    }

    /// Draw a textured rect
    /// 
    /// # Arguments
    /// 
    /// * `rect` - The rect to draw
    /// * `texture` - The texture to draw
    /// * `tint` - The color to tint the texture
    pub fn draw_textured_rect(&self, rect: Rect, texture: &Texture, tint: Vec4f) {
        let vertex_array = VertexArray::new();
        
        let bounds = rect.bounds();
        let vertices = [
            RectVertex::new(Vec3f::new(bounds.0, bounds.3, rect.position.z), rect.uv_min, tint, 0),
            RectVertex::new(Vec3f::new(bounds.1, bounds.3, rect.position.z), Vec2f::new(rect.uv_max.x, rect.uv_min.y), tint, 0),
            RectVertex::new(Vec3f::new(bounds.1, bounds.2, rect.position.z), rect.uv_max, tint, 0),
            RectVertex::new(Vec3f::new(bounds.0, bounds.2, rect.position.z), Vec2f::new(rect.uv_min.x, rect.uv_max.y), tint, 0)
        ];

        let vertex_layout = BufferLayout::new(Vec::from([
            BufferAttribute::new(AttributeType::Vec3f, false),
            BufferAttribute::new(AttributeType::Vec2f, false),
            BufferAttribute::new(AttributeType::Vec4f, false),
            BufferAttribute::new(AttributeType::Int, false)
        ]));
        let vertex_buffer = ArrayBuffer::new_static(
            vertex_layout,
            vertices.as_ptr().cast(),
            size_of_val(&vertices)
        );
        
        const INDICES: [u32; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = IndexBuffer::new();
        index_buffer.set_data(INDICES.as_ptr().cast(), size_of_val(&INDICES));
        
        vertex_array.add_vertex_buffer(&vertex_buffer);
        vertex_array.set_index_buffer(&index_buffer);
        
        self.default_shader.bind();
        texture.bind_to_slot(0);
        Renderer::draw_elements(&vertex_array, 6);
    }

    pub fn batch_rect(&mut self, rect: Rect, color: Vec4f) {
        self.rect_batch.add_rect(rect, color);
    }

    pub fn batch_textured_rect(&mut self, rect: Rect, texture: &Texture, tint: Vec4f) {
        self.rect_batch.add_textured_rect(rect, texture, tint);
    }
}