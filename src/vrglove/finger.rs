use kiss3d::scene::SceneNode;
use nalgebra::{UnitQuaternion, Vector3};

const MAGIC_VALUE: f32 = 651.2605042;

#[derive(Clone)]
pub struct Phalanx {
    pub model: SceneNode,
    pub cur_pos: i32,
}

impl Phalanx {
    pub fn rotate_phalanx(&mut self, value: i32) {
        self.cur_pos = value;
        let rot = UnitQuaternion::from_axis_angle(
            &Vector3::x_axis(),
            (self.cur_pos as f32) / MAGIC_VALUE,
        );

        self.model.set_local_rotation(rot);
    }
    
    pub fn rotate_base(roll: f32, pitch: f32, yaw: f32) {
        todo!()
    }
}