#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub postgres: PostgresConfig,
    pub server: ServerConfig,
}

#[derive(Debug, serde::Deserialize)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub database_name: String,
    pub username: String,
    pub password: String,
    pub max_pool_connections: u32,
}

#[derive(Debug, serde::Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

impl Default for PostgresConfig {
    fn default() -> Self {
        PostgresConfig {
            max_pool_connections: 5,
            ..Default::default()
        }
    }
}

impl PostgresConfig {
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
