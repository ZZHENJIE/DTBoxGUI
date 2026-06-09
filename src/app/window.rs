use iced::{Element, Task};

use crate::IntoMessage;

pub enum WindowHandle {
    MainWindow(crate::MainWindow),
    SettingsWindow(crate::SettingsWindow),
}

pub trait Window {
    type Message: IntoMessage;

    fn close_request(&self) -> bool;
    fn close(&mut self) -> Task<crate::Message>;
    fn update(&mut self, message: Self::Message, state: &mut crate::State) -> Task<crate::Message>;
    fn view(&self) -> Element<'_, Self::Message>;
    fn title(&self) -> String;
    fn settings(config: &crate::app::config::WindowConfig) -> iced::window::Settings;
    fn theme(&self) -> iced::Theme;
}
