use std::io::{BufRead, BufReader, ErrorKind};
use std::path::Path;
use std::thread;
use std::time::Duration;
use kiss3d::camera::ArcBall;
use kiss3d::light::Light;
use kiss3d::window::Window;
use nalgebra::{Point3, Vector3};
use crate::vrglove::encoder::{init_port, string_to_val, serial_value, serial_read};
use crate::vrglove::finger::Phalanx;
use crate::scene::grid::draw_grid;
use crate::vrglove::arm::{VrGlove, arm_init};

mod vrglove;
mod scene;

fn main() {
    // Window settings
    let mut window = Window::new_with_size(
        "Robot simulator", 1280, 720);
    window.set_light(Light::StickToCamera);
    window.set_background_color(0.192, 0.192, 0.192);

    // Camera settings
    let eye = Point3::new(2.0, 2.0, 2.0);
    let at = Point3::origin();
    let mut arc_ball = ArcBall::new(eye, at);

    //Port initialisation
    // let mut port = init_port("/dev/ttyACM0", 4800);
    // let mut reader = BufReader::new(port);
    // let mut val = 0;

    //VR glove settings
    let (mut vrglove, mut main_arm) = arm_init(&mut window);
    // let mut buff= String::new();


    while window.render_with_camera(&mut arc_ball) {
        draw_grid(&mut window, 1.0, 30);

        // serial_read(&mut reader, &mut vrglove, &mut main_arm);
    }
}