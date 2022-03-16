extern crate kiss3d;

use kiss3d::window::Window;
use kiss3d::light::Light;


fn main() {
    let mut window = Window::new_with_size("Robot simulator", 1920, 1080);

    window.set_light(Light::StickToCamera)

}

fn window_init() {

}
