use std::path::Path;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use kiss3d::event::Key;
use na::{Vector3, OVector, Unit};
use ncollide3d::na::Translation3;

pub struct Robot {
    pub base: Part,
    pub shoulder: Part,
    // lower_arm: SceneNode,

    pub elbow: Part,
    // pub upper_arm: SceneNode,
    // flange: SceneNode,
}

#[derive(Clone)]
pub struct Part {
    pub node: SceneNode,
    pub axis: Unit<Vector3<f32>>,
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
    let mut base_model = window.add_obj(parts.base_obj, parts.base_mtl, scale);
    let mut shoulder_model = base_model.add_obj( parts.shoulder_obj, parts.shoulder_mtl, scale);
    let mut elbow_model = shoulder_model.add_obj( parts.elbow_obj, parts.elbow_mtl, scale);

     let mut base = Part {
        node: base_model,
        axis: Vector3::y_axis(),
    };

    let mut shoulder = Part {
        node: shoulder_model,
        axis:Vector3::z_axis(),
    };

    let mut elbow = Part {
        node: elbow_model,
        axis: Vector3::x_axis(),
    };

    shoulder.node.set_color(1.0, 0.0, 0.0);
    elbow.node.set_color(1.0, 0.0, 0.0);

    shoulder.node.append_translation(&Translation3::new(-3.0, 0.0, 0.0));
    elbow.node.append_translation(&Translation3::new(-3.0, 0.0, 0.0));
    // shoulder.append_translation(&Translation3::new(1.0, 0.0, 0.0));
    // elbow.append_translation(&Translation3::new(1.0, 0.0, 0.0));


    Robot {
        base,
        shoulder,
        elbow,
        // upper_arm,
    }
}

// impl Part {
//     fn set_model(&mut self, model: SceneNode) {
//         self.node = model
//     }
//
//     fn set_axis(&mut self, axis: OVector<T, D>) {
//         self.axis = axis
//     }
// }

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