pub mod app {
    pub mod config;
    pub mod message;
    pub mod state;
    pub mod window;
}

pub mod window {
    pub mod main;
    pub mod settings;
}

pub mod utils {
    pub mod dir;
}

pub mod view;

pub mod widget;

pub use {
    app::config::Config, app::message::IntoMessage, app::message::Message, app::state::State,
    app::window::Window,
};
pub use {
    window::main::MainWindow, window::main::Message as MainWindowMessage,
    window::settings::Message as SettingsWindowMessage, window::settings::SettingsWindow,
};
