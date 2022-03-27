use kiss3d::scene::SceneNode;
use na::{Vector3, Unit};

#[derive(Clone)]
pub struct Robot {
    pub joints: Vec<Part>,

    // Active joint
    pub index_of_joint: usize,
}

#[derive(Clone)]
pub struct Part {
    pub node: SceneNode,
    pub axis: Unit<Vector3<f32>>,

    // MIN, CURRENT, MAX
    pub angle: (f32, f32, f32),
}

#[derive(Debug, Clone, Default)]
pub struct RPoint {
    axis_angles: Vec<f32>,
}

impl Robot {
    pub fn set_joint_index(&mut self, joint: usize) {
        self.index_of_joint = joint;
    }

    pub fn get_active_joint(&self) -> usize {
        self.index_of_joint
    }
}

impl RPoint {
    pub fn save_point(&mut self, robot: &Robot) {
        for (id, joint) in robot.joints.iter().enumerate() {
            self.axis_angles.push(joint.angle.1)
        }
    }
}