use iced::Task;
use iced::{
    widget::{Button, Column, Container, Row, Text, TextInput},
    Alignment, Element, Length,
};
use smart_home_gui::tcp_client::SmartSocketClient;

use std::sync::{Arc, Mutex};

const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 55331;

pub fn main() -> iced::Result {
    iced::application(
        "Smart Socket Control",
        SmartSocketApp::update,
        SmartSocketApp::view,
    )
    .run_with(SmartSocketApp::new)
}

#[derive(Debug)]
struct SmartSocketApp {
    power_state: bool,
    client: Option<Arc<Mutex<SmartSocketClient>>>,
    error_message: Option<String>,
    host: String,
    port: String,
}

#[derive(Debug, Clone)]
enum Message {
    TogglePower,
    Connect,
    HostInput(String),
    PortInput(String),
}

impl SmartSocketApp {
    fn new() -> (Self, Task<Message>) {
        (
            SmartSocketApp {
                power_state: false,
                client: None,
                error_message: None,
                host: DEFAULT_HOST.to_string(),
                port: DEFAULT_PORT.to_string(),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Connect => {
                let addr = format!("{}:{}", self.host, self.port);
                match SmartSocketClient::new(&addr) {
                    Ok(client) => {
                        self.client = Some(Arc::new(Mutex::new(client)));
                        self.error_message = None;

                        if let Ok(mut client) = self.client.as_ref().unwrap().lock() {
                            if let Ok(status) = client.get_status() {
                                println!("Status: {}", status);
                                self.power_state = status == "on";
                            }
                        }
                    }
                    Err(e) => {
                        self.error_message = Some(e.to_string());
                    }
                }
                Task::none()
            }
            Message::TogglePower => {
                if let Some(client) = &self.client {
                    if let Ok(mut client) = client.lock() {
                        match client.toggle_power() {
                            Ok(_) => {
                                if let Ok(status) = client.get_status() {
                                    self.power_state = status == "on";
                                }
                            }
                            Err(e) => {
                                self.error_message = Some(format!("Failed to toggle power: {}", e));
                            }
                        }
                    }
                } else {
                    self.error_message = Some("Not connected to socket".to_string());
                }
                Task::none()
            }
            Message::HostInput(value) => {
                self.host = value;
                Task::none()
            }
            Message::PortInput(value) => {
                self.port = value;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let title = Text::new("Smart Socket Control").size(40);

        let host_settings = Row::new()
            .push(Text::new("Host: ").size(16))
            .push(
                TextInput::new("Host", &self.host)
                    .on_input(Message::HostInput)
                    .padding(5)
                    .width(Length::Fixed(150.0)),
            )
            .spacing(10);

        let port_settings = Row::new()
            .push(Text::new("Port: ").size(14))
            .push(
                TextInput::new("Port", &self.port)
                    .on_input(Message::PortInput)
                    .padding(5)
                    .width(Length::Fixed(100.0)),
            )
            .spacing(10);

        let connect_button = Button::new(Text::new("Connect")).on_press(Message::Connect);

        let power_status = Text::new(if self.power_state {
            "Socket Status: ON"
        } else {
            "Socket Status: OFF"
        })
        .size(20);

        let power_button = Button::new(Text::new(if self.power_state {
            "Turn OFF"
        } else {
            "Turn ON"
        }))
        .on_press(Message::TogglePower);

        let mut content = Column::new()
            .push(title)
            .push(host_settings)
            .push(port_settings)
            .push(connect_button)
            .push(power_status)
            .push(power_button)
            .spacing(24)
            .align_x(Alignment::Center);

        if let Some(error) = &self.error_message {
            content = content.push(Text::new(error).size(16).color([1.0, 0.0, 0.0]));
        }

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .into()
    }
}
