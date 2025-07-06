#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub database: DatabaseSetting,
    pub application_port: u16,
}

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseSetting {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configurations() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;

    settings.try_deserialize::<Settings>()
}

impl DatabaseSetting {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}
