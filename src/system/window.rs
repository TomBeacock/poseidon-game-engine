use sdl2::{Sdl, video::{GLContext, GLProfile, SwapInterval}};

pub struct Window {
    window: sdl2::video::Window,
    gl_context: GLContext
}

impl Window {
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

    pub fn native(&self) -> &sdl2::video::Window {
        &self.window
    }
}