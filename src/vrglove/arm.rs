use crate::Phalanx;
use kiss3d::window::Window;
use nalgebra::{Translation3, Vector3};
use std::path::Path;

pub struct ArmParts<'a> {
    main_arm_obj: &'a Path,
    index1_obj: &'a Path,
    index2_obj: &'a Path,
    index3_obj: &'a Path,
    thumb_obj: &'a Path,
    middle_obj: &'a Path,
    ring_obj: &'a Path,
    pink_obj: &'a Path,
    mtl: &'a Path,
}

pub struct VrGlove {
    pub phalanges: Vec<Phalanx>,
}

pub fn arm_init(window: &mut Window) -> (VrGlove, Phalanx) {
    let parts = ArmParts {
        main_arm_obj: Path::new("src/assets/main_arm.obj"),
        index1_obj: Path::new("src/assets/index1.obj"),
        index2_obj: Path::new("src/assets/index2.obj"),
        index3_obj: Path::new("src/assets/index3.obj"),
        thumb_obj: Path::new("src/assets/thumb.obj"),
        middle_obj: Path::new("src/assets/middle.obj"),
        ring_obj: Path::new("src/assets/ring.obj"),
        pink_obj: Path::new("src/assets/pink.obj"),
        mtl: Path::new("src/assets/main_arm.mtl"),
    };

    let scale = Vector3::new(1.0, 1.0, 1.0);
    let mut main_arm_model = window.add_obj(parts.main_arm_obj, parts.mtl, scale);
    let mut thumb_model = main_arm_model.add_obj(parts.thumb_obj, parts.mtl, scale);
    let mut middle_model = main_arm_model.add_obj(parts.middle_obj, parts.mtl, scale);
    let mut ring_model = main_arm_model.add_obj(parts.ring_obj, parts.mtl, scale);
    let mut pink_model = main_arm_model.add_obj(parts.pink_obj, parts.mtl, scale);

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

    thumb_model.set_local_translation(Translation3::new(0.035, -0.013, -0.0072));
    middle_model.set_local_translation(Translation3::new(0.002, -0.002, 0.037));
    ring_model.set_local_translation(Translation3::new(-0.0158, -0.002, 0.032));
    pink_model.set_local_translation(Translation3::new(-0.0332, -0.002, 0.022));

    index1
        .model
        .set_local_translation(Translation3::new(0.0233, -0.0021, 0.036));
    index2
        .model
        .set_local_translation(Translation3::new(-0.0025, -0.002, 0.035));
    index3
        .model
        .set_local_translation(Translation3::new(-0.002, -0.0041, 0.03));

    (
        VrGlove {
            phalanges: vec![index1, index2, index3],
        },
        main_arm,
    )
}
