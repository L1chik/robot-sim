use std::io::{BufReader, BufRead, ErrorKind};
use std::process::exit;
use std::time::Duration;
use serialport::{SerialPort, new};

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

pub struct PortReader {
    port_reader: BufReader<Box<dyn SerialPort>>,
    pub data: String
}

impl PortReader {
    pub fn new(port: Box<dyn SerialPort>) -> Self {
        PortReader {
            port_reader: BufReader::new(port),
            data: String::new(),
        }
    }
}
