extern crate kiss3d;
extern crate nalgebra as na;


use std::path::Path;
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::camera::ArcBall;
use na::{Isometry3, OVector, Point3, Translation3, UnitQuaternion, Vector3, OPoint};


fn draw_grid(window: &mut Window, spacing: f32, slices: u32) {
    let half_slices = slices as i32 / 2;
    let default_color = &Point3::new(0.318, 0.318, 0.318);
    let x_color = &Point3::new(0.898, 0.266, 0.266);
    let z_color = &Point3::new(0.176, 0.537, 0.898);
    let range = -half_slices..=half_slices;

    range.into_iter()
        .enumerate()
        .map(|(_e, i)| if i != 0 {
            // Drawing default axes
            window.draw_line(&Point3::new(-(half_slices as f32), 0.0, spacing * i as f32),
                             &Point3::new(half_slices as f32, 0.0, spacing * i as f32),
                             default_color);
            window.draw_line(&Point3::new(spacing * i as f32, 0.0, -(half_slices as f32)),
                             &Point3::new(spacing * i as f32, 0.0, half_slices as f32),
                             default_color);
            } else {
            // Drawing main axes
                window.draw_line(&Point3::new(-(half_slices as f32), 0.0, 0.0),
                                 &Point3::new(half_slices as f32, 0.0, 0.0),
                                 x_color);
                window.draw_line(&Point3::new(0.0, 0.0, -(half_slices as f32)),
                                 &Point3::new(0.0, 0.0, half_slices as f32),
                                 z_color);
        }).collect::<Vec<_>>();
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

    // Robot mesh
    let robot_path = Path::new("/home/l1chik/Blender/monkey.obj");
    let robot_mtl = Path::new("/home/l1chik/Blender/monkey.mtl");
    let robot_scale = Vector3::new(1.0, 1.0, 1.0);
    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.1);

    let mut robot  = window.add_obj(robot_path, robot_mtl, robot_scale);

    // Scene render
    while window.render_with_camera(&mut arc_ball) {
        let _ = draw_grid(&mut window, 1.0, 30);

        robot.prepend_to_local_rotation(&rot);
    }
}
