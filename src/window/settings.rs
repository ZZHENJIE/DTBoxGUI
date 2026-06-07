use crate::IntoMessage;

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

impl IntoMessage for SettingsWindow {
    type Message = Message;

    fn into_message(self, message: Self::Message) -> crate::Message {
        crate::Message::SettingsWindowMessage(self.id.clone(), message)
    }
}

impl crate::Window for SettingsWindow {
    fn settings(settings: &crate::Settings) -> iced::window::Settings {
        let mut window_settings = iced::window::Settings::default();
        window_settings.exit_on_close_request = true;
        window_settings
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
        println!("Close");
        iced::Task::none()
    }
    fn title(&self) -> String {
        "Settings".to_string()
    }
    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}
