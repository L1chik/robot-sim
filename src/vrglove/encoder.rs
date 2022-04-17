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

pub fn serial_value(reader: &mut BufReader<Box<dyn SerialPort>>) -> i32 {
    let mut string= String::new();

    match reader.read_line(&mut string) {
        Ok(t) => {
            return string.trim().parse::<i32>().unwrap();
        },
        Err(ref e) if e.kind() == ErrorKind::TimedOut => (),
        Err(e) => (),
    }

    -1
}

pub fn string_to_val(str: &String) -> i32 {
    let mut buff = 0;

    buff
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

    // for pair in vec.into_iter().zip(vrglove.phalanges.clone().into_iter()) {
    //     let (value, mut phalanx) = pair;
    //     let val = match value.parse::<i32>() {
    //         Ok(res) => res,
    //         Err(_) => -1,
    //     };
    //     phalanx.rotate_phalanx(val); //value.parse::<i32>().unwrap()
    // }
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

    println!("roll {}, pitch {}, yaw {}", &roll, &pitch, &yaw);
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