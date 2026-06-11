use iced::Task;

#[derive(Clone, Debug)]
pub enum Message {
    UserNameChanged(String),
    PasswordChanged(String),
    ChangePage,
    Confirm,
    OpenSettingsWindow,
    ServerSettings(crate::widget::ServerSettingsMessage),
}

pub struct StartPage {
    pub user_login_state: bool,
    pub user_name: String,
    pub password: String,
    pub is_login_page: bool,
    pub server_settings: crate::widget::ServerSettings,
}

impl crate::Widget for StartPage {
    type Message = Message;

    fn update(
        &mut self,
        message: Self::Message,
        state: &mut crate::State,
    ) -> iced::Task<Self::Message> {
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
            Message::Confirm => {
                self.user_login_state = true;
                iced::Task::none()
            }
            Message::ServerSettings(msg) => self
                .server_settings
                .update(msg, state)
                .map(Message::ServerSettings),
            Message::OpenSettingsWindow => {
                state.emit(crate::Message::OpenSettingsWindow);
                iced::Task::none()
            }
        }
    }
    fn view(&self) -> iced::Element<'_, Self::Message> {
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

        iced::widget::column![
            user_name,
            password,
            user_button,
            self.server_settings.view().map(Message::ServerSettings),
            iced::widget::button("Open Settings Window").on_press(Message::OpenSettingsWindow)
        ]
        .into()
    }
}

impl StartPage {
    pub fn new() -> Self {
        Self {
            user_login_state: false,
            user_name: String::new(),
            password: String::new(),
            is_login_page: true,
            server_settings: crate::widget::ServerSettings::new(),
        }
    }
    pub fn check_login_state(&self) -> Task<Message> {
        iced::Task::none()
    }
    pub fn is_login_page_str(&self) -> &str {
        if self.is_login_page {
            "Create"
        } else {
            "Login"
        }
    }
}
