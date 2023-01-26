extern crate sdl2;
extern crate gl;

mod math;
mod graphics;

use std::mem::size_of_val;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::{GLProfile, SwapInterval};

use crate::math::vec3f::Vec3f;
use crate::math::mat4f::Mat4f;
use crate::graphics::vertex_array::VertexArray;
use crate::graphics::array_buffer::{ArrayBuffer, BufferLayout, BufferAttribute, AttributeType};
use crate::graphics::index_buffer::IndexBuffer;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_version(3, 3);
    gl_attr.set_context_profile(GLProfile::Core);
     
    let window = video_subsystem.window("Poseidon Engine", 1280, 720)
        .opengl()
        .build()
        .unwrap();
        
    let gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&gl_context).unwrap();
    video_subsystem.gl_set_swap_interval(SwapInterval::VSync).unwrap();

    gl::load_with(|fn_name| video_subsystem.gl_get_proc_address(fn_name) as *const _);

    unsafe {
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }

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
    let shader_program: u32;
    unsafe {
        // Vertex shader
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        assert_ne!(vertex_shader, 0);

        const VERT_SHADER: &str = r#"#version 330 core
        layout (location = 0) in vec3 pos;

        uniform mat4 model;
        uniform mat4 view_projection;

        void main() {
            gl_Position = view_projection * model * vec4(pos, 1.0);
        }
        "#;

        gl::ShaderSource(
            vertex_shader,
            1,
            &(VERT_SHADER.as_bytes().as_ptr().cast()),
            &(VERT_SHADER.len().try_into().unwrap())
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
            panic!("Vertex Shader Compile Error: {}", String::from_utf8_lossy(&v));
        }

        // Fragment shader
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        assert_ne!(fragment_shader, 0);

        const FRAG_SHADER: &str = r#"#version 330 core
        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
        "#;

        gl::ShaderSource(
            fragment_shader,
            1,
            &(FRAG_SHADER.as_bytes().as_ptr().cast()),
            &(FRAG_SHADER.len().try_into().unwrap())
        );
        gl::CompileShader(fragment_shader);

        let mut success = 0;
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0;
            gl::GetShaderInfoLog(
                fragment_shader,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Fragment Shader Compile Error: {}", String::from_utf8_lossy(&v));
        }

        // Program
        shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        let mut success = 0;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetProgramInfoLog(
                shader_program,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        gl::UseProgram(shader_program);

        // Set uniforms
        let model = Mat4f::transformation(
            Vec3f::new(0.0, 0.0, 0.0), 
            Vec3f::new(0.0, 0.0, 0.0),
            Vec3f::new(1.0, 1.0, 1.0));
        let view = Mat4f::translate(-Vec3f::new(0.0, 0.0, -3.0));
        //let projection = Mat4f::ortho(16.0, 9.0, 0.1, 10.0);
        let projection = Mat4f::persp_fov(f32::to_radians(90.0), 16.0 / 9.0, 0.1, 10.0);

        let name_model = std::ffi::CString::new("model").unwrap();
        let uniform_model = gl::GetUniformLocation(
            shader_program, 
            name_model.as_ptr()
        );

        gl::UniformMatrix4fv(
            uniform_model, 
            1, 
            gl::FALSE, 
            model.values.as_ptr()
        );
        
        let name_view_projection = std::ffi::CString::new("view_projection").unwrap();
        let uniform_view_projection = gl::GetUniformLocation(
            shader_program, 
            name_view_projection.as_ptr()
        );

        gl::UniformMatrix4fv(
            uniform_view_projection, 
            1, 
            gl::FALSE, 
            (projection * view).values.as_ptr()
        );
    }

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut angle: f32 = 0.0;

    'running: loop {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        // Rotate quad
        angle += 1.0;
        unsafe {
            // Set uniforms
            let model = Mat4f::transformation(
                Vec3f::new(0.0, 0.0, 0.0), 
                Vec3f::new(0.0, angle.to_radians(), 0.0),
                Vec3f::new(1.0, 1.0, 1.0));

            //println!("{}: {}", angle, model * Vec4f::new(1.0, 0.0, 0.0, 1.0));

            let name_model = std::ffi::CString::new("model").unwrap();
            let uniform_model = gl::GetUniformLocation(
                shader_program, 
                name_model.as_ptr()
            );

            gl::UniformMatrix4fv(
                uniform_model, 
                1, 
                gl::FALSE, 
                model.values.as_ptr()
            );
        }

        unsafe {
            gl::DrawElements(
                gl::TRIANGLES, 
                6*6, 
                gl::UNSIGNED_INT, 
                0 as *const _
            );
        }

        window.gl_swap_window();
    }
}
