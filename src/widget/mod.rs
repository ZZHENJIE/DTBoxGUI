pub mod settings {
    pub mod server;
}

pub mod app {
    pub mod menu;
}

pub mod example {
    pub mod plotters;
}

pub use {settings::server::Message as ServerSettingsMessage, settings::server::ServerSettings};
