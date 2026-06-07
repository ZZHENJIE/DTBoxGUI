use std::fmt::Debug;

use iced::{Element, Task};

pub enum WindowHandle {
    MainWindow(crate::MainWindow),
    SettingsWindow(crate::SettingsWindow),
}

pub trait IntoMessage {
    type Message: Clone + Debug;
    fn into_message(self, message: Self::Message) -> crate::Message;
}

pub trait Window: IntoMessage {
    fn close_request(&self) -> bool;
    fn close(&mut self) -> Task<crate::Message>;
    fn update(
        &mut self,
        message: Self::Message,
        settings: &mut crate::Settings,
    ) -> Task<crate::Message>;
    fn view(&self) -> Element<'_, Self::Message>;
    fn title(&self) -> String;
    fn settings(settings: &crate::Settings) -> iced::window::Settings;
    fn theme(&self) -> iced::Theme;
}
