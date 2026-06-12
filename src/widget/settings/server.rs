#[derive(Clone, Debug)]
pub enum Message {
    HostChanged(String),
    PortChanged(u16),
    SSLChanged(bool),
    Check,
    CheckFailed,
    CheckDone(crate::APIResponse<core_domain::result::HealthResult>),
}

pub struct ServerSettings {
    pub config: crate::ServerConfig,
    pub message: String,
}

impl ServerSettings {
    pub fn new() -> Self {
        Self {
            config: crate::ServerConfig::default(),
            message: String::new(),
        }
    }
    pub fn with_config(config: &crate::ServerConfig) -> Self {
        Self {
            config: config.clone(),
            message: String::new(),
        }
    }
    pub async fn check(server_config: crate::ServerConfig) -> Message {
        match server_config.health().await {
            Ok(result) => Message::CheckDone(result),
            Err(e) => {
                tracing::error!("server check error:{}", e);
                Message::CheckFailed
            }
        }
    }
}

impl crate::Widget for ServerSettings {
    type Message = Message;
    fn update(
        &mut self,
        message: Self::Message,
        _: &mut crate::State,
    ) -> iced::Task<Self::Message> {
        match message {
            Message::HostChanged(value) => {
                self.config.host = value;
                iced::Task::none()
            }
            Message::PortChanged(value) => {
                self.config.port = value;
                iced::Task::none()
            }
            Message::SSLChanged(value) => {
                self.config.is_ssl = value;
                iced::Task::none()
            }
            Message::Check => {
                self.message = "Checking...".to_string();
                iced::task::Task::future(Self::check(self.config.clone()))
            }
            Message::CheckDone(result) => {
                self.message = result.message;
                iced::Task::none()
            }
            Message::CheckFailed => {
                self.message = "Check failed".to_string();
                iced::Task::none()
            }
        }
    }
    fn view(&self) -> iced::Element<'_, Self::Message> {
        let host = iced::widget::column![
            iced::widget::text!("Host"),
            iced::widget::text_input("Please enter the server host", self.config.host.as_str())
                .on_input(Message::HostChanged)
        ]
        .width(iced::Length::Fixed(100.0));
        let port = iced::widget::column![
            iced::widget::text!("Port"),
            iced_aw::number_input(&self.config.port, 1..65535, Message::PortChanged).step(1)
        ]
        .width(iced::Length::Fixed(100.0));
        let ssl = iced::widget::column![
            iced::widget::text!("SSL"),
            iced::widget::checkbox(self.config.is_ssl).on_toggle(Message::SSLChanged)
        ]
        .width(iced::Length::Fixed(50.0));
        let config = iced::widget::row![host, port, ssl].spacing(10);
        let check = iced::widget::column![
            iced::widget::button("Check").on_press(Message::Check),
            iced::widget::text(self.message.as_str()),
        ];
        iced::widget::center_x(iced::widget::column![config, check]).into()
    }
}
