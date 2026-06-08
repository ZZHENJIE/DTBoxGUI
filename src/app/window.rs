use iced::{Element, Task};

use crate::IntoMessage;

pub enum WindowHandle {
    MainWindow(crate::MainWindow),
    SettingsWindow(crate::SettingsWindow),
}

pub trait Window {
    type Message: IntoMessage;

    fn close_request(&self) -> bool;
    fn close(&mut self) -> Task<crate::Message>;
    fn update(
        &mut self,
        message: Self::Message,
        window_config: &mut WindowConfig,
    ) -> Task<crate::Message>;
    fn view(&self) -> Element<'_, Self::Message>;
    fn title(&self) -> String;
    fn settings(window_config: &WindowConfig) -> iced::window::Settings;
    fn theme(&self) -> iced::Theme;
}

use serde::{Deserialize, Serialize};

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

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            main_window: WindowItem {
                pos_x: None,
                pos_y: None,
                width: 1200.0,
                height: 900.0,
            },
            chart_windows: Vec::new(),
        }
    }
}

impl WindowConfig {
    pub fn file_path(dir: &directories::ProjectDirs) -> std::path::PathBuf {
        dir.config_dir().join("window_config.json")
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
