#[derive(Clone, Debug)]
pub enum Message {
    Open,
}

pub struct SettingsWindow {
    pub id: iced::window::Id,
}

impl SettingsWindow {
    pub fn new(id: iced::window::Id) -> Self {
        Self { id }
    }
}

impl crate::Widget for SettingsWindow {
    type Message = Message;

    fn update(&mut self, _: Self::Message, _: &mut crate::State) -> iced::Task<Message> {
        iced::Task::none()
    }
    fn view(&self) -> iced::Element<'_, Self::Message> {
        iced::widget::column![].into()
    }
}

impl crate::Window for SettingsWindow {
    fn settings(_: &crate::domain::config::WindowConfig) -> iced::window::Settings {
        iced::window::Settings {
            size: iced::Size::new(800.0, 600.0),
            position: iced::window::Position::Centered,
            exit_on_close_request: false,
            ..Default::default()
        }
    }

    fn close_request(&self) -> bool {
        true
    }
    fn close(&mut self) -> iced::Task<Message> {
        iced::Task::none()
    }
    fn title(&self) -> String {
        "Settings".to_string()
    }
    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}
