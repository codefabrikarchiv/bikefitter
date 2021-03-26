use std::string::String;
use std::thread::{self, JoinHandle};
use std::str;

use serialport::{DataBits, StopBits};
use std::io::{self, Write};
use std::time::Duration;

#[derive(Default)]
pub struct Datagram {
    pub x: i32,
    pub y: i32,
    pub action: i32,
}

pub struct SerialPort {
    pub name: String,
    pub index: i32,
}

pub fn get_ports() -> Vec<SerialPort> {
    let mut vec = Vec::new();
    let ports = serialport::available_ports().expect("No ports found!");
    let mut index = 1;
    for p in ports {
        vec.push(SerialPort {
            name: p.port_name,
            index: index,
        });
        index += 1;
    }
    return vec;
}

pub fn register<F>(port_name: String, mut cb: F) -> JoinHandle<()>
    where F: FnMut(Box<Datagram>) -> bool + Send + 'static {

    thread::spawn(move|| {
        let mut conn = serialport::new(port_name, 9600)
            .stop_bits(StopBits::One)
            .data_bits(DataBits::Eight)
            .timeout(Duration::from_millis(10))
            .open()
            .unwrap();

        let mut serial_buf: Vec<u8> = vec![0; 1000];
        loop {
            match conn.read(serial_buf.as_mut_slice()) {
                Ok(t) => {
                    io::stdout().write_all(&serial_buf[..t]).unwrap();
                    let first_line = str::from_utf8(&serial_buf).unwrap().lines().next().unwrap();
                    let parts: Vec<&str> = first_line.split('|').collect();
                    let x = parts[0].to_string().parse::<i32>().unwrap();
                    let y = parts[1].to_string().parse::<i32>().unwrap();
                    let action = parts[2].to_string().parse::<i32>().unwrap();
                    println!("SEND {} {} {}", x, y, action);
                    let datagram = Datagram {
                        x: x,
                        y: y,
                        action: action
                    };
                    if !cb(Box::new(datagram)) {
                        return ();
                    }
                },
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{:?}", e),
            }
        }
    })
}

pub fn port_name_of(index: i32) -> String {
    get_ports()
        .into_iter()
        .find(|x| x.index == index)
        .unwrap()
        .name
}
