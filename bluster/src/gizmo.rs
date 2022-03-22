use kiss3d::event::Key;
use kiss3d::scene::SceneNode;
use na::{Translation3, UnitQuaternion, Vector3, Unit};
use crate::node::{Robot, Part};

pub fn loc_rot(input: Key, part: &mut Part) {

    match input {
        Key::J => part.node.append_rotation_wrt_center(&UnitQuaternion::from_axis_angle(
        &part.axis, -0.014)),

        Key::U => part.node.append_rotation_wrt_center(&UnitQuaternion::from_axis_angle(
        &part.axis, 0.014)),
        
        _ => {}
    }
}

pub fn loc_trans(input: Key, robot: &mut Part) {
    match input {
        Key::A => robot.node.append_translation(&Translation3::new(-1.0, 0.0, 0.0)),
        Key::D => robot.node.append_translation(&Translation3::new(1.0, 0.0, 0.0)),
        Key::W => robot.node.append_translation(&Translation3::new(0.0, 0.0, 1.0)),
        Key::S => robot.node.append_translation(&Translation3::new(0.0, 0.0, -1.0)),
        Key::E => robot.node.append_translation(&Translation3::new(0.0, 1.0, 0.0)),
        Key::Q => robot.node.append_translation(&Translation3::new(0.0, -1.0, 0.0)),
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
