use std::borrow::BorrowMut;
use std::f32::consts::PI;
use std::io::{BufReader, BufRead, ErrorKind};
use std::process::exit;
use std::time::Duration;
use kiss3d::event::Key::P;
use nalgebra::UnitQuaternion;
use serialport::{SerialPort, new};
use crate::{Phalanx, VrGlove};

pub fn init_port(port_name: &str, baud_rate: u32) -> Box<dyn SerialPort> {
    let port = new(port_name, baud_rate)
        .timeout(Duration::from_millis(100))
        .open()
        .expect("Unable to open SerialPort");

    println!("Receiving data on: {}, with baud rate: {}", &port_name, &baud_rate);

    port
}

pub fn serial_read(reader: &mut BufReader<Box<dyn SerialPort>>, vrglove: &mut VrGlove, arm: &mut Phalanx) {

    let mut buff= String::new();
    let mut vec = vec![];

    match reader.read_line(&mut buff) {
        Ok(t) => {
            vec = buff.trim().split("/").collect();
        },
        Err(ref e) if e.kind() == ErrorKind::TimedOut => (),
        Err(e) => (),
    }

    // println!("{:?}", &buff);

    if vec.len() == 6 {
        mpu6050(&vec[..3], arm);
        control_finger(&vec[3..], vrglove);
    }
}

pub fn mpu6050(slice: &[&str], arm: &mut Phalanx) {
    let (mut roll, mut pitch, mut yaw) = (0.0, 0.0, 0.0);
    match slice[0].parse::<f32>() {
        Ok(res) => roll = res * PI/180.,
        Err(e) => (),
    }
    match slice[1].parse::<f32>() {
            Ok(res) => pitch = res * PI/180.,
            Err(e) => (),
        }
    match slice[2].parse::<f32>() {
        Ok(res) => yaw = res * PI/180.,
        Err(e) => (),
    }

    arm.model.set_local_rotation(
        UnitQuaternion::from_euler_angles(roll, pitch, yaw));
}

pub fn control_finger(slice: &[&str], finger: &mut VrGlove) {
    for (id, phalanx) in finger.phalanges.iter().enumerate() {
        let val = match slice[id].parse::<i32>() {
            Ok(res) => res,
            Err(_) => -1,
        };

        phalanx.clone().rotate_phalanx(val);
    }
}