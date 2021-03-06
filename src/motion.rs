use crate::scene::node::{Part, Robot};
use kiss3d::event::Key;
use na::{Translation3, UnitQuaternion};

// Rotates model relative to the axis in origin point
pub fn rotate_joint(joint: usize, angle: f32, robot: &mut Robot) {
    if (angle > 0.0) & (robot.joints[joint].angle.1 < robot.joints[joint].angle.2) {
        let rot = UnitQuaternion::from_axis_angle(&robot.joints[joint].axis, angle);

        robot.joints[joint].node.append_rotation_wrt_center(&rot);
        robot.joints[joint].angle.1 += (rot.angle().to_degrees()).round();
    } else if (angle < 0.0) && (robot.joints[joint].angle.1 > robot.joints[joint].angle.0) {
        let rot = UnitQuaternion::from_axis_angle(&robot.joints[joint].axis, angle);

        robot.joints[joint].node.append_rotation_wrt_center(&rot);
        robot.joints[joint].angle.1 -= (rot.angle().to_degrees()).round();
    }
}

#[allow(unused)]
pub fn move_base(input: Key, robot: &mut Part) {
    match input {
        Key::A => robot
            .node
            .append_translation(&Translation3::new(-1.0, 0.0, 0.0)),
        Key::D => robot
            .node
            .append_translation(&Translation3::new(1.0, 0.0, 0.0)),
        Key::W => robot
            .node
            .append_translation(&Translation3::new(0.0, 0.0, 1.0)),
        Key::S => robot
            .node
            .append_translation(&Translation3::new(0.0, 0.0, -1.0)),
        Key::E => robot
            .node
            .append_translation(&Translation3::new(0.0, 1.0, 0.0)),
        Key::Q => robot
            .node
            .append_translation(&Translation3::new(0.0, -1.0, 0.0)),
        _ => {}
    }
}
