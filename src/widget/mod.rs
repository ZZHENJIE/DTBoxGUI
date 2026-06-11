pub mod settings {
    pub mod server;
}

pub mod app {
    pub mod menu;
}

pub use {settings::server::Message as ServerSettingsMessage, settings::server::ServerSettings};
