// #![windows_subsystem="windows"]

mod download;
mod serial;
mod dataframe;

use std::str::FromStr;
use dataframe::Dataframe;
use std::convert::TryInto;

use iced::{
    button, executor, Align, Application, Button, Column, Command,
    Element, Settings, Subscription, Text, Radio, Clipboard, Row, Length,
};

#[derive(Default)]
struct Reader {
    start_button: button::State,
    copy_button: button::State,
    port: i32,
    active: bool,
    last_value: Dataframe,
    snapshots: Vec<(dataframe::Dataframe, button::State)>,
}

#[derive(Debug, Clone)]
enum Message {
    RadioSelected(i32),
    SerialStartStop,
    SerialUpdate(download::Progress),
    CopyToClipboard,
    Delete(i32)
}

impl Application for Reader {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Bike Fitting")
    }

    fn update(&mut self, message: Self::Message, clipboard: &mut Clipboard) -> Command<Self::Message> {
        match message {
            Message::RadioSelected(v) => {
                self.port = v;
            }
            Message::SerialStartStop => {
                self.active = !self.active;
            }
            Message::SerialUpdate(message) => {
                match message {
                    download::Progress::Started => {
                        // no op
                    }
                    download::Progress::Advanced(line) => {
                        match Dataframe::from_str(&line) {
                            Ok(frame) => {
                                println!("SerialUpdate read");
                                if self.active {
                                    self.last_value = frame;
                                    if frame.action == 1 {
                                        self.snapshots.push((frame, button::State::new()));
                                    }
                                }
                            },
                            Err(_e) => println!("SerialUpdate error")
                        }
                    }
                    download::Progress::Errored => {
                        // no op
                    }
                }
            }
            Message::CopyToClipboard => {
                let mut payload = "".to_string();
                payload.push_str("ID\tX\tY\n");
                for (index, (frame, _state)) in self.snapshots.iter().enumerate() {
                    payload.push_str(&(format!("{}\t{}\t{}\n", index + 1, frame.x, frame.y)));
                }
                clipboard.write(payload);
            }
            Message::Delete(index) => {
                self.snapshots.remove(index as usize);
            }
        };

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
        let window = Row::new().padding(40).spacing(20).align_items(Align::Center);
        let mut ports = Column::new().spacing(20).align_items(Align::Start).width(Length::Fill).height(Length::Fill);
        let mut data = Column::new().spacing(20).align_items(Align::Start).width(Length::Fill).height(Length::Fill);
        let mut list = Column::new().spacing(10).align_items(Align::Start).width(Length::Fill).height(Length::Fill);

        ports = ports.push(
            Text::new("Portauswahl").size(30).height(Length::Units(50))
        );
        for p in serial::get_ports() {
            ports = ports.push(Radio::new(p.index, format!("{}", p.name), Some(self.port), Message::RadioSelected))
        }
        let label = if self.active { "Stop" } else { "Start" };
        ports = ports.push(
            Button::new(&mut self.start_button, Text::new(label)).on_press(Message::SerialStartStop)
        );

        data = data.push(
            Text::new("Live").size(30).height(Length::Units(50))
        )
        .push(
            Column::new().spacing(20).align_items(Align::Center).push(
                Row::new().spacing(12).align_items(Align::Center).push(
                    Text::new("X")
                ).push(
                    Text::new(self.last_value.x.to_string()).size(30)
                )
            ).push(
                Row::new().spacing(12).align_items(Align::Center).push(
                    Text::new("Y")
                ).push(
                    Text::new(self.last_value.y.to_string()).size(30)
                )
            )
        );

        list = list.push(
            Text::new("Daten").size(30).height(Length::Units(50))
        ).push(
            Button::new(&mut self.copy_button, Text::new("Kopieren")).on_press(Message::CopyToClipboard)
        );

        let dat = self.snapshots.iter_mut().enumerate()
            .fold(Column::new(), |col, (i, (frame, state))| {
                col.push(
                    Row::new().spacing(12).padding(5).align_items(Align::Center).push(
                        Text::new((i + 1).to_string()).size(30)
                    ).push(
                        Text::new(format!("{} | {}", frame.x, frame.y)).width(Length::Fill)
                    ).push(
                        Button::new(state, Text::new("löschen")).on_press(Message::Delete(i.try_into().unwrap()))
                    )
                )
            });
        list = list.push(dat);

        /*for (frame, state) in &self.snapshots {
            let str = format!("{} {} {}", (index + 1), frame.x, frame.y);
            list = list.push(
                Row::new().spacing(12).align_items(Align::Center).push(
                    Text::new(str).width(Length::Fill)
                ).push(
                    Button::new(state.clone().to_mut(), Text::new("löschen")).on_press(Message::Delete(index.try_into().unwrap()))
                )
            );
            index += 1;
        }*/

        window.push(ports).push(data).push(list).into()
    }
}

fn main() {
    let result = Reader::run(Settings::default());
    match result {
        Ok(v) => println!("v {:?}", v),
        Err(e) => println!("e {:?}", e)
    }
}
