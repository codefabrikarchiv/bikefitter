use std::string::String;
use crate::dataframe::Dataframe;

pub struct Port {
    pub index: i32,
    pub name: String,
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
    get_ports()
        .into_iter()
        .find(|x| x.index == index)
        .unwrap()
        .name
}

pub fn parse(line: String) -> Dataframe {
    let parts: Vec<&str> = line.split('|').collect();
    let x = parts[0].to_string().parse::<i32>().unwrap();
    let y = parts[1].to_string().parse::<i32>().unwrap();
    let action = parts[2].to_string().parse::<i32>().unwrap();
    Dataframe {
        x,
        y,
        action
    }
}

