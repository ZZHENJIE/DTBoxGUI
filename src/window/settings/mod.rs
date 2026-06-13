pub mod index;

pub mod tabs {
    pub mod logging;
    pub mod server;
}

pub use {index::Message as SettingsWindowMessage, index::SettingsWindow};
