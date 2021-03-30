mod download;
mod serial;
mod dataframe;

extern crate nfd;
use nfd::Response;

use xlsxwriter::Workbook;

use iced::{
    button, executor, Align, Application, Button, Column, Command,
    Element, Settings, Subscription, Text, Radio, Clipboard, Row,
};

#[derive(Default)]
struct Reader {
    start_button: button::State,
    export_button: button::State,
    port: i32,
    active: bool,
    last_value: dataframe::Dataframe,
    snapshots: Vec<dataframe::Dataframe>,
}

#[derive(Debug, Clone)]
enum Message {
    RadioSelected(i32),
    SerialStartStop,
    SerialUpdate(download::Progress),
    Export,
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

    fn update(&mut self, message: Self::Message, _clipboard: &mut Clipboard) -> Command<Self::Message> {
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
                        let frame = serial::parse(line);
                        self.last_value = frame;
                        if frame.action == 1 {
                            self.snapshots.push(frame);
                        }
                    }
                    download::Progress::Errored => {
                        // no op
                    }
                }
            }
            Message::Export => {
                let result = nfd::open_save_dialog(Some("xlsx"), None).unwrap_or_else(|e| {
                    panic!("{}", e);
                });
                match result {
                    Response::Cancel => println!("User canceled"),
                    Response::Okay(file_path) => {
                        let fp = file_path + ".xlsx";
                        let workbook = Workbook::new(&fp);
                        let sheet = workbook.add_worksheet(None);
                        match sheet {
                            Ok(mut sheet) => {
                                sheet.write_string(0, 0, "Nummer", None);
                                sheet.write_string(0, 1, "x", None);
                                sheet.write_string(0, 2, "y", None);

                                let mut row = 1;
                                for snapshot in &self.snapshots {
                                    sheet.write_number(row, 0, row.into(), None);
                                    sheet.write_number(row, 1, snapshot.x.into(), None);
                                    sheet.write_number(row, 2, snapshot.y.into(), None);
                                    row += 1;
                                }
                                workbook.close();
                            }
                            Err(e) => {
                                eprintln!("{:?}", e);
                            }
                        }
                    }
                    Response::OkayMultiple(files) => println!("Files {:?}", files),
                }
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
        let window = Row::new().padding(20).align_items(Align::Center);
        let mut ports = Column::new().padding(20).align_items(Align::Start);
        let mut data = Column::new().padding(20).align_items(Align::Start);
        let mut list = Column::new().padding(20).align_items(Align::Start);

        ports = ports.push(
            Text::new("Portauswahl").size(30)
        );
        for p in serial::get_ports() {
            ports = ports.push(Radio::new(p.index, format!("{}", p.name), Some(self.port), Message::RadioSelected))
        }
        let label = if self.active { "Stop" } else { "Start" };
        ports = ports.push(
            Button::new(&mut self.start_button, Text::new(label)).on_press(Message::SerialStartStop)
        );

        data = data.push(
            Text::new("Live").size(30)
        ).push(
            Row::new().padding(20).align_items(Align::Center).push(
                Text::new(self.last_value.x.to_string())
            ).push(
                Text::new(self.last_value.y.to_string())
            )
        );

        list = list.push(
            Text::new("Daten").size(30)
        );
        let mut index = 1;
        for snapshot in &self.snapshots {
            let str = format!("{} {} {}", index, snapshot.x, snapshot.y);
            list = list.push(Text::new(str));
            index += 1;
        }
        list = list.push(
            Button::new(&mut self.export_button, Text::new("Export")).on_press(Message::Export)
        );

        window.push(ports).push(data).push(list).into()
    }
}

fn main() {
    let result = Reader::run(Settings::default());
    match result {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{:?}", e)
    }
}
