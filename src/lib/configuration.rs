use config::{File, FileFormat};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::{
    postgres::{PgConnectOptions, PgSslMode},
    ConnectOptions,
};

#[derive(Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    // pub fn connection_string(&self) -> Secret<String> {
    //     Secret::new(format!(
    //         "postgres://{}:{}@{}:{}/{}",
    //         self.username,
    //         self.password.expose_secret(),
    //         self.host,
    //         self.port,
    //         self.database_name
    //     ))
    // }
    //
    // pub fn connection_string_without_db(&self) -> Secret<String> {
    //     Secret::new(format!(
    //         "postgres://{}:{}@{}:{}",
    //         self.username,
    //         self.password.expose_secret(),
    //         self.host,
    //         self.port
    //     ))
    // }
    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db()
            .database(&self.database_name)
            .log_statements(tracing_log::log::LevelFilter::Trace)
    }

    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine current directory.");
    let config_dir = base_path.join("configuration");
    let env: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    let mut builder = config::Config::builder()
        .add_source(
            File::from(config_dir.join("base"))
                .format(FileFormat::Yaml)
                .required(true),
        )
        .add_source(
            File::from(config_dir.join(env.as_str()))
                .format(FileFormat::Yaml)
                .required(true),
        );
    // .add_source(config::Environment::with_prefix("APP").separator("__")) // doesn't work

    for (key, val) in std::env::vars() {
        if key.starts_with("APP") {
            let key = key[4..key.len()]
                .to_lowercase()
                .split("__")
                .collect::<Vec<&str>>()
                .join(".");

            builder = builder.clone().set_override(key, val)?;
        }
    }

    builder.build_cloned()?.try_deserialize()
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not supported environment, Use either `local` or `production`.",
                other
            )),
        }
    }
}
