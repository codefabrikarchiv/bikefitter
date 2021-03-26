mod serial;

use iced::{executor, button, Align, Clipboard, Button, Column, Element, Application, Settings, Text, Radio, Command, Subscription};

#[derive(Default)]
struct Reader {
    port: i32,
    start_button: button::State,
    active: bool,
    last: Datagram
}

#[derive(Debug, Clone, Copy)]
enum Message {
    RadioSelected(i32),
    SerialStarted,
    SerialUpdated(Datagram)
}

impl Application for Reader {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Self::Message, _clipboard: &mut Clipboard) -> Command<Self::Message> {
        match message {
            Message::RadioSelected(v) => {
                self.port = v;
            }
            Message::SerialStarted => {
                println!("start");
                self.active = true;
                let port_name = serial::port_name_of(self.port);
                let mut Serial = Serial::new()
                /*let handler = serial::register(port_name, |datagram: Box<Datagram>| {
                    // println!("RECV {} {} {}", datagram.x, datagram.y, datagram.action);
                    self.last_values = *datagram;
                    true
                });
                handler.join();*/
            },
            Message::SerialUpdate(datagram) => {
                *self = Reader {
                    reading: datagram
                }
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let mut ui = Column::new()
            .padding(20)
            .align_items(Align::Center);
        let ports = serial::get_ports();
        for p in ports {
            ui = ui.push(
                Radio::new(p.index, format!("{}", p.name), Some(self.port), Message::RadioSelected)
            )
        }
        ui.push(
            Button::new(&mut self.start_button, Text::new("Start"))
                    .on_press(Message::SerialStarted),
        ).push(
            Text::new(self.active.to_string())
        ).push(
            Text::new(self.last_values.x.to_string())
        ).into()
    }


}

fn main() {
    let result = Reader::run(Settings::default());
    match result {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{:?}", e)
    }
}
