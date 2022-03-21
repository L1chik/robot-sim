use kiss3d::event::Key;
use kiss3d::scene::SceneNode;
use na::{Translation3, UnitQuaternion, Vector3};
use crate::node::Robot;

pub fn loc_rot(input: Key, part: &mut SceneNode) {
    match input {
        Key::H => part.append_rotation(&UnitQuaternion::new(
        Vector3::new(-0.2, 0.0, 0.0))),
        Key::K => part.append_rotation(&UnitQuaternion::new(
        Vector3::new(0.2, 0.0, 0.0))),
        Key::J => part.append_rotation(&UnitQuaternion::new(
        Vector3::new(0.0, -0.2, 0.0))),
        Key::U => part.append_rotation(&UnitQuaternion::new(
        Vector3::new(0.0, 0.2, 0.0))),
        Key::I => part.append_rotation(&UnitQuaternion::new(
        Vector3::new(0.0, 0.0, 0.2))),
        Key::Y => part.append_rotation(&UnitQuaternion::new(
        Vector3::new(0.0, 0.0, -0.2))),
        _ => {}
    }
}

pub fn loc_trans(input: Key, robot: &mut SceneNode) {
    match input {
        Key::A => robot.append_translation(&Translation3::new(-1.0, 0.0, 0.0)),
        Key::D => robot.append_translation(&Translation3::new(1.0, 0.0, 0.0)),
        Key::W => robot.append_translation(&Translation3::new(0.0, 0.0, 1.0)),
        Key::S => robot.append_translation(&Translation3::new(0.0, 0.0, -1.0)),
        Key::E => robot.append_translation(&Translation3::new(0.0, 1.0, 0.0)),
        Key::Q => robot.append_translation(&Translation3::new(0.0, -1.0, 0.0)),
        _ => {}
    }
}






// use na::Matrix4;
//
// pub struct Gizmo {
//     config: GizmoConfig,
//
// }
//
// pub struct GizmoConfig {
//     pub view_matrix: Matrix4::fill_
// }
