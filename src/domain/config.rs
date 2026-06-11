use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub window: WindowConfig,
    pub server: Option<crate::ServerConfig>,
    pub logging: LoggingConfig,
}

impl Default for Config {
    fn default() -> Self {
        let window = WindowConfig {
            main_window: WindowItem {
                pos_x: None,
                pos_y: None,
                width: 1200.0,
                height: 900.0,
            },
            chart_windows: Vec::new(),
        };
        Self {
            window,
            server: None,
            logging: LoggingConfig::default(),
        }
    }
}

impl Config {
    pub fn file_path(dir: &directories::ProjectDirs) -> std::path::PathBuf {
        dir.config_dir().join("config.json")
    }
    pub fn load(dir: &directories::ProjectDirs) -> anyhow::Result<Self> {
        let bytes = std::fs::read(Self::file_path(dir))?;
        let settings: Self = serde_json::from_slice(&bytes)?;
        Ok(settings)
    }
    pub fn save(&self, dir: &directories::ProjectDirs) -> anyhow::Result<()> {
        let bytes = serde_json::to_vec(&self)?;
        std::fs::write(Self::file_path(dir), bytes)?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowConfig {
    pub main_window: WindowItem,
    pub chart_windows: Vec<ChartWindow>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowItem {
    pub pos_x: Option<f32>,
    pub pos_y: Option<f32>,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChartWindow {
    pub symbol: String,
    pub window: WindowItem,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub iced_wgpu_level: String,
    pub iced_winit_level: String,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            iced_wgpu_level: "error".to_string(),
            iced_winit_level: "error".to_string(),
        }
    }
}
