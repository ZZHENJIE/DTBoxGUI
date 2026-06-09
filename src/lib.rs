pub mod app {
    pub mod application;
    pub mod config;
    pub mod message;
    pub mod server;
    pub mod state;
    pub mod token_store;
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
    app::application::Application, app::config::Config, app::message::IntoMessage,
    app::message::Message, app::server::ServerConfig, app::state::State,
    app::token_store::TokenStore, app::window::Window,
};
pub use {
    window::main::MainWindow, window::main::Message as MainWindowMessage,
    window::settings::Message as SettingsWindowMessage, window::settings::SettingsWindow,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct APIResponse<T: serde::Serialize> {
    pub code: i32,
    pub data: Option<T>,
    pub message: String,
}
