use kiss3d::window::Window;
use na::{Point3, Vector3};
use ncollide3d::query::{Ray, RayIntersection};
use ncollide3d::shape::FeatureId;

pub fn draw_grid(window: &mut Window, spacing: f32, slices: u32) {
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

pub fn draw_ray(ray: Ray<f32>, window: &mut Window) {
    let min_intersection =
            RayIntersection::new(1.0, Vector3::new(1.0, 1.0, 1.0), FeatureId::Unknown);

    let p1 = ray.origin;
    let p2 = ray.point_at(min_intersection.toi + 20.0);

    // println!("p1: {:?}, p2: {:?}", &p1, &p2);
    // Drawing ray
    window.draw_line(&p1, &p2, &Point3::new(1.0, 1.0, 1.0));
}