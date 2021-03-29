use std::string::String;

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

