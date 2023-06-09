#[derive(Debug, serde::Deserialize)]
pub struct Settings {
    pub application: AppSettings
}

#[derive(Debug, serde::Deserialize)]
pub struct AppSettings {
    pub host: String,
    pub port: String
}

pub enum Env {
    Local,
    Prod
}
 
pub fn get_config() -> Result<Settings, config::ConfigError> {

    let base_path = std::env::current_dir().expect("Unable to get current directory");
    let config_dir = base_path.join("configuration");

    let env: Env = std::env::var("APP_ENV")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENV");

    let env_file = format!("{}.yaml", env.as_str());
    let settings = config::Config::builder()
        .add_source(config::File::from(
            config_dir.join("base.yaml"),
        ))
        .add_source(config::File::from(
            config_dir.join(env_file)
        ))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__")
        ).build()?;
    
    settings.try_deserialize::<Settings>()

}

impl Env {
    pub fn as_str(&self) -> &'static str {

        match self {
            Env::Prod => "prod",
            Env::Local => "local"
        }
    }
}

impl TryFrom<String> for Env {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "prod" => Ok(Self::Prod),
            other => Err(format!(
                "{} is not a supported env. Use either 'local' or 'production'", 
                other
            ))
        }
    }
}
