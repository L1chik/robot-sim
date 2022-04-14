// use std::path::Path;
// use kiss3d::text::Font;
// use kiss3d::window::Window;
// use nalgebra::{Point2, Point3};
// use node::Robot;
//
// pub fn draw_info(window: &mut Window, robot: &mut Robot) {
//     let font = Font::new(&Path::new("src/assets/Robot.ttf")).unwrap();
//     let mut offset = 0.0;
//     let active_color = Point3::new(0.176, 0.898, 0.475);
//     let default_color = Point3::new(1.0, 0.659, 0.392);
//
//     let active = robot.get_active_joint();
//
//     for (id, val) in robot.joints.iter().enumerate() {
//         if id == active {
//             window.draw_text(format!("Angle: {:?}; ({:?}, {:?})",
//                                      val.angle.1, val.angle.0, val.angle.2).as_str(),
//                                  &Point2::new(3830.0, 15.0 + offset),
//                                  90.0,
//                                  &font,
//                                  &active_color
//                 );
//
//                 offset += 100.0;
//
//         } else {
//             window.draw_text(format!("Angle: {:?}; ({:?}, {:?})",
//                                      val.angle.1, val.angle.0, val.angle.2).as_str(),
//                                  &Point2::new(3830.0, 15.0 + offset),
//                                  90.0,
//                                  &font,
//                                  &default_color,
//                 );
//
//                 offset += 100.0;
//         }
//     }
// }