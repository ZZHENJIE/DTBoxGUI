pub struct TokenStore {
    entry: keyring::Entry,
}

impl TokenStore {
    pub fn new(service: &str, user: &str) -> Result<Self, keyring::Error> {
        Ok(Self {
            entry: keyring::Entry::new(service, user)?,
        })
    }

    pub fn save(&self, token: &str) -> Result<(), keyring::Error> {
        self.entry.set_password(token)
    }

    pub fn load(&self) -> Result<String, keyring::Error> {
        self.entry.get_password()
    }

    pub fn clear(&self) -> Result<(), keyring::Error> {
        self.entry.delete_credential()
    }
}
