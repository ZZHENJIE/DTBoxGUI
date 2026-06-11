pub mod app {
    pub mod application;
    pub mod macros;
    pub mod message;
    pub mod state;
    pub mod widget;
    pub mod window;
}

pub mod domain {
    pub mod api;
    pub mod config;
    pub mod server;
}

pub mod infra {
    pub mod dir;
    pub mod logging;
    pub mod token_store;
}

pub mod window {
    pub mod main;
    pub mod settings;
}

pub mod view;

pub mod widget;

pub use {
    app::application::Application, app::message::Message, app::state::State, app::widget::Widget,
    app::window::Window, domain::api::APIResponse, domain::config::Config,
    domain::server::ServerConfig, infra::token_store::TokenStore,
};
pub use {
    window::main::MainWindow, window::main::Message as MainWindowMessage,
    window::settings::Message as SettingsWindowMessage, window::settings::SettingsWindow,
};
