use kiss3d::event::Key;
use na::{Translation3, UnitQuaternion, Vector3};

pub fn loc_rot(input: Key) {
    match input {
        // Key::H => robot.append_rotation(&UnitQuaternion::new(
        // Vector3::new(1.0, 0.0, 0.0))),
        // Key::H => robot.append_rotation(&UnitQuaternion::new(
        // Vector3::new(-0.2, 0.0, 0.0))),
        // Key::K => robot.append_rotation(&UnitQuaternion::new(
        // Vector3::new(0.2, 0.0, 0.0))),
        // Key::J => robot.append_rotation(&UnitQuaternion::new(
        // Vector3::new(0.0, -0.2, 0.0))),
        // Key::U => robot.append_rotation(&UnitQuaternion::new(
        // Vector3::new(0.0, 0.2, 0.0))),
        // Key::I => robot.append_rotation(&UnitQuaternion::new(
        // Vector3::new(0.0, 0.0, 0.2))),
        // Key::Y => robot.append_rotation(&UnitQuaternion::new(
        // Vector3::new(0.0, 0.0, -0.2))),
        _ => {}
    }
}

pub fn loc_trans(input: Key) {
    match input {
        // Key::A => robot.append_translation(&Translation3::new(-1.0, 0.0, 0.0)),
        // Key::D => robot.append_translation(&Translation3::new(1.0, 0.0, 0.0)),
        // Key::W => robot.append_translation(&Translation3::new(0.0, 0.0, 1.0)),
        // Key::S => robot.append_translation(&Translation3::new(0.0, 0.0, -1.0)),
        // Key::E => robot.append_translation(&Translation3::new(0.0, 1.0, 0.0)),
        // Key::Q => robot.append_translation(&Translation3::new(0.0, -1.0, 0.0)),
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
