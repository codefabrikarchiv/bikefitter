use std::string::String;
use std::thread::{self, JoinHandle};
use std::str;

use serialport::{DataBits, StopBits};
use std::io::{self, Write};
use std::time::Duration;

mod datagram;
mod port;

pub struct Serial {
    pub conn: SerialPort,
    pub data: Datagram,
}

impl Serial {
    pub fn new() -> Serial {
        Serial { count: 0 }
    }

    pub fn get_ports() -> Vec<Port> {
        let mut vec = Vec::new();
        let ports = serialport::available_ports().expect("No ports found!");
        let mut index = 1;
        for p in ports {
            vec.push(Seria lPort {
                name: p.port_name,
                index: index,
            });
            index += 1;
        }
        return vec;
    }

    pub fn port_name_of(index: i32) -> String {
        get_ports()
            .into_iter()
            .find(|x| x.index == index)
            .unwrap()
            .name
    }

    pub fn start() -> () {
        let mut conn = serialport::new(port_name, 9600)
            .stop_bits(StopBits::One)
            .data_bits(DataBits::Eight)
            .timeout(Duration::from_millis(10))
            .open()
            .unwrap();
    }
}

impl Stream for Serial {
    type Item = Datagram;

    fn poll_next(mut self: Pin<&mut Self>, cy: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut serial_buf: Vec<u8> = vec![0; 1000];
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
                return Poll::Ready(Some(datagram))
            },
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
    }
}
