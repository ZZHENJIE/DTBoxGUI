use std::fmt::Debug;

#[derive(Clone, Debug)]
pub enum Message {
    OpenMainWindow(iced::window::Id),
    OpenSettingsWindow,
    CloseRequestWindow(iced::window::Id),
    CloseWindow(iced::window::Id),
    MainWindowMessage(iced::window::Id, crate::MainWindowMessage),
    SettingsWindowMessage(iced::window::Id, crate::SettingsWindowMessage),
}
