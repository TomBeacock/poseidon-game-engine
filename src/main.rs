extern crate sdl2;
extern crate gl;

mod system;
mod math;
mod graphics;

use crate::system::application::Application;

fn main() {
    let mut app = Application::new();
    app.execute();
}
