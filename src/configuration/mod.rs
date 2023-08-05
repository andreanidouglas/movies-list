use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
use config::ConfigError;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Configuration {
    pub themoviedb: TheMovieDb
}

#[derive(Deserialize)]
pub struct TheMovieDb {
    pub api_secret: String
}

pub fn get_configuration() -> Result<Configuration, ConfigError> {
    let base_path = std::env::current_dir().expect("Cannot get current dir");
    let configuration_dir = base_path.join("configuration");

    let configuration = config::Config::builder()
        .add_source(config::File::from(
            configuration_dir.join("production.yaml")
        ))
        .build()?;

    configuration.try_deserialize::<Configuration>()
}
}
}
