use std::borrow::BorrowMut;
use std::io::{BufReader, BufRead, ErrorKind};
use std::process::exit;
use std::time::Duration;
use serialport::{SerialPort, new};
use crate::VrGlove;

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

pub fn serial_read(reader: &mut BufReader<Box<dyn SerialPort>>, vrglove: &mut VrGlove) {
    let mut buff= String::new();
    let mut vec = vec![];

    match reader.read_line(&mut buff) {
        Ok(t) => {
            vec = buff.trim().split("/").collect();
        },
        Err(ref e) if e.kind() == ErrorKind::TimedOut => (),
        Err(e) => (),
    }

    let test = vrglove.phalanges.clone();

    for pair in vec.into_iter().zip(test.into_iter()) {
        let (value, mut phalanx) = pair;
        phalanx.borrow_mut().rotate_phalanx(value.parse::<i32>().unwrap());
    }


}