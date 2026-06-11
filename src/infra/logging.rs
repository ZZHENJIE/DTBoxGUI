use std::sync::OnceLock;
use tracing_appender::non_blocking;
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};

static GUARD: OnceLock<non_blocking::WorkerGuard> = OnceLock::new();

pub fn setup(path: std::path::PathBuf, config: &crate::domain::config::LoggingConfig) {
    let iced_wgpu_level = format!("iced_wgpu={}", config.iced_wgpu_level);
    let iced_winit_level = format!("iced_winit={}", config.iced_winit_level);

    let log_filter = EnvFilter::new(config.level.clone())
        .add_directive(iced_wgpu_level.parse().unwrap_or_default())
        .add_directive(iced_winit_level.parse().unwrap_or_default());

    let file_appender = tracing_appender::rolling::daily(path, "dtbox_gui.log");

    let (file_writer, guard) = non_blocking(file_appender);

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(file_writer)
        .with_ansi(false)
        .with_filter(log_filter);

    tracing_subscriber::registry().with(file_layer).init();

    GUARD.set(guard).expect("logging already initialized");
}
