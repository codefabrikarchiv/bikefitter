use std::string::String;
use std::str;

use async_std::task::Poll;
use async_std::task::Context;
use async_std::prelude::Stream;


use serialport::{DataBits, StopBits};
use std::io::{self, Write};
use std::time::Duration;

use crate::protocol::protocol::Datagram;
use crate::protocol::protocol::Port;

pub struct Serial {
    pub conn: Box<dyn serialport::SerialPort>,
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
            vec.push(Port {
                name: p.port_name,
                index: index,
            });
            index += 1;
        }
        return vec;
    }

    pub fn port_name_of(index: i32) -> String {
        Serial::get_ports()
            .into_iter()
            .find(|x| x.index == index)
            .unwrap()
            .name
    }

    pub fn start(port_name: String) -> () {
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

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut serial_buf: Vec<u8> = vec![0; 1000];
        match self.conn.read(serial_buf.as_mut_slice()) {
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
