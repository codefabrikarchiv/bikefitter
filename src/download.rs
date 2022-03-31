use iced_futures::futures;
use std::time::Duration;
use std::io::BufReader;
use std::io::BufRead;

use serialport::{SerialPort, DataBits, StopBits};

// Just a little utility function
pub fn file<T: ToString>(port: T) -> iced::Subscription<Progress> {
    iced::Subscription::from_recipe(Download {
        port: port.to_string(),
    })
}

pub struct Download {
    port: String,
}

// Make sure iced can use our download stream
impl<H, I> iced_native::subscription::Recipe<H, I> for Download where H: std::hash::Hasher {
    type Output = Progress;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;

        std::any::TypeId::of::<Self>().hash(state);
        self.port.hash(state);
    }

    fn stream(self: Box<Self>, _input: futures::stream::BoxStream<'static, I>) -> futures::stream::BoxStream<'static, Self::Output> {
        Box::pin(futures::stream::unfold(State::Ready(self.port), |state| async move {
            match state {
                State::Ready(port) => {
                    let connection = serialport::new(port, 9600)
                        .stop_bits(StopBits::One)
                        .data_bits(DataBits::Eight)
                        .timeout(Duration::from_millis(1000))
                        .open();

                    match connection {
                        Ok(connection) => {
                            Some((
                                Progress::Started,
                                State::Reading {
                                    connection: BufReader::with_capacity(100, connection),
                                },
                            ))
                        }
                        Err(_) => {
                            Some((Progress::Errored, State::Finished))
                        }
                    }
                }
                State::Reading {
                    mut connection,
                } => {
                    let mut serial_buf = String::new();
                    match connection.read_line(&mut serial_buf) {
                        Ok(len) => len,
                        Err(_) => return Some((
                            Progress::Advanced("".to_string()),
                            State::Reading  {
                                connection
                            }
                        ))
                    };

                    Some((
                        Progress::Advanced(serial_buf),
                        State::Reading {
                            connection,
                        },
                    ))
                },
                State::Finished => {
                    // We do not let the stream die, as it would start a
                    // new download repeatedly if the user is not careful
                    // in case of errors.
                    let _: () = iced::futures::future::pending().await;

                    None
                }
            }},
        ))
    }
}

#[derive(Debug, Clone)]
pub enum Progress {
    Started,
    Advanced(String),
    Errored,
}

pub enum State {
    Ready(String),
    Reading {
        connection: BufReader<Box<dyn SerialPort>>,
    },
    Finished,
}
