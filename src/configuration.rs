#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseConfig,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseConfig {
    pub db_name: String,
    pub password: String,
    pub user_name: String,
    pub host: String,
    pub port: u16,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        // Add configuration values from a file named `configuration.yaml`.
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;

    settings.try_deserialize::<Settings>()
}
