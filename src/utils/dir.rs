pub fn project_dir() -> anyhow::Result<directories::ProjectDirs> {
    if let Some(project_dir) = directories::ProjectDirs::from("com", "zzhenjie", "DTBox") {
        let config_dir = project_dir.config_dir();
        if !config_dir.exists() {
            std::fs::create_dir_all(config_dir)?;
        }
        Ok(project_dir)
    } else {
        Err(anyhow::anyhow!("Failed to get project directory"))
    }
}
