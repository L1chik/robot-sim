mod motion;
mod scene;
mod robot;
mod node;
mod gizmo;


extern crate kiss3d;
extern crate nalgebra as na;


use kiss3d::window::Window;
use kiss3d::event::{Action, Key, Modifiers, MouseButton, WindowEvent};
use kiss3d::light::Light;
use kiss3d::camera::{ArcBall, Camera};
use na::{Point3, Point2, Vector3, Translation2, Vector2};
use ncollide3d::query::{Ray};
use crate::motion::{rotate_joint};
use crate::node::{Robot, RPoint};
use crate::scene::grid::{draw_grid, draw_ray};
use crate::scene::stats::draw_info;


static CTRL: Modifiers = Modifiers::Control;


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

    // Comment on Mac
    arc_ball.rebind_rotate_button(Some(MouseButton::Button3));

    // Ray settings
    let mut last_pos = Point2::new(0.0, 0.0);
    let sel_pos = Vector3::new(0.0, 0.0, 0.0);
    let mut ray = Ray::new(
        Point3::new(0.0, 0.0, 0.0),
        sel_pos);

    // Robot mesh
    let robot = &mut robot::robot_init(&mut window);

    // InfoBar
    let mut tablet = window.add_rectangle(350.0, 400.0);
    tablet.set_local_translation(Translation2::new(480.0, 160.0));
    tablet.set_color(0.278, 0.278, 0.278);

    // Custom settings
    let mut r_point = RPoint::default();

    // TODO: object picking with raycast. Impl RayCast for SceneNode
    // let mut cube = Cuboid::new(Vector3::new(1.0f32, 1.0, 1.0));

    // Scene render
    while window.render_with_camera(&mut arc_ball) {

        // Initial drawings: grid, stats; *ray
        draw_grid(&mut window, 1.0, 30);
        draw_ray(ray, &mut window);
        draw_info(&mut window, robot);

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
                },

                WindowEvent::CursorPos(x, y, modif) => {
                    if modif.contains(CTRL) {
                        event.inhibited = true;
                        let move_gain = 0.005 as f32;
                        rotate_joint(robot.get_active_joint(),
                                     (x as f32 - last_pos.x) * move_gain,
                                     robot);
                    }

                    last_pos = na::Point2::new(x as f32, y as f32);
                },

                WindowEvent::Key(input, Action::Press, _modif) => {
                    match input{
                        Key::Key1 => {
                            robot.set_joint_index(0)
                        }
                        Key::Key2 => {
                            robot.set_joint_index(1)
                        }
                        Key::Key3 => {
                            robot.set_joint_index(2)
                        }
                        Key::Key4 => {
                            robot.set_joint_index(3)
                        }
                        Key::Key5 => {
                            robot.set_joint_index(4)
                        }
                        Key::Key6 => {
                            robot.set_joint_index(5)
                        }
                        Key::Key7 => {
                            robot.set_joint_index(6)
                        }
                        Key::J => {
                            rotate_joint(robot.get_active_joint(), -0.017, robot)
                        }
                        Key::U => {
                            rotate_joint(robot.get_active_joint(), 0.017, robot)
                        }
                        Key::P => {
                            r_point.save_point(&robot);
                        }
                        _ => {  }
                    }
                }
                _ => {}
            }
        }
    }
}
