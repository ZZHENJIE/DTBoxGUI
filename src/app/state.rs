pub struct State {
    pub project_dir: directories::ProjectDirs,
    pub config: crate::Config,
    pub user_settings: Option<core_domain::UserSettings>,
    pub token_store: crate::TokenStore,
}

impl State {
    pub fn new() -> anyhow::Result<Self> {
        let project_dir = crate::utils::dir::project_dir()?;
        let config = crate::Config::load(&project_dir)?;
        let token_store = crate::TokenStore::new("dtbox", "zzhenjie")?;
        Ok(Self {
            project_dir,
            config,
            token_store,
            user_settings: None,
        })
    }
    pub fn config_save(&self) -> anyhow::Result<()> {
        self.config.save(&self.project_dir)
    }
}
