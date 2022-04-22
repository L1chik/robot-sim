use crate::{Phalanx, VrGlove};
use nalgebra::UnitQuaternion;
use serialport::{new, SerialPort};
use std::io::{BufRead, BufReader, ErrorKind};
use std::time::Duration;

pub fn init_port(port_name: &str, baud_rate: u32) -> Box<dyn SerialPort> {
    let port = new(port_name, baud_rate)
        .timeout(Duration::from_millis(100))
        .open()
        .expect("Unable to open SerialPort");

    println!(
        "Receiving data on: {}, with baud rate: {}",
        &port_name, &baud_rate
    );

    port
}

pub fn serial_read(
    reader: &mut BufReader<Box<dyn SerialPort>>,
    vrglove: &mut VrGlove,
    arm: &mut Phalanx,
) {
    let mut buff = String::new();
    let mut vec = vec![];

    match reader.read_line(&mut buff) {
        Ok(_) => {
            vec = buff.trim().split('/').collect();
        }
        Err(ref e) if e.kind() == ErrorKind::TimedOut => (),
        Err(_) => (),
    }

    // println!("{:?}", &buff);

    if vec.len() == 6 {
        mpu6050(&vec[..3], arm);
        control_finger(&vec[3..], vrglove);
    }
}

pub fn mpu6050(slice: &[&str], arm: &mut Phalanx) {
    let (mut roll, mut pitch, mut yaw) = (0.0, 0.0, 0.0);

    if let Ok(res) = slice[0].parse::<f32>() {
        roll = res.to_radians()
    };
    if let Ok(res) = slice[1].parse::<f32>() {
        pitch = res.to_radians()
    };
    if let Ok(res) = slice[2].parse::<f32>() {
        yaw = res.to_radians()
    };

    arm.model
        .set_local_rotation(UnitQuaternion::from_euler_angles(roll, pitch, yaw));
}

pub fn control_finger(slice: &[&str], finger: &mut VrGlove) {
    for (id, phalanx) in finger.phalanges.iter().enumerate() {
        let val = slice[id].parse::<i32>().unwrap_or(-1);

        phalanx.clone().rotate_phalanx(val);
    }
}
