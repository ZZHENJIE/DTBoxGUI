pub struct State {
    pub project_dir: directories::ProjectDirs,
    pub config: crate::Config,
    pub token_store: crate::TokenStore,
    pub user_settings: Option<core_domain::UserSettings>,
    pub pending_messages: Vec<crate::Message>,
}

impl State {
    pub fn new(
        project_dir: directories::ProjectDirs,
        config: crate::Config,
    ) -> anyhow::Result<Self> {
        let token_store = crate::TokenStore::new("dtbox", "zzhenjie")?;
        Ok(Self {
            project_dir,
            config,
            token_store,
            user_settings: None,
            pending_messages: Vec::new(),
        })
    }

    pub fn emit(&mut self, message: crate::Message) {
        self.pending_messages.push(message);
    }

    pub fn config_save(&self) -> anyhow::Result<()> {
        self.config.save(&self.project_dir)
    }
}
