use crate::error::ApplicationError;

#[derive(serde::Deserialize)]
pub struct Configuration {
    database_host: String,
    database_port: String,
    database_name: String,
    database_user: String,
    database_password: String,
    pub application_port: String,
}

impl Configuration {
    pub fn load() -> Result<Self, ApplicationError> {
        let config = envy::from_env::<Configuration>()?;

        Ok(config)
    }
    pub fn get_postgres_url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.database_user,
            self.database_password,
            self.database_host,
            self.database_port,
            self.database_name
        )
    }
}
