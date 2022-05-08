use crate::vrglove::arm::{arm_init, VrGlove};
use crate::vrglove::encoder::{init_port, serial_read};
use crate::vrglove::finger::Phalanx;
use kiss3d::camera::ArcBall;
use kiss3d::light::Light;
use kiss3d::window::Window;
use nalgebra::Point3;
use scene::grid::draw_grid;
use std::io::BufReader;

mod scene;
mod vrglove;

fn main() {
    // Window settings
    let mut window = Window::new_with_size("Robot simulator", 1280, 720);
    window.set_light(Light::StickToCamera);
    window.set_background_color(0.192, 0.192, 0.192);

    // Camera settings
    let eye = Point3::new(2.0, 2.0, 2.0);
    let at = Point3::origin();
    let mut arc_ball = ArcBall::new(eye, at);

    //Port initialisation
    let port = init_port("/dev/ttyACM0", 4800);
    let mut reader = BufReader::new(port);

    //VR glove settings
    let (mut vrglove, mut main_arm) = arm_init(&mut window);

    while window.render_with_camera(&mut arc_ball) {
        draw_grid(&mut window, 1.0, 30);

        // Read all data from sensors
        serial_read(&mut reader, &mut vrglove, &mut main_arm);
    }
}
