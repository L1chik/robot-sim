use crystal_engine::*;
use cgmath::{Vector3, Zero};


/// STRUCTURES ///
pub struct Robot {
    position: Vector3<f32>,
    direction: Vector3<f32>,
    handle: ModelHandle
}

impl Robot {
    pub fn new() -> Self {
        Self {
            position: Vector3::zero(),
            direction: Vector3::zero(),
            handle: state
                .new_fbx_model("src/assets/SM_NightStand2.fbx")
                .build()
                .unwrap()
        }
    }
}