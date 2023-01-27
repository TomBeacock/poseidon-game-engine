use std::ffi::CString;
use std::mem::size_of_val;

use sdl2::Sdl;
use sdl2::event::Event;

use super::window::Window;

use crate::math::vec3f::Vec3f;
use crate::math::vec4f::Vec4f;
use crate::math::mat4f::Mat4f;

use crate::graphics::vertex_array::VertexArray;
use crate::graphics::array_buffer::{ArrayBuffer, BufferLayout, BufferAttribute, AttributeType};
use crate::graphics::index_buffer::IndexBuffer;
use crate::graphics::shader::Shader;
use crate::graphics::renderer::Renderer;

/// Main application
pub struct Application {
    sdl: Sdl,
    window: Window
}

impl Application {
    /// Creates a new `Application`
    pub fn new() -> Self {
        let sdl = sdl2::init().unwrap();
        let window = Window::new(&sdl);
    
        Renderer::init();
        Renderer::set_clear_color(Vec4f::new(0.0, 0.0, 0.0, 0.0));
        
        Application { sdl, window }
    }

    /// Start executing the application
    pub fn execute(&mut self) {
        // Vertex Buffer
        type Vertex = [f32; 3];
        const VERTICES: [Vertex; 4*6] = [
            [-0.5, -0.5, -0.5], [0.5, -0.5, -0.5] , [0.5, 0.5, -0.5]  , [-0.5, 0.5, -0.5],
            [0.5, -0.5, -0.5] , [0.5, -0.5, 0.5]  , [0.5, 0.5, 0.5]   , [0.5, 0.5, -0.5] ,
            [0.5, -0.5, 0.5]  , [-0.5, -0.5, 0.5] , [-0.5, 0.5, 0.5]  , [0.5, 0.5, 0.5]  ,
            [-0.5, -0.5, 0.5] , [-0.5, -0.5, -0.5], [-0.5, 0.5, -0.5] , [-0.5, 0.5, 0.5] ,
            [-0.5, 0.5, -0.5] , [0.5, 0.5, -0.5]  , [0.5, 0.5, 0.5]   , [-0.5, 0.5, 0.5] ,
            [0.5, -0.5, 0.5]  , [-0.5, -0.5, 0.5] , [-0.5, -0.5, -0.5], [0.5, -0.5, -0.5]];
            
            
        let vertex_layout = BufferLayout::new(Vec::from([
            BufferAttribute::new(AttributeType::Vec3f, false)
            ]));
        let vertex_buffer = ArrayBuffer::new(vertex_layout);
        vertex_buffer.set_data(VERTICES.as_ptr().cast(), size_of_val(&VERTICES));
        
        // Index Buffer
        const INDICES: [u32; 6*6] = [
            0, 1, 2, 0, 2, 3,
            4, 5, 6, 4, 6, 7,
            8, 9, 10, 8, 10, 11,
            12, 13, 14, 12, 14, 15,
            16, 17, 18, 16, 18, 19,
            20, 21, 22, 20, 22, 23];
        
        let index_buffer = IndexBuffer::new();
        index_buffer.set_data(INDICES.as_ptr().cast(), size_of_val(&INDICES));
        
        // Vertex Array
        let vertex_array = VertexArray::new();
        vertex_array.add_vertex_buffer(&vertex_buffer);
        vertex_array.set_index_buffer(&index_buffer);
    
        // Create shader
        const VERTEX_SHADER: &str = r#"#version 330 core
        layout (location = 0) in vec3 pos;
    
        uniform mat4 model;
        uniform mat4 view_projection;
    
        void main() {
            gl_Position = view_projection * model * vec4(pos, 1.0);
        }
        "#;
    
        const FRAGMENT_SHADER: &str = r#"#version 330 core
        out vec4 color;
    
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
        "#;
    
        let shader = Shader::new(
            VERTEX_SHADER, 
            FRAGMENT_SHADER
        );
        shader.bind();
    
        // Set uniforms
        let model = Mat4f::transformation(
            Vec3f::new(0.0, 0.0, 0.0), 
            Vec3f::new(0.0, 0.0, 0.0),
            Vec3f::new(1.0, 1.0, 1.0));
        let view = Mat4f::translate(-Vec3f::new(0.0, 0.0, -3.0));
        //let projection = Mat4f::ortho(16.0, 9.0, 0.1, 10.0);
        let projection = Mat4f::persp_fov(f32::to_radians(90.0), 16.0 / 9.0, 0.1, 10.0);
    
        shader.set_mat4f(&CString::new("model").unwrap(), model);
        shader.set_mat4f(&CString::new("view_projection").unwrap(), projection * view);

        let mut event_pump = self.sdl.event_pump().unwrap();
    
        let mut angle: f32 = 0.0;
    
        'running: loop {
            Renderer::clear();
    
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        break 'running
                    },
                    _ => {}
                }
            }
    
            // Rotate quad
            angle += 1.0;
    
            let model = Mat4f::transformation(
                Vec3f::new(0.0, 0.0, 0.0), 
                Vec3f::new(0.0, angle.to_radians(), 0.0),
                Vec3f::new(1.0, 1.0, 1.0));
    
            shader.set_mat4f(&CString::new("model").unwrap(), model);
    
            Renderer::draw_elements(&vertex_array, 6*6);
    
            self.window.native().gl_swap_window();
        }
    }
}
