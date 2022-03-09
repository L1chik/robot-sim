// mod robot_model;

extern crate crystal_engine;
extern crate cgmath;

use cgmath::{Matrix4, Point3, Vector3, BaseFloat};
use crystal_engine::*;
use crystal_engine::ModelHandle;

pub struct Game {
    robot: ModelHandle,
}

impl crystal_engine::Game for Game {
    fn init(state: &mut GameState) -> Self {
        let robot = state
            .new_fbx_model("assets/kuka.fbx")
            .build()
            .unwrap();

        // state.camera = cgmath::matrix::Matrix4::from_translation(Vector3::new(0.0, 1.0, 0.0));

        Self {
            robot,
        }
    }

    fn update(&mut self, state: &mut GameState) {
        todo!()
    }
}

fn main() {
    Window::<Game>::new(800., 600.).unwrap().run();
}

