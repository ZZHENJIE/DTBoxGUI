use iced::{Element, Task};

#[derive(Clone, Debug)]
pub enum Message {
    UserNameChanged(String),
    PasswordChanged(String),
    ChangePage,
    Confirm,
    ServerHostChanged(String),
    ServerPortChanged(u16),
    ServerSSLChanged(bool),
    ServerCheck,
    ServerCheckFailed,
    ServerCheckDone(crate::APIResponse<core_domain::result::HealthResult>),
}

pub struct StartPage {
    pub user_login_state: bool,
    pub user_name: String,
    pub password: String,
    pub is_login_page: bool,
    pub server_config: crate::ServerConfig,
    pub server_check_result: String,
}

impl StartPage {
    pub fn new() -> Self {
        Self {
            user_login_state: false,
            user_name: String::new(),
            password: String::new(),
            is_login_page: true,
            server_config: crate::ServerConfig::default(),
            server_check_result: String::new(),
        }
    }
    pub fn check_login_state(&self) -> Task<Message> {
        iced::Task::none()
    }
    pub fn update(&mut self, message: Message, state: &mut crate::State) -> Task<Message> {
        match message {
            Message::PasswordChanged(value) => {
                self.password = value;
                iced::Task::none()
            }
            Message::UserNameChanged(value) => {
                self.user_name = value;
                iced::Task::none()
            }
            Message::ChangePage => {
                self.is_login_page = !self.is_login_page;
                iced::Task::none()
            }
            Message::Confirm => iced::Task::none(),
            Message::ServerHostChanged(value) => {
                self.server_config.host = value;
                iced::Task::none()
            }
            Message::ServerPortChanged(value) => {
                self.server_config.port = value;
                iced::Task::none()
            }
            Message::ServerSSLChanged(value) => {
                self.server_config.is_ssl = value;
                iced::Task::none()
            }
            Message::ServerCheck => {
                self.server_check_result = "Checking...".to_string();
                iced::task::Task::future(Self::server_check(self.server_config.clone()))
            }
            Message::ServerCheckDone(result) => {
                self.server_check_result = result.message;
                iced::Task::none()
            }
            Message::ServerCheckFailed => {
                self.server_check_result = "Check failed".to_string();
                iced::Task::none()
            }
        }
    }
    pub async fn server_check(server_config: crate::ServerConfig) -> Message {
        match server_config.health().await {
            Ok(result) => Message::ServerCheckDone(result),
            Err(e) => {
                println!("server check error:{}", e);
                Message::ServerCheckFailed
            }
        }
    }
    pub fn is_login_page_str(&self) -> &str {
        if self.is_login_page {
            "Create"
        } else {
            "Login"
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        let user_name = iced::widget::row![
            iced::widget::text!("UserName"),
            iced::widget::text_input("Please enter your username", self.user_name.as_str())
                .on_input(Message::UserNameChanged)
        ];
        let password = iced::widget::row![
            iced::widget::text!("Password"),
            iced::widget::text_input("Please enter your password", self.password.as_str())
                .on_input(Message::PasswordChanged)
        ];
        let user_button = iced::widget::row![
            iced::widget::button(self.is_login_page_str()).on_press(Message::ChangePage),
            iced::widget::button("Confirm").on_press(Message::Confirm)
        ];
        let server_host = iced::widget::row![
            iced::widget::text!("Server Host"),
            iced::widget::text_input(
                "Please enter the server host",
                self.server_config.host.as_str()
            )
            .on_input(Message::ServerHostChanged)
        ];
        let server_port = iced::widget::row![
            iced::widget::text!("Server Port"),
            iced::widget::text_input(
                "Please enter the server port",
                self.server_config.port.to_string().as_str()
            )
            .on_input(|port| Message::ServerPortChanged(port.parse().unwrap_or_default()))
        ];
        let server_ssl = iced::widget::row![
            iced::widget::text!("Server SSL"),
            iced::widget::checkbox(self.server_config.is_ssl).on_toggle(Message::ServerSSLChanged)
        ];
        let server_check = iced::widget::row![
            iced::widget::text!("Server Check"),
            iced::widget::button("Check").on_press(Message::ServerCheck),
            iced::widget::text(self.server_check_result.as_str()),
        ];
        iced::widget::column![
            user_name,
            password,
            user_button,
            server_host,
            server_port,
            server_ssl,
            server_check
        ]
        .into()
    }
}
