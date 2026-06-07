use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub windows: Windows,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Windows {
    pub main_window: Window,
    pub chart_windows: Vec<ChartWindow>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Window {
    pub pos_x: f64,
    pub pos_y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChartWindow {
    pub symbol: String,
    pub window: Window,
}

impl Default for Settings {
    fn default() -> Self {
        let windows = Windows {
            main_window: Window {
                pos_x: 100.0,
                pos_y: 100.0,
                width: 800.0,
                height: 600.0,
            },
            chart_windows: Vec::new(),
        };
        Self { windows }
    }
}

impl Settings {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let bytes = std::fs::read(path)?;
        let settings: Self = serde_json::from_slice(&bytes)?;
        Ok(settings)
    }
}
