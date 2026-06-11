use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub is_ssl: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            is_ssl: false,
        }
    }
}

impl ServerConfig {
    pub fn url(&self) -> String {
        if self.is_ssl {
            format!("https://{}:{}", self.host, self.port)
        } else {
            format!("http://{}:{}", self.host, self.port)
        }
    }
    pub async fn health(
        &self,
    ) -> Result<crate::APIResponse<core_domain::result::HealthResult>, reqwest::Error> {
        let client = reqwest::Client::new();
        let url = format!("{}/api/health", self.url());
        let response = client.get(&url).send().await?;
        let body: crate::APIResponse<core_domain::result::HealthResult> = response.json().await?;
        Ok(body)
    }
}
