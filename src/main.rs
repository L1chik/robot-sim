extern crate nalgebra as na;

use kiss3d::camera::{ArcBall, Camera};
use kiss3d::event::{Action, MouseButton, WindowEvent};
use kiss3d::light::Light;
use kiss3d::window::Window;
use na::{Isometry3, Point2, Point3, Translation2, Vector2, Vector3};
use ncollide3d::query::{Ray, RayCast, RayIntersection};
use ncollide3d::shape::{Cuboid, TriMesh};


pub fn draw_ray(ray: Ray<f32>, window: &mut Window) {
    let p1 = ray.origin;
    let p2 = ray.point_at(20.0);

    // println!("p1: {:?}, p2: {:?}", &p1, &p2);
    // Drawing ray
    window.draw_line(&p1, &p2, &Point3::new(1.0, 1.0, 1.0));
}


fn main() {
    // Window setup
    let mut window = Window::new("Picking");
    window.set_light(Light::StickToCamera);
    window.set_background_color(0.192, 0.192, 0.192);

    //Camera
    let eye = Point3::new(2.0, 2.0, 2.0);
    let at = Point3::origin();
    let mut arc_ball = ArcBall::new(eye, at);
    arc_ball.rebind_drag_button(Option::from(MouseButton::Button3));

    //Object settings
    // let mut cube = window.add_cube(1.0, 1.0, 1.0);
    let cube = ncollide3d::procedural::cuboid(&Vector3::new(1.0f32, 1.0, 1.0));
    window.add_trimesh(cube, Vector3::new(1.0, 1.0, 1.0));
    let mut ray = Ray::new(Point3::new(0.0f32, 0.0, -1.0), Vector3::z());

    // window.add
    assert!(cube.intersects_ray(&Isometry3::identity(), &ray, std::f32::MAX));
    //Ray settings
    let mut last_pos = Point2::new(0.0, 0.0);
    let sel_pos = Vector3::new(0.0, 0.0, 0.0);
    // let mut ray = Ray::new(Point3::new(0.0, 0.0, 0.0), sel_pos);

    while window.render_with_camera(&mut arc_ball) {
        draw_ray(ray, &mut window);

        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::FramebufferSize(x, y) => {
                    println!("frame buffer size event {}, {}", x, y);
                }

                WindowEvent::MouseButton(MouseButton::Button1, Action::Press, _modif) => {
                    let window_size =
                        Vector2::new(window.size()[0] as f32, window.size()[1] as f32);

                    let (pos, dir) = arc_ball.unproject(&last_pos, &window_size);

                    ray = Ray::new(pos, dir);
                }

                WindowEvent::CursorPos(x, y, modif) => {
                    last_pos = na::Point2::new(x as f32, y as f32);
                }
                _ => {}
            }
        }
    }
}
