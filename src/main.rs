mod download;
mod serial;

mod dataframe;
use dataframe::Dataframe;

use iced::{
    button, executor, Align, Application, Button, Column, Command,
    Element, Settings, Subscription, Text, Radio, Clipboard,
};

#[derive(Default)]
struct Reader {
    start_button: button::State,
    port: i32,
    active: bool,
    last_value: dataframe::Dataframe,
}

#[derive(Debug, Clone)]
enum Message {
    RadioSelected(i32),
    SerialStarted,
    SerialUpdate(download::Progress),
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
                self.active = true;
            }
            Message::SerialUpdate(message) => {
                match message {
                    download::Progress::Started => {
                        // no op
                    }
                    download::Progress::Advanced(line) => {
                        let parts: Vec<&str> = line.split('|').collect();
                        let x = parts[0].to_string().parse::<i32>().unwrap();
                        let y = parts[1].to_string().parse::<i32>().unwrap();
                        let action = parts[2].to_string().parse::<i32>().unwrap();
                        self.last_value = Dataframe {
                            x,
                            y,
                            action
                        };
                    }
                    download::Progress::Errored => {
                        // no op
                    }
                }
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.active {
            download::file(serial::port_name_of(self.port))
                .map(Message::SerialUpdate)
        } else {
            Subscription::none()
        }
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
            Text::new(self.last_value.x.to_string())
        ).push(
            Text::new(self.last_value.y.to_string())
        ).push(
            Text::new(self.last_value.action.to_string())
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
