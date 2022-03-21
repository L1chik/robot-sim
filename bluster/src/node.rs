use std::path::Path;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use kiss3d::event::Key;
use na::Vector3;

pub struct Robot {
    pub base: SceneNode,
    pub shoulder: SceneNode,
    // lower_arm: SceneNode,
    elbow: SceneNode,
    upper_arm: SceneNode,
    // flange: SceneNode,
}

struct RobotParts<'a> {
    base_obj: &'a Path,
    base_mtl: &'a Path,
    shoulder_obj: &'a Path,
    shoulder_mtl: &'a Path,
    elbow_obj: &'a Path,
    elbow_mtl: &'a Path,
    upper_arm_obj: &'a Path,
    upper_arm_mtl: &'a Path,
}

pub fn robot_init(window: &mut Window) -> Robot {
    let parts = RobotParts {
        base_obj: Path::new("bluster/src/assets/base.obj"),
        base_mtl: Path::new("bluster/src/assets/base.mtl"),
        shoulder_obj: Path::new("bluster/src/assets/shoulder.obj"),
        shoulder_mtl: Path::new("bluster/src/assets/shoulder.mtl"),
        elbow_obj: Path::new("bluster/src/assets/elbow.obj"),
        elbow_mtl: Path::new("bluster/src/assets/elbow.mtl"),
        upper_arm_obj: Path::new("bluster/src/assets/upper_arm.obj"),
        upper_arm_mtl: Path::new("bluster/src/assets/upper_arm.mtl")
    };

    let scale = Vector3::new(1.5, 1.5, 1.5);

    let mut base = window.add_obj(parts.base_obj, parts.base_mtl, scale);
    let mut shoulder = window.add_obj( parts.shoulder_obj, parts.shoulder_mtl, scale);
    let mut elbow = window.add_obj( parts.elbow_obj, parts.elbow_mtl, scale);
    let mut upper_arm = window.add_obj(parts.upper_arm_obj, parts.upper_arm_mtl, scale);

    Robot {
        base,
        shoulder,
        elbow,
        upper_arm,
    }
}

// impl Robot {
//     pub fn active(&self, input: Key, ) -> SceneNode {
//         match input {
//             Key::Key1 => {
//                 println!("active is base");
//                 self.base.clone()
//             },
//             Key::Key2 => {
//                 println!("active is shoulder");
//                 self.shoulder.clone()
//             },
//             _ => {self.shoulder.clone()}
//         }
//     }
// }