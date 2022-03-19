extern crate kiss3d;
extern crate nalgebra as na;


use std::path::Path;
use kiss3d::window::Window;
use kiss3d::event::{Action, MouseButton, TouchAction, WindowEvent};
use kiss3d::light::Light;
use kiss3d::camera::{ArcBall, Camera};
use na::{Isometry3, OVector, Point3, Point2, Translation3, UnitQuaternion, Vector3, OPoint, Vector2};
use ncollide3d::pipeline::CollisionGroups;
use ncollide3d::query::{Ray, RayIntersection};
use ncollide3d::shape::FeatureId;
use ncollide3d::world::CollisionWorld;


fn draw_ray(ray: Ray<f32>, window: &mut Window) {
    let groups = CollisionGroups::new();
    let mut min_intersection =
            RayIntersection::new(0.0, Vector3::new(0.0, 0.0, 0.0), FeatureId::Unknown);

    let p1 = ray.origin;
    let p2 = ray.point_at(20.0);

    // println!("p1: {:?}, p2: {:?}", &p1, &p2);
    // Drawing ray
    window.draw_line(&p1, &p2, &Point3::new(1.0, 1.0, 1.0));
}

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
    // let world = CollisionWorld::new(0);

    // Camera settings
    let eye = Point3::new(2.0, 2.0, 2.0);
    let at = Point3::origin();
    let mut arc_ball = ArcBall::new(eye, at);
    arc_ball.rebind_rotate_button(Some(MouseButton::Button3));
    let mut last_pos = Point2::new(0.0, 0.0);
    let mut sel_pos = Vector3::new(0.0, 0.0, 0.0);
    let mut ray = Ray::new(
        Point3::new(0.0, 0.0, 0.0),
        sel_pos);

    // Control point
    let mut control_point = window.add_sphere(0.5);
    control_point.set_color(1.0, 0.522, 0.0);


    // Robot mesh
    let robot_path = Path::new("/home/l1chik/Blender/monkey2.obj");
    let robot_mtl = Path::new("/home/l1chik/Blender/monkey2.mtl");
    let robot_scale = Vector3::new(1.0, 1.0, 1.0);
    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.1);

    let mut robot  = window.add_obj(robot_path, robot_mtl, robot_scale);

    // Scene render
    while window.render_with_camera(&mut arc_ball) {
        let _ = draw_grid(&mut window, 1.0, 30);

        robot.prepend_to_local_rotation(&rot);
        draw_ray(ray, &mut window);


        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::FramebufferSize(x, y) => {
                    println!("frame buffer size event {}, {}", x, y);
                }
                WindowEvent::MouseButton(MouseButton::Button1, Action::Press, _modif) => {
                    let window_size = Vector2::new(
                        window.size()[0] as f32,
                        window.size()[1] as f32);

                    let (pos, dir) = arc_ball.unproject(&last_pos, &window_size);
                    
                    ray = Ray::new(pos, dir);
                    robot.set_color(1.0, 0.522, 0.0);
                }
                WindowEvent::CursorPos(x, y, _modif) => {
                    last_pos = na::Point2::new(x as f32, y as f32);
                }
                _ => {}
            }
        }


    }
}
