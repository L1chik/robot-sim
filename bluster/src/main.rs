mod gizmo;
mod node;

extern crate kiss3d;
extern crate nalgebra as na;


use std::path::Path;
use kiss3d::window::Window;
use kiss3d::event::{Action, Key, MouseButton, WindowEvent};
use kiss3d::light::Light;
use kiss3d::camera::{ArcBall, Camera};
use na::{Isometry3, Point3, Point2, Translation3, UnitQuaternion, Vector3, Vector2};
use ncollide3d::pipeline::CollisionGroups;
use ncollide3d::query::{Ray, RayIntersection};
use ncollide3d::shape::{FeatureId};
use crate::node::Robot;


fn draw_ray(ray: Ray<f32>, window: &mut Window) {
    let min_intersection =
            RayIntersection::new(1.0, Vector3::new(1.0, 1.0, 1.0), FeatureId::Unknown);

    let p1 = ray.origin;
    let p2 = ray.point_at(min_intersection.toi + 20.0);

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

    // Comment on Mac
    // arc_ball.rebind_rotate_button(Some(MouseButton::Button3));

    // Ray settings
    let mut last_pos = Point2::new(0.0, 0.0);
    let sel_pos = Vector3::new(0.0, 0.0, 0.0);
    let mut ray = Ray::new(
        Point3::new(0.0, 0.0, 0.0),
        sel_pos);

    // Control point
    // let mut control_point = window.add_sphere(0.5);
    // control_point.set_color(1.0, 0.522, 0.0);

    // Robot mesh
    let robot = node::robot_init(&mut window);
    let mut active = robot.base.clone();



    // TODO: object picking with raycast. Impl RayCast for SceneNode???
    // let mut cube = Cuboid::new(Vector3::new(1.0f32, 1.0, 1.0));

    // Scene render
    while window.render_with_camera(&mut arc_ball) {
        let _ = draw_grid(&mut window, 1.0, 30);

        // robot.prepend_to_local_rotation(&rot);
        draw_ray(ray, &mut window);

        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::FramebufferSize(x, y) => {
                    println!("frame buffer size event {}, {}", x, y);
                },
                WindowEvent::MouseButton(MouseButton::Button1, Action::Press, _modif) => {
                    let window_size = Vector2::new(
                        window.size()[0] as f32,
                        window.size()[1] as f32);

                    let (pos, dir) = arc_ball
                        .unproject(&last_pos, &window_size);

                    ray = Ray::new(pos, dir);

                    // ray.distance_to_point()
                    // println!("{:?}", cube.intersects_ray( &Isometry3::identity(), &ray, std::f32::MAX));
                },
                WindowEvent::CursorPos(x, y, _modif) => {
                    last_pos = na::Point2::new(x as f32, y as f32);
                },

                WindowEvent::Key(input, Action::Press, _modif) => {
                    match input{
                        Key::Key1 => {
                            println!("active is base");
                            active = robot.base.clone()
                        },
                        Key::Key2 => {
                            println!("active is shoulder");
                            active = robot.shoulder.clone()
                        }
                        Key::Key3 => {
                            active = robot.elbow.clone()
                        }
                        _ => {}
                    }
                    // active = robot.active(input, &mut active);
                    gizmo::loc_trans(input, &mut active);
                    gizmo::loc_rot(input, &mut active);
                }
                _ => {}
            }
        }


    }
}
