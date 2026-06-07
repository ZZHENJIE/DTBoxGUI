pub mod app {
    pub mod message;
    pub mod settings;
    pub mod state;
    pub mod window;
}

pub mod window {
    pub mod main;
    pub mod settings;
}

pub mod widget {}

pub use {
    app::message::IntoMessage, app::message::Message, app::settings::Settings, app::state::State,
    app::window::Window,
};
pub use {
    window::main::MainWindow, window::main::Message as MainWindowMessage,
    window::settings::Message as SettingsWindowMessage, window::settings::SettingsWindow,
};
