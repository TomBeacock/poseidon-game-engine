extern crate poseidon;
use poseidon::system::application::Application;

fn main() {
    let mut app = Application::new();
    app.execute();
}
