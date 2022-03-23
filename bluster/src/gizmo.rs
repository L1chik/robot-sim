use kiss3d::event::Key;
use kiss3d::scene::SceneNode;
use na::{Translation3, UnitQuaternion, Vector3, Unit};
use crate::node::{Robot, Part};

pub fn loc_rot(input: Key, part: &mut Part) {

    match input {
        Key::J => {
            if part.angle.0 != part.angle.1 {
                let rot = UnitQuaternion::from_axis_angle(
                &part.axis, -0.0174533);
                part.node.append_rotation_wrt_center(&rot);

                part.angle.1 -= (rot.angle() * 180.0 / std::f32::consts::PI).round();
                println!("current rotation: {:?}", part.angle);
            };
        }

        Key::U => {
            if part.angle.2 != part.angle.1 {
                let rot = UnitQuaternion::from_axis_angle(
                    &part.axis, 0.0174533);
                part.node.append_rotation_wrt_center(&rot);

                part.angle.1 += (rot.angle() * 180.0 / std::f32::consts::PI).round();
                println!("current rotation: {:?}", part.angle);
            };
        }

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

pub fn animation(part: &mut Part) {
    for i in 1..100 {
        part.node.prepend_to_local_rotation(
            &UnitQuaternion::from_axis_angle(&part.axis, 0.085));
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
