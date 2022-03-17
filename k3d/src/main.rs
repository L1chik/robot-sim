extern crate kiss3d;
extern crate nalgebra;

use std::ops::Add;
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::camera::ArcBall;
use kiss3d::scene::SceneNode;
use nalgebra::{Isometry3, OVector, Point3, Translation3, UnitQuaternion, Vector3, OPoint};


fn grid(window: &mut Window) {
    // let default_color = Point3::new(0.702, 0.800, 0.745);
    let default_color = Point3::new(0.318, 0.318, 0.318);
    let coord_plus_x = Point3::new(50.0, 0.0, 0.0);
    let coord_minus_x = Point3::new(-50.0, 0.0, 0.0);
    let coord_plus_z = Point3::new(0.0, 0.0, 50.0);
    let coord_minus_z = Point3::new(0.0, 0.0, -50.0);

    let x = window.draw_line(
        &Point3::new(-1000.0, 0.0, 0.0),
        &Point3::new(1000.0, 0.0, 0.0),
        &Point3::new(0.898, 0.266, 0.266)
    );

    let z = window.draw_line(
        &Point3::new(0.0, 0.0, -1000.0),
        &Point3::new(0.0, 0.0, 1000.0),
        &Point3::new(0.176, 0.537, 0.898)
    );

    // lines.push(x);
    // lines.push(z);

    for i in 1..51 {
        let offset_z = Vector3::new(0.0, 0.0, i as f32);
        let offset_x = Vector3::new(i as f32, 0.0, 0.0);
        window.draw_line(&(coord_minus_z + offset_x), &(coord_plus_z + offset_x), &default_color);
        window.draw_line(&(coord_minus_z - offset_x), &(coord_plus_z - offset_x), &default_color);
        window.draw_line(&(coord_minus_x + offset_z), &(coord_plus_x + offset_z), &default_color);
        window.draw_line(&(coord_minus_x - offset_z), &(coord_plus_x - offset_z), &default_color);
    }
}

fn main() {
    // Window settings
    let mut window = Window::new_with_size(
        "Robot simulator", 1280, 720);
    window.set_light(Light::Absolute(Point3::new(5.0, 5.0, 5.0)));
    window.set_background_color(0.192, 0.192, 0.192);

    // Camera settings
    let eye = Point3::new(2.0, 2.0, 2.0);
    let at = Point3::origin();
    let mut arc_ball = ArcBall::new(eye, at);

    // Control point
    let mut control_point = window.add_sphere(0.5);
    control_point.set_color(1.0, 0.522, 0.0);

    // let _ = grid(&mut window);

    // Scene render
    while window.render_with_camera(&mut arc_ball) {
        let _ = grid(&mut window);
        // window.draw_line(&x_left, &x_right, &x_color);
        // window.draw_line(&z_left, &z_right, &z_color);
    }
}
