use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub driver: String,  // "mysql" or "mongodb"
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub access_token_expires: i64,   // seconds
    pub refresh_token_expires: i64,  // seconds
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let run_mode = std::env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let settings = config::Config::builder()
            .add_source(config::File::with_name("config/default"))
            .add_source(config::File::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(config::Environment::with_prefix("APP").separator("__"))
            .build()?;

        settings.try_deserialize()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 3000,
            },
            database: DatabaseConfig {
                driver: "mysql".to_string(),
                url: "mysql://root:password@localhost/money_notes".to_string(),
                max_connections: 10,
            },
            jwt: JwtConfig {
                secret: "your-secret-key".to_string(),
                access_token_expires: 3600,
                refresh_token_expires: 604800,
            },
        }
    }
}
