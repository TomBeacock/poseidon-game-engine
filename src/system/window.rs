use sdl2::{Sdl, video::{GLContext, GLProfile, SwapInterval}};

/// Holds window information
pub struct Window {
    window: sdl2::video::Window,
    gl_context: GLContext
}

impl Window {
    /// Creates a new `Window`.\
    /// Initializes OpenGL context and functions
    /// 
    /// # Arguments
    /// 
    /// * `sdl` - Reference to sdl
    pub fn new(sdl: &Sdl) -> Self {
        let video = sdl.video().unwrap();
    
        let gl_attr = video.gl_attr();
        gl_attr.set_context_version(3, 3);
        gl_attr.set_context_profile(GLProfile::Core);
         
        let window = video.window("Poseidon Engine", 1280, 720)
            .opengl()
            .build()
            .unwrap();
            
        let gl_context = window.gl_create_context().unwrap();
        window.gl_make_current(&gl_context).unwrap();
        video.gl_set_swap_interval(SwapInterval::VSync).unwrap();
    
        gl::load_with(|fn_name| video.gl_get_proc_address(fn_name) as *const _);
        Window { window, gl_context }
    }

    /// Get the native SDL window
    pub fn native(&self) -> &sdl2::video::Window {
        &self.window
    }
}