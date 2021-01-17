#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
}

#[derive(Debug, serde::Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub database_name: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password,
            self.host,
            self.port,
            self.database_name
        )
    }
}

pub fn get_config() -> Result<Config, config::ConfigError> {
    let mut config = config::Config::default();
    config.merge(config::File::with_name("config"))?;
    config.try_into()
}
