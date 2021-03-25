use std::io::{self, Write, stdin, stdout};
use std::time::Duration;

fn main() {
    let ports = serialport::available_ports().expect("No ports found!");
    let mut index = 1;
    for p in &ports {
        println!("{}: {}", index, p.port_name);
        index += 1;
    }

    println!("Select port number: ");
    let _ = stdout().flush();
    let mut s = String::new();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    let index = s.parse::<usize>().unwrap();

    let port = serialport::new(&ports[index - 1].port_name, 9600)
        .timeout(Duration::from_millis(10))
        .open();

    match port {
        Ok(mut port) => {
            let mut serial_buf: Vec<u8> = vec![0; 1000];
            loop {
                match port.read(serial_buf.as_mut_slice()) {
                    Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open port. Error: {}", e);
            ::std::process::exit(1);
        }
    }
}
