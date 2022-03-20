use std::path::Path;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use na::Vector3;

pub struct Robot {
    pub base: SceneNode,
    pub shoulder: SceneNode,
    // upper_arm: SceneNode,
    // elbow: SceneNode,
    // lower_arm: SceneNode,
    // flange: SceneNode,
}

struct RobotParts<'a> {
    base_obj: &'a Path,
    base_mtl: &'a Path,
    shoulder_obj: &'a Path,
    shoulder_mtl: &'a Path,
    elbow_obj: &'a Path,
    elbow_mtl: &'a Path,
}



pub fn robot_init(window: &mut Window) {
    let robot = RobotParts {
        base_obj: Path::new("/home/l1chik/Rust/robot-sim/bluster/src/assets/base.obj"),
        base_mtl: Path::new("/home/l1chik/Rust/robot-sim/bluster/src/assets/base.mtl"),
        shoulder_obj: Path::new("/home/l1chik/Rust/robot-sim/bluster/src/assets/shoulder.obj"),
        shoulder_mtl: Path::new("/home/l1chik/Rust/robot-sim/bluster/src/assets/shoulder.mtl"),
        elbow_obj: Path::new("/home/l1chik/Rust/robot-sim/bluster/src/assets/elbow.obj"),
        elbow_mtl: Path::new("/home/l1chik/Rust/robot-sim/bluster/src/assets/elbow.mtl"),
    };

    let scale = Vector3::new(1.5, 1.5, 1.5);

    // Robot {
    //     base: SceneNode::add_obj(robot, base_obj, base_mtl, scale),
    //     shoulder: SceneNode::add_obj(robot, shoulder_obj, shoulder_mtl, scale),
    // };

    let mut base = window.add_obj(robot.base_obj, robot.base_mtl, scale);
    let mut shoulder = window.add_obj( robot.shoulder_obj, robot.shoulder_mtl, scale);
    let mut elbow = window.add_obj( robot.elbow_obj, robot.elbow_mtl, scale);
}