use crate::IntoMessage;

#[derive(Clone, Debug)]
pub enum Message {
    Open,
}

impl IntoMessage for Message {
    fn into_message(&self, id: iced::window::Id) -> crate::Message {
        crate::Message::SettingsWindowMessage(id, self.clone())
    }
}

pub struct SettingsWindow {
    pub id: iced::window::Id,
}

impl SettingsWindow {
    pub fn new(id: iced::window::Id) -> Self {
        Self { id }
    }
}

impl crate::Window for SettingsWindow {
    type Message = Message;

    fn settings(_: &crate::Settings) -> iced::window::Settings {
        iced::window::Settings {
            size: iced::Size::new(800.0, 600.0),
            position: iced::window::Position::Centered,
            exit_on_close_request: false,
            ..Default::default()
        }
    }
    fn update(
        &mut self,
        message: Self::Message,
        settings: &mut crate::Settings,
    ) -> iced::Task<crate::Message> {
        match message {
            Message::Open => println!("Open"),
        }
        iced::Task::none()
    }
    fn view(&self) -> iced::Element<'_, Self::Message> {
        iced::widget::column![].into()
    }
    fn close_request(&self) -> bool {
        true
    }
    fn close(&mut self) -> iced::Task<crate::Message> {
        iced::Task::none()
    }
    fn title(&self) -> String {
        "Settings".to_string()
    }
    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}
