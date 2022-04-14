use std::path::Path;
use kiss3d::window::Window;
use nalgebra::{Translation3, Vector3};
use crate::Phalanx;

pub struct ArmParts<'a> {
    main_arm_obj: &'a Path,
    index1_obj: &'a Path,
    index2_obj: &'a Path,
    index3_obj: &'a Path,
    mtl: &'a Path,
}

pub struct VrGlove {
    pub phalanges: Vec<Phalanx>,
}

pub fn arm_init(window: &mut Window) -> VrGlove {
    let parts = ArmParts {
        main_arm_obj: Path::new("assets/main_arm.obj"),
        index1_obj: Path::new("assets/index1.obj"),
        index2_obj: Path::new("assets/index2.obj"),
        index3_obj: Path::new("assets/index3.obj"),
        mtl: Path::new("assets/main_arm.mtl"),
    };

    let scale = Vector3::new(1.0, 1.0, 1.0);
    let mut main_arm_model = window.add_obj(parts.main_arm_obj, parts.mtl, scale);
    let mut index1_model = main_arm_model.add_obj(parts.index1_obj, parts.mtl, scale);
    let mut index2_model = index1_model.add_obj(parts.index2_obj, parts.mtl, scale);
    let mut index3_model = index2_model.add_obj(parts.index3_obj, parts.mtl, scale);

    let mut main_arm = Phalanx {
        model: main_arm_model,
        cur_pos: 0,
    };

    let mut index1 = Phalanx {
        model: index1_model,
        cur_pos: 0,
    };

    let mut index2 = Phalanx {
        model: index2_model,
        cur_pos: 0,
    };

    let mut index3 = Phalanx {
        model: index3_model,
        cur_pos: 0,
    };

    index2.model.set_local_translation(Translation3::new(-0.0025, -0.002, 0.035));
    index3.model.set_local_translation(Translation3::new(-0.002, -0.0041, 0.03));

    VrGlove {
        phalanges: vec![main_arm, index1, index2, index3],
    }
}