mod robot_model;

use crystal_engine::*;
use cgmath::{Matrix4, Point3, Vector3};
use crystal_engine::event::{VirtualKeyCode, WindowEvent};

use self::{
    robot_model::Robot,
};

pub struct Game {
    robot: Robot,
}

impl crystal_engine::Game for Game {
    fn init(state: &mut GameState) -> Self {
        let robot = Robot::new();

        state.camera = Matrix4::look_at(
            Point3::new(0.0, 0.0, 1.0),
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        );

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


