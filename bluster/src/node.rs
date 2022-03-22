use std::path::Path;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use kiss3d::event::Key;
use na::{Vector3, OVector, Unit};
use ncollide3d::na::Translation3;

pub struct Robot {
    pub base: Part,
    pub shoulder: Part,
    pub lower_arm: Part,
    pub elbow: Part,
    pub upper_arm: Part,
    pub wrist: Part,
}

#[derive(Clone)]
pub struct Part {
    pub node: SceneNode,
    pub axis: Unit<Vector3<f32>>,
}

struct PartsPathes<'a> {
    base_obj: &'a Path,
    base_mtl: &'a Path,
    shoulder_obj: &'a Path,
    shoulder_mtl: &'a Path,
    lower_arm_obj: &'a Path,
    lower_arm_mtl: &'a Path,
    elbow_obj: &'a Path,
    elbow_mtl: &'a Path,
    upper_arm_obj: &'a Path,
    upper_arm_mtl: &'a Path,
    wrist_mtl: &'a Path,
    wrist_obj: &'a Path,
}

pub fn robot_init(window: &mut Window) -> Robot {
    let parts = PartsPathes {
        base_obj: Path::new("bluster/src/assets/base.obj"),
        base_mtl: Path::new("bluster/src/assets/base.mtl"),
        shoulder_obj: Path::new("bluster/src/assets/shoulder.obj"),
        shoulder_mtl: Path::new("bluster/src/assets/shoulder.mtl"),
        lower_arm_obj: Path::new("bluster/src/assets/lower_arm.obj"),
        lower_arm_mtl: Path::new("bluster/src/assets/lower_arm.mtl"),
        elbow_obj: Path::new("bluster/src/assets/elbow.obj"),
        elbow_mtl: Path::new("bluster/src/assets/elbow.mtl"),
        upper_arm_obj: Path::new("bluster/src/assets/upper_arm.obj"),
        upper_arm_mtl: Path::new("bluster/src/assets/upper_arm.mtl"),
        wrist_obj: Path::new("bluster/src/assets/wrist.obj"),
        wrist_mtl: Path::new("bluster/src/assets/wrist.mtl"),
    };

    let scale = Vector3::new(1.0, 1.0, 1.0);
    let mut base_model = window.add_obj(parts.base_obj, parts.base_mtl, scale);
    let mut shoulder_model = base_model.add_obj( parts.shoulder_obj, parts.shoulder_mtl, scale);
    let mut lower_arm_model = shoulder_model.add_obj(parts.lower_arm_obj, parts.lower_arm_mtl, scale);
    let mut elbow_model = lower_arm_model.add_obj( parts.elbow_obj, parts.elbow_mtl, scale);
    let mut upper_arm_model = elbow_model.add_obj(parts.upper_arm_obj, parts.upper_arm_obj, scale);
    let mut wrist_model = upper_arm_model.add_obj(parts.wrist_obj, parts.wrist_mtl, scale);

     let mut base = Part {
        node: base_model,
        axis: Vector3::y_axis(),
    };

    let mut shoulder = Part {
        node: shoulder_model,
        axis:Vector3::y_axis(),
    };

    let mut lower_arm = Part {
        node: lower_arm_model,
        axis: Vector3::x_axis(),
    };

    let mut elbow = Part {
        node: elbow_model,
        axis: Vector3::x_axis(),
    };

    let mut upper_arm = Part {
        node: upper_arm_model,
        axis: Vector3::x_axis(),
    };

    let mut wrist = Part {
        node: wrist_model,
        axis: Vector3::z_axis(),
    };

    shoulder.node.append_translation(&Translation3::new(0.0, 0.0, 0.0));
    lower_arm.node.append_translation(&Translation3::new(0.0, 0.8, 0.25));
    elbow.node.append_translation(&Translation3::new(0.0, 0.7, -1.47));
    upper_arm.node.append_translation(&Translation3::new(0.0, 0.9, 1.85));
    wrist.node.append_translation(&Translation3::new(0.0, -0.1, 0.32));

    Robot {
        base,
        shoulder,
        elbow,
        lower_arm,
        upper_arm,
        wrist,
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