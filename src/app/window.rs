pub enum WindowHandle {
    MainWindow(crate::window::MainWindow),
    SettingsWindow(crate::window::SettingsWindow),
}

impl WindowHandle {
    pub fn theme(&self) -> iced::Theme {
        match self {
            Self::MainWindow(window) => window.theme(),
            Self::SettingsWindow(window) => window.theme(),
        }
    }

    pub fn title(&self) -> String {
        match self {
            Self::MainWindow(window) => window.title(),
            Self::SettingsWindow(window) => window.title(),
        }
    }

    pub fn close_request(&self) -> bool {
        match self {
            Self::MainWindow(window) => window.close_request(),
            Self::SettingsWindow(window) => window.close_request(),
        }
    }
}

pub trait Window: crate::Widget {
    fn close_request(&self) -> bool;
    fn close(&mut self) -> iced::Task<Self::Message>;
    fn title(&self) -> String;
    fn theme(&self) -> iced::Theme;
    fn settings(config: &crate::domain::config::WindowConfig) -> iced::window::Settings;
}
