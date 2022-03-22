use std::path::Path;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use kiss3d::event::Key;
use na::Vector3;
use ncollide3d::na::Translation3;

pub struct Robot {
    pub base: SceneNode,
    pub shoulder: SceneNode,
    // lower_arm: SceneNode,
    pub elbow: SceneNode,
    // pub upper_arm: SceneNode,
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
        base_obj: Path::new("bluster/src/assets/base1.obj"),
        base_mtl: Path::new("bluster/src/assets/base1.mtl"),
        shoulder_obj: Path::new("bluster/src/assets/base1.obj"),
        shoulder_mtl: Path::new("bluster/src/assets/base1.mtl"),
        elbow_obj: Path::new("bluster/src/assets/base1.obj"),
        elbow_mtl: Path::new("bluster/src/assets/base1.mtl"),
        upper_arm_obj: Path::new("bluster/src/assets/upper_arm.obj"),
        upper_arm_mtl: Path::new("bluster/src/assets/upper_arm.mtl")
    };

    let scale = Vector3::new(1.0, 1.0, 1.0);

    let mut base = window.add_obj(parts.base_obj, parts.base_mtl, scale);
    let mut shoulder = base.add_obj( parts.shoulder_obj, parts.shoulder_mtl, scale);
    let mut elbow = shoulder.add_obj( parts.elbow_obj, parts.elbow_mtl, scale);
    // let mut upper_arm = window.add_obj   (parts.upper_arm_obj, parts.upper_arm_mtl, scale);

    // let mut base = window.add_cube(1.0, 0.4, 0.4);
    // let mut base_plane = base.add_cube(0.01, 1.0, 1.0);
    // let mut shoulder = base_plane.add_cube(100.0, 1.0, 1.0);
    // let mut elbow = shoulder.add_cube(1.0, 0.4, 0.4);

    shoulder.set_color(1.0, 0.0, 0.0);
    elbow.set_color(1.0, 0.0, 0.0);

    shoulder.append_translation(&Translation3::new(-3.0, 0.0, 0.0));
    elbow.append_translation(&Translation3::new(-3.0, 0.0, 0.0));
    // shoulder.append_translation(&Translation3::new(1.0, 0.0, 0.0));
    // elbow.append_translation(&Translation3::new(1.0, 0.0, 0.0));


    Robot {
        base,
        shoulder,
        elbow,
        // upper_arm,
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